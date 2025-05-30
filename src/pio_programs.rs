// PIO Programs for WS2812 NeoPixel control

use pio_proc::pio_asm;
use pio::Program;

/// WS2812 (NeoPixel) driver program
/// Reads 24-bit RGB data from FIFO and outputs WS2812 protocol
/// 
/// This program generates the precise timing required for WS2812 LEDs:
/// - 0 bit: 400ns high, 850ns low
/// - 1 bit: 800ns high, 450ns low
/// - Reset: >50μs low
pub fn ws2812() -> Program<32> {
    pio_asm!(
        ".side_set 1 opt",
        ".wrap_target",
        "bitloop:",
        "    out x, 1       side 0 [2]", // Side-set still takes place when instruction stalls
        "    jmp !x do_zero side 1 [1]", // Branch on the bit we shifted out. Positive pulse
        "do_one:",
        "    jmp  bitloop   side 1 [4]", // Continue driving high, for a long pulse
        "do_zero:",
        "    nop            side 0 [4]", // Or drive low, for a short pulse
        ".wrap",
    ).program
}

/// RGB color structure for easy color handling
#[derive(Copy, Clone, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Convert RGB to the 24-bit format expected by WS2812
    /// WS2812 expects GRB format (Green-Red-Blue)
    pub fn to_grb24(&self) -> u32 {
        ((self.g as u32) << 16) | ((self.r as u32) << 8) | (self.b as u32)
    }

    /// Predefined colors
    pub const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
    pub const RED: Rgb = Rgb { r: 255, g: 0, b: 0 };
    pub const GREEN: Rgb = Rgb { r: 0, g: 255, b: 0 };
    pub const BLUE: Rgb = Rgb { r: 0, g: 0, b: 255 };
    pub const WHITE: Rgb = Rgb { r: 255, g: 255, b: 255 };
    pub const YELLOW: Rgb = Rgb { r: 255, g: 255, b: 0 };
    pub const CYAN: Rgb = Rgb { r: 0, g: 255, b: 255 };
    pub const MAGENTA: Rgb = Rgb { r: 255, g: 0, b: 255 };
}

/// Generate a rainbow color based on position (0-255)
pub fn rainbow(pos: u8) -> Rgb {
    match pos {
        0..=84 => Rgb::new(255 - pos * 3, pos * 3, 0),
        85..=169 => {
            let pos = pos - 85;
            Rgb::new(0, 255 - pos * 3, pos * 3)
        }
        170..=255 => {
            let pos = pos - 170;
            Rgb::new(pos * 3, 0, 255 - pos * 3)
        }
    }
} 