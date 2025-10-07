use crate::{action::Action, chunk_list::ChunkCoord, tiles::tile_kind::TileKind};
use rand::prelude::*;

pub struct Chunk {
    pub tiles: Vec<TileKind>, // row-major order
    dirty: bool,
    pub incoming: Vec<IncomingTile>,
    pub width: usize,
    pub height: usize,
    pub x: i32,
    pub y: i32,
}

pub struct IncomingTile {
    pub location: (usize, usize), // (x, y) in chunk coordinates
    pub tile: TileKind,
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

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn update(
        &self,
        chunk_neighbors: &[&Chunk],
    ) -> std::vec::Vec<Action> {
        let mut actions = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let tile_kind = self.tiles[idx];

                // run rules to get an action
                for rule in tile_kind.rules() {
                    let action = rule(x, y, self, chunk_neighbors);
                    if !matches!(action, Action::None) {
                        actions.push(action.clone());
                        break; // only one action per tile per update
                    }
                }
            }
        }

        actions
    }

    pub fn apply_incoming(&mut self) {
        for incoming in self.incoming.drain(..) {
            let (x, y) = incoming.location;
            if x < self.width && y < self.height {
                let idx = y * self.width + x;
                self.tiles[idx] = incoming.tile;
            }
        }
    }
}
