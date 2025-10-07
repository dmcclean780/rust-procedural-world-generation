
use crate::{action::Action, chunk::Chunk, colors::Colors, tiles::{empty::Empty, game_of_life::GameOfLife, powder::Powder, base_elements::{Sand, Stone}}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileKind {
    GameOfLife,
    Empty,
    Sand,
    Stone,
}

impl TileKind {
    pub fn to_colors(&self) -> Colors {
        match self {
            TileKind::GameOfLife => GameOfLife::COLOR,
            TileKind::Empty => Empty::COLOR,
            TileKind::Sand => Sand::COLOR,
            TileKind::Stone => Stone::COLOR,
        }
    }

    pub fn rules(
        &self,
    ) -> &'static [fn(usize, usize, &Chunk, &[&Chunk]) -> Action] {
        match self {
            TileKind::GameOfLife => &[/*GameOfLife::death_rule*/],
            TileKind::Empty => &[/*GameOfLife::birth_rule*/],
            TileKind::Sand => &[Powder::fall_down_rule, Powder::fall_diagonal_rule],
            TileKind::Stone => &[],

        }
    }
}


