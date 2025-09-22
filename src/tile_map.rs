use crate::colors::Colors;

pub struct TileMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Colors>, // RGBA values
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Colors::Green; width * height];
        Self { width, height, tiles: pixels }
    }

    pub fn draw_checkerboard(&mut self) {
        let square_size = 1; // size of each square in tiles
        for y in 0..self.height {
            for x in 0..self.width {

                let square_row = y / square_size;
                let square_col = x / square_size;
                let half_rows = (self.height / square_size) / 2;
                let half_cols = (self.width / square_size) / 2;

                if square_row < half_rows && square_col < half_cols {
                    if (square_row + square_col) % 2 == 0 {
                        self.tiles[y * self.width + x] = Colors::Red;
                    } else {
                        self.tiles[y * self.width + x] = Colors::Green;
                    }
                } else if square_row < half_rows && square_col >= half_cols {
                    if (square_row + square_col) % 2 == 0 {
                        self.tiles[y * self.width + x] = Colors::Blue;
                    } else {
                        self.tiles[y * self.width + x] = Colors::Yellow;
                    }
                } else if square_row >= half_rows && square_col < half_cols {
                    if (square_row + square_col) % 2 == 0 {
                        self.tiles[y * self.width + x] = Colors::Yellow;
                    } else {
                        self.tiles[y * self.width + x] = Colors::Green;
                    }
                } else if square_row >= half_rows && square_col >= half_cols {
                    if (square_row + square_col) % 2 == 0 {
                        self.tiles[y * self.width + x] = Colors::Blue;
                    } else {
                        self.tiles[y * self.width + x] = Colors::Red;
                    }
                }
                   
            }
        }
    }

    pub fn update(&mut self) {
        for tile in &mut self.tiles {
            if tile == &Colors::Red {
                *tile = Colors::Green;
            } else if tile == &Colors::Green {
                *tile = Colors::Blue;
            } else if tile == &Colors::Blue {
                *tile = Colors::Yellow;
            } else if tile == &Colors::Yellow {
                *tile = Colors::Red;
            }
        }
    }

}