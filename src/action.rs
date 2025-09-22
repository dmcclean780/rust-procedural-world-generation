use crate::tiles::tile_kind::TileKind;

pub enum Action {
    None,
    Destroy(usize),     // remove target tile
    Replace(usize, TileKind), // replace target with new kind
}