use crate::{chunk::Chunk, colors::Colors, action::Action, tiles::tile_kind::TileKind};

pub struct Powder;

impl Powder {
    pub const COLOR: Colors = Colors::Yellow;

    pub fn fall_down_rule(x: usize, y: usize, chunk: &Chunk, neighbors: &[&Chunk]) -> Action {
        let below_y = y + 1;
        if below_y < chunk.height {
            let below_idx = below_y * chunk.width + x;
            if chunk.tiles[below_idx] == TileKind::Empty {
                let idx = y * chunk.width + x;
                return Action::Swap(idx, below_idx);
            }
        } else {
            // Check neighbor chunk below
            let neighbor_coord = (chunk.x, chunk.y + 1);
            if let Some(neighbor_chunk) = neighbors.iter().find(|c| (c.x, c.y) == neighbor_coord).copied() {
                if x < neighbor_chunk.width && neighbor_chunk.tiles[x] == TileKind::Empty {
                    let idx = y * chunk.width + x;
                    let below_idx = x; // top row of neighbor chunk
                    return Action::SwapCrossChunk(idx, neighbor_coord, below_idx, chunk.tiles[idx]);
                }
            }
        }
        Action::None
    }

}