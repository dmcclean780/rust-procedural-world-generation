use crate::{
    action::Action,
    chunk::{Chunk},
    tiles::tile_kind::TileKind,
};
use rayon::prelude::*;
use std::collections::HashMap;

pub type ChunkCoord = (i32, i32);

pub struct ChunkList {
    pub alive_chunks: HashMap<ChunkCoord, Chunk>,
    dead_chunks: HashMap<ChunkCoord, Chunk>,
    pub chunk_width: usize,
    pub chunk_height: usize,
}

impl ChunkList {
    pub fn new(
        chunk_width: usize,
        chunk_height: usize,
        chunk_x_num: i32,
        chunk_y_num: i32,
    ) -> Self {
        let mut chunks = HashMap::new();

        for y in 0..chunk_y_num {
            for x in 0..chunk_x_num {
                let coord = (x, y);
                chunks.insert(coord, Chunk::new(chunk_width, chunk_height, x, y, true));
            }
        }

        Self {
            alive_chunks: chunks,
            dead_chunks: HashMap::new(),
            chunk_width,
            chunk_height,
        }
    }

    pub fn update(&mut self) {
        let dirty_coords: Vec<ChunkCoord> = self
            .alive_chunks
            .iter()
            .filter(|(_, chunk)| chunk.is_dirty())
            .map(|(coord, _)| *coord)
            .collect();

        // Ensure neighbors exist for edge chunks
        self.extend_chunks(&dirty_coords);

        let mut next_actions: HashMap<ChunkCoord, Vec<Action>> = HashMap::new();

        // Process chunks in color groups (9-color scheme)
        for color in 0..9 {
            let chunks_of_color: Vec<ChunkCoord> = dirty_coords
                .iter()
                .copied()
                .filter(|&(x, y)| {
                    let rx = x.rem_euclid(3);
                    let ry = y.rem_euclid(3);
                    (rx, ry) == self.color_to_coords(color, 3)
                })
                .collect();

            // Phase 1: parallel compute next states (read-only)
            let updates: Vec<(ChunkCoord, Vec<Action>)> = chunks_of_color
                .par_iter()
                .filter_map(|coord| {
                    let neighbors = self.get_neighbors(*coord);
                    if let Some(chunk) = self.alive_chunks.get(coord) {
                        let new_tiles = chunk.update(&neighbors);
                        Some((*coord, new_tiles))
                    } else {
                        None
                    }
                })
                .collect();

            // Collect Actions and dirty chunks
            for (coord, actions) in updates {
                next_actions.insert(coord, actions);
            }
        }

        let mut cross_swaps = Vec::new();

        // Commit new actions
        for (coord, actions) in next_actions {
            if let Some(chunk) = self.alive_chunks.get_mut(&coord) {
                for action in actions {
                    match action {
                        Action::Replace(idx, new_kind) => {
                            chunk.tiles[idx] = new_kind;
                            chunk.mark_dirty();
                        }
                        Action::Destroy(idx) => {
                            chunk.tiles[idx] = TileKind::Empty;
                            chunk.mark_dirty();
                        }
                        Action::Swap(idx_a, idx_b) => {
                            chunk.tiles.swap(idx_a, idx_b);
                            chunk.mark_dirty();
                        }
                        Action::SwapCrossChunk(idx_a, neighbor_coord, idx_b, tile_kind) => {
                            // push into neighbor (deferred)
                            cross_swaps.push((coord, idx_a, neighbor_coord, idx_b, tile_kind));
                        }
                        Action::None => {
                            chunk.mark_clean();
                        }
                    }
                }
            }
        }

        for (coord, idx_a, neighbor_coord, idx_b, tile_kind) in cross_swaps {
            // careful: fetch chunks separately
            let other_tile =
                if let Some(neighbor_chunk) = self.alive_chunks.get_mut(&neighbor_coord) {
                    let other_tile = neighbor_chunk.tiles[idx_b];
                    neighbor_chunk.tiles[idx_b] = tile_kind;
                    neighbor_chunk.mark_dirty();
                    other_tile
                } else {
                    TileKind::Empty
                };

            if let Some(chunk) = self.alive_chunks.get_mut(&coord) {
                chunk.tiles[idx_a] = other_tile;
                chunk.mark_dirty();
            }
        }
    }

    fn get_neighbors(&self, coord: ChunkCoord) -> Vec<&Chunk> {
        let mut neighbors = Vec::new();
        let (x, y) = coord;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // skip self
                }
                let neighbor_coord = (x + dx, y + dy);
                if let Some(neighbor) = self.alive_chunks.get(&neighbor_coord) {
                    neighbors.push(neighbor);
                }
            }
        }

        neighbors
    }

    pub fn get(&self, coord: &ChunkCoord) -> Option<&Chunk> {
        self.alive_chunks.get(coord)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ChunkCoord, &Chunk)> {
        self.alive_chunks.iter()
    }

    pub fn get_or_create_chunk(&mut self, x: i32, y: i32) -> &mut Chunk {
        let coord = (x, y);

        // Entry API lets us insert if missing and get mutable reference
        self.alive_chunks
            .entry(coord)
            .or_insert_with(|| Chunk::new(self.chunk_width, self.chunk_height, x, y, true))
    }

    pub fn extend_chunks(&mut self, dirty_chunks: &[ChunkCoord]) {
        for coord in dirty_chunks {
            let (x, y) = *coord;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let neighbor_coord = (x + dx, y + dy);
                    self.get_or_create_chunk(neighbor_coord.0, neighbor_coord.1);
                }
            }
        }
    }

    pub fn cull_chunks(
        &mut self,
        viewport_x: isize,
        viewport_y: isize,
        viewport_tile_width: usize,
        viewport_tile_height: usize,
        buffer_chunks: usize,
    ) {
        // Compute the min/max chunk coords we want to keep alive
        let min_chunk_x =
            ((viewport_x / self.chunk_width as isize) - buffer_chunks as isize) as i32;
        let max_chunk_x = ((viewport_x / self.chunk_width as isize) as i32
            + viewport_tile_width as i32 / self.chunk_width as i32
            + buffer_chunks as i32) as i32;

        let min_chunk_y =
            ((viewport_y / self.chunk_height as isize) - buffer_chunks as isize) as i32;
        let max_chunk_y = ((viewport_y / self.chunk_height as isize) as i32
            + viewport_tile_height as i32 / self.chunk_height as i32
            + buffer_chunks as i32) as i32;

        // Iterate over alive chunks and move ones outside the bounds
        let mut to_remove = Vec::new();
        for (&coord, _) in self.alive_chunks.iter() {
            if coord.0 < min_chunk_x
                || coord.0 > max_chunk_x
                || coord.1 < min_chunk_y
                || coord.1 > max_chunk_y
            {
                to_remove.push(coord);
            }
        }

        for coord in to_remove {
            if let Some(chunk) = self.alive_chunks.remove(&coord) {
                self.dead_chunks.insert(coord, chunk);
            }
        }
    }

    pub fn revive_chunks_near_viewport(
        &mut self,
        viewport_x: isize,
        viewport_y: isize,
        viewport_tile_width: usize,
        viewport_tile_height: usize,
        buffer_chunks: usize,
    ) -> Vec<ChunkCoord> {
        let min_chunk_x =
            ((viewport_x / self.chunk_width as isize) - buffer_chunks as isize) as i32;
        let max_chunk_x = ((viewport_x / self.chunk_width as isize) as i32
            + viewport_tile_width as i32 / self.chunk_width as i32
            + buffer_chunks as i32) as i32;

        let min_chunk_y =
            ((viewport_y / self.chunk_height as isize) - buffer_chunks as isize) as i32;
        let max_chunk_y = ((viewport_y / self.chunk_height as isize) as i32
            + viewport_tile_height as i32 / self.chunk_height as i32
            + buffer_chunks as i32) as i32;

        let mut revived = Vec::new();

        // Collect coordinates of dead chunks that need to be revived
        let to_revive: Vec<ChunkCoord> = self
            .dead_chunks
            .iter()
            .filter(|(coord, _)| {
                coord.0 >= min_chunk_x
                    && coord.0 <= max_chunk_x
                    && coord.1 >= min_chunk_y
                    && coord.1 <= max_chunk_y
            })
            .map(|(&coord, _)| coord)
            .collect();

        for coord in to_revive {
            if let Some(chunk) = self.dead_chunks.remove(&coord) {
                self.alive_chunks.insert(coord, chunk);
                revived.push(coord);
            }
        }

        revived
    }

    fn color_to_coords(&self, color: i32, n: i32) -> (i32, i32) {
        let cx = (color % n + n) % n; // or just color.rem_euclid(n)
        let cy = (color / n) % n;
        (cx, cy)
    }
}
