use crate::{chunk::Chunk, colors::Colors, tiles::tile_kind::TileKind, action::Action};

pub struct GameOfLife;

impl GameOfLife {
    pub const COLOR: Colors = Colors::Green;

    fn count_live_neighbors(x: usize, y: usize, chunk: &Chunk, neighbors: &[&Chunk]) -> usize {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // skip self
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                let neighbor_chunk: Option<&Chunk>;
                let mut local_x = nx;
                let mut local_y = ny;

                // Determine if the neighbor tile is inside current chunk
                if nx >= 0 && nx < chunk.width as isize && ny >= 0 && ny < chunk.height as isize {
                    neighbor_chunk = Some(chunk);
                } else {
                    // Map to the correct neighbor chunk
                    let chunk_offset_x = if nx < 0 {
                        -1
                    } else if nx >= chunk.width as isize {
                        1
                    } else {
                        0
                    };
                    let chunk_offset_y = if ny < 0 {
                        -1
                    } else if ny >= chunk.height as isize {
                        1
                    } else {
                        0
                    };

                    // Compute local coordinates in that neighbor chunk
                    local_x = (nx - chunk_offset_x * chunk.width as isize) as isize;
                    local_y = (ny - chunk_offset_y * chunk.height as isize) as isize;

                    // Look up the chunk by offset
                    let neighbor_coord = (
                        chunk.x + chunk_offset_x as i32,
                        chunk.y + chunk_offset_y as i32,
                    );
                    neighbor_chunk = neighbors
                        .iter()
                        .find(|c| (c.x, c.y) == neighbor_coord)
                        .copied();
                }

                // Now safely index into neighbor_chunk
                if let Some(chunk) = neighbor_chunk {
                    let idx = (local_y * chunk.width as isize + local_x) as usize;
                    if chunk.tiles[idx].to_colors() == GameOfLife::COLOR {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn birth_rule(x: usize, y: usize, chunk: &Chunk, neighbors: &[&Chunk]) -> Action {
        let idx = y * chunk.width + x;
        let live_neighbors = GameOfLife::count_live_neighbors(x, y, chunk, neighbors);
        if live_neighbors == 3 {
            return Action::Replace(idx, TileKind::GameOfLife);
        }
        Action::None
    }

    pub fn death_rule(x: usize, y: usize, chunk: &Chunk, neighbors: &[&Chunk]) -> Action {
        let idx = y * chunk.width + x;
        let live_neighbors = GameOfLife::count_live_neighbors(x, y, chunk, neighbors);
        if live_neighbors < 2 || live_neighbors > 3 {
            return Action::Destroy(idx);
        }
        Action::None
    }
}
