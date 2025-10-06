
use crate::{chunk::Chunk, colors::Colors, tiles::game_of_life::GameOfLife, tiles::empty::Empty, action::Action, tiles::powder::Powder};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileKind {
    GameOfLife,
    Empty,
    Sand,
}

impl TileKind {
    pub fn to_colors(&self) -> Colors {
        match self {
            TileKind::GameOfLife => GameOfLife::COLOR,
            TileKind::Empty => Empty::COLOR,
            TileKind::Sand => Powder::COLOR,
        }
    }

    pub fn rules(
        &self,
    ) -> &'static [fn(usize, usize, &Chunk, &[&Chunk]) -> Action] {
        match self {
            TileKind::GameOfLife => &[GameOfLife::death_rule],
            TileKind::Empty => &[GameOfLife::birth_rule],
            TileKind::Sand => &[Powder::fall_down_rule],
        }
    }
}

