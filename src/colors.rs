#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Colors {
    Black = 0,
    Green = 1,
    Red = 2,
    Blue = 3,
    Yellow = 4,
    White = 5,
    Sand = 6,
    Stone = 7,
    // add more if needed
}

impl Colors {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    pub fn random_alpha(self, pixel_x: u32, pixel_y: u32) -> u8 {
        // Cheap deterministic hash for per-pixel variation
        let hash = pixel_x.wrapping_mul(374761393)
            ^ pixel_y.wrapping_mul(668265263)
            ^ (self.as_u8() as u32).wrapping_mul(362437);

        // Alpha in 180–255 range
        (180 + (hash % 76) as u8)
    }
}

// Precomputed RGBA table
pub const COLORS_RGBA: [[u8; 4]; 8] = [
    [0, 0, 0, 255],       // Black
    [0, 255, 0, 255],     // Green
    [255, 0, 0, 255],     // Red
    [0, 0, 255, 255],     // Blue
    [255, 255, 0, 255],   // Yellow
    [255, 255, 255, 255], // White
    [194, 178, 128, 255], // Sand
    [128, 128, 128, 255], // Stone
];
