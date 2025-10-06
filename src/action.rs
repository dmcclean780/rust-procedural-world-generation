use crate::tiles::tile_kind::TileKind;

#[derive(Clone, Debug)]
pub enum Action {
    None,
    Destroy(usize),     // remove target tile
    Replace(usize, TileKind), // replace target with new kind
    Swap(usize, usize), // swap two tiles
    SwapCrossChunk(usize, (i32, i32), usize, TileKind), // swap two tiles across chunks
}