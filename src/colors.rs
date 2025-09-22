#[derive(PartialEq, Eq,Clone, Copy, Debug)]
pub enum Colors {
    Black = 0,
    Green = 1,
    Red = 2,
    Blue = 3,
    Yellow = 4,
    White = 5,
    // add more if needed
}

impl Colors {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

// Precomputed RGBA table
pub const COLORS_RGBA: [[u8; 4]; 6] = [
    [0, 0, 0, 255],      // Black
    [0, 255, 0, 255],    // Green
    [255, 0, 0, 255],    // Red
    [0, 0, 255, 255],    // Blue
    [255, 255, 0, 255],  // Yellow
    [255, 255, 255, 255],// White
];