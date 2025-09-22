use crate::{tiles::tile_kind::TileKind, action::Action, chunk_list::ChunkCoord};
use rand::prelude::*;

pub struct Chunk {
    pub tiles: Vec<TileKind>, // row-major order
    dirty: bool,
    incoming: Vec<IncomingTile>,
    pub width: usize,
    pub height: usize,
    pub x: i32,
    pub y: i32,
}

struct IncomingTile {
    location: (usize, usize), // (x, y) in chunk coordinates
    tile: TileKind,
}

impl Chunk {
    pub fn new(width: usize, height: usize, x: i32, y: i32, blank: bool) -> Self {
        let dirty = true;
        let incoming = Vec::new();

        let mut tiles = Vec::with_capacity(width * height);

        let mut rng = rand::rng();

        if !blank {
            for _ in 0..(width * height) {
                tiles.push(if rng.random_bool(0.2) {
                    TileKind::GameOfLife
                } else {
                    TileKind::Empty
                });
            }
        } else {
            for _ in 0..(width * height) {
                tiles.push(TileKind::Empty);
            }
        }
        Self {
            tiles,
            width,
            height,
            x,
            y,
            dirty,
            incoming,
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn update(
        &self,
        coord: ChunkCoord,
        chunk_neighbors: &[&Chunk],
    ) -> (std::vec::Vec<TileKind>, std::vec::Vec<ChunkCoord>) {
        let mut dirty_neighbors = Vec::new();
        let mut new_tiles = Vec::with_capacity(self.width * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let tile_kind = self.tiles[idx];

                // run rules to get an action
                for rule in tile_kind.rules() {
                    let action = rule(x, y, self, chunk_neighbors);

                    let next_tile = match action {
                        Action::Replace(_, new_kind) => {
                            dirty_neighbors.push(coord);
                            new_kind
                        }
                        Action::Destroy(_) => {
                            dirty_neighbors.push(coord);
                            TileKind::Empty
                        }
                        Action::None => tile_kind.clone(),
                    };

                    new_tiles.push(next_tile);

                    // mark neighbor chunks dirty if needed
                    if !matches!(action, Action::None) {
                        if x == 0 {
                            dirty_neighbors.push((coord.0 - 1, coord.1));
                        }
                        if x == self.width - 1 {
                            dirty_neighbors.push((coord.0 + 1, coord.1));
                        }
                        if y == 0 {
                            dirty_neighbors.push((coord.0, coord.1 - 1));
                        }
                        if y == self.height - 1 {
                            dirty_neighbors.push((coord.0, coord.1 + 1));
                        }
                    }
                }
            }
        }

        (new_tiles, dirty_neighbors)
    }
}
