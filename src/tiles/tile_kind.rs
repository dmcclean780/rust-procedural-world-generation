
use crate::{chunk::Chunk, colors::Colors, tiles::game_of_life::GameOfLife, tiles::empty::Empty, action::Action};

#[derive(Clone, Copy)]
pub enum TileKind {
    GameOfLife,
    Empty
}

impl TileKind {
    pub fn to_colors(&self) -> Colors {
        match self {
            TileKind::GameOfLife => GameOfLife::COLOR,
            TileKind::Empty => Empty::COLOR,}
    }

    pub fn rules(
        &self,
    ) -> &'static [fn(usize, usize, &Chunk, &[&Chunk]) -> Action] {
        match self {
            TileKind::GameOfLife => &[GameOfLife::death_rule],
            TileKind::Empty => &[GameOfLife::birth_rule],
        }
    }
}

