use crate::{
    action::Action, chunk::Chunk, colors::Colors, tile_checks::{below_tile, below_right_tile, below_left_tile},
    tiles::tile_kind::TileKind,
};

pub struct Powder;

impl Powder {
    pub const COLOR: Colors = Colors::Yellow;

    pub fn fall_down_rule(x: usize, y: usize, chunk: &Chunk, neighbors: &[&Chunk]) -> Action {
        let idx = y * chunk.width + x;
        let this_tile = chunk.tiles[idx];
        if let Some((below_idx, cross_chunk, destination_chunk  )) = below_tile(x, y, chunk, neighbors) {
            if cross_chunk {
                let below_tile = neighbors
                    .iter()
                    .find(|c| (c.x, c.y) == destination_chunk)
                    .unwrap()
                    .tiles[below_idx];
                if below_tile == TileKind::Empty {
                    return Action::SwapCrossChunk(idx, destination_chunk, below_idx, this_tile);
                }
            } else {
                let below_tile = chunk.tiles[below_idx];
                if below_tile == TileKind::Empty {
                    return Action::Swap(idx, below_idx);
                }
            }
        }
        Action::None
    }

    pub fn fall_diagonal_rule(x: usize, y: usize, chunk: &Chunk, neighbors: &[&Chunk]) -> Action {
        let fall_right = rand::random();
        if fall_right {
            Self::fall_right_rule(x, y, chunk, neighbors)
        } else {
            Self::fall_left_rule(x, y, chunk, neighbors)
        }
    }

    fn fall_right_rule(x: usize, y: usize, chunk: &Chunk, neighbors: &[&Chunk]) -> Action {
        let idx = y * chunk.width + x;
        let this_tile = chunk.tiles[idx];
        if let Some((below_idx, cross_chunk, destination_chunk)) = below_right_tile(x, y, chunk, neighbors) {
            if cross_chunk {
                let below_tile = neighbors
                    .iter()
                    .find(|c| (c.x, c.y) == destination_chunk)
                    .unwrap()
                    .tiles[below_idx];
                if below_tile == TileKind::Empty {
                    return Action::SwapCrossChunk(idx, destination_chunk, below_idx, this_tile);
                }
            } else {
                let below_tile = chunk.tiles[below_idx];
                if below_tile == TileKind::Empty {
                    return Action::Swap(idx, below_idx);
                }
            }
        }
        Action::None
    }

    fn fall_left_rule(x: usize, y: usize, chunk: &Chunk, neighbors: &[&Chunk]) -> Action {
        let idx = y * chunk.width + x;
        let this_tile = chunk.tiles[idx];
        if let Some((below_idx, cross_chunk, destination_chunk)) = below_left_tile(x, y, chunk, neighbors) {
            if cross_chunk {
                let below_tile = neighbors
                    .iter()
                    .find(|c| (c.x, c.y) == destination_chunk)
                    .unwrap()
                    .tiles[below_idx];
                if below_tile == TileKind::Empty {
                    return Action::SwapCrossChunk(idx, destination_chunk, below_idx, this_tile);
                }
            } else {
                let below_tile = chunk.tiles[below_idx];
                if below_tile == TileKind::Empty {
                    return Action::Swap(idx, below_idx);
                }
            }
        }
        Action::None
    }
}
