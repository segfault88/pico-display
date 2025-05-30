# Pico Display - WS2812 NeoPixel Controller

This project is a **WS2812 NeoPixel controller** for the Raspberry Pi Pico (RP2040 chip) using Rust and **PIO (Programmable I/O)** for precise timing control.

## Features

🌈 **Multiple Animation Patterns**: Rainbow waves, color chase, sparkle effects, and solid colors  
⚡ **Hardware-accelerated**: Uses RP2040's PIO for perfect WS2812 timing  
🎨 **Easy Color Control**: RGB color utilities and rainbow generation  
📡 **Real-time Control**: Fast updates with no CPU overhead for signal generation  

## Prerequisites

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install the ARM target**:
   ```bash
   rustup target add thumbv6m-none-eabi
   ```

3. **Install required tools**:
   ```bash
   cargo install flip-link
   cargo install probe-rs --features cli
   cargo install elf2uf2-rs
   ```

## Hardware Setup

### Required Components
- Raspberry Pi Pico
- WS2812B LED strip (NeoPixels)
- 5V power supply (for LED strip)
- 470Ω resistor (recommended for data line)
- Jumper wires

### Wiring Diagram
```
Pico            WS2812 Strip
GPIO2    ──────── Data In (through 470Ω resistor)
GND      ──────── GND
5V/VBUS  ──────── +5V (for low LED counts)
```

**Important Notes:**
- Use external 5V power supply for strips with >10 LEDs
- The 470Ω resistor on the data line helps prevent signal issues
- Connect power supply ground to Pico ground
- GPIO2 is configurable in the code (`LED_PIN` constant)

## Configuration

Edit `src/main.rs` to customize your setup:

```rust
const NUM_LEDS: usize = 8; // Change to your LED strip length
const LED_PIN: u8 = 2;     // GPIO pin connected to LED data line
```

**Important**: If you change `LED_PIN`, you must also update the pin configuration line:
```rust
// Change this line to match your LED_PIN
let _led_data_pin = pins.gpio2.into_mode::<bsp::hal::gpio::FunctionPio0>();
```

For example, to use GPIO3:
```rust
const LED_PIN: u8 = 3;
// ...
let _led_data_pin = pins.gpio3.into_mode::<bsp::hal::gpio::FunctionPio0>();
```

## Building and Flashing

### Build the Project
```bash
cargo build --release
```

### Convert to UF2 Format
```bash
elf2uf2-rs target/thumbv6m-none-eabi/release/pico-display pico-display.uf2
```

### Flash to Pico
1. Hold BOOTSEL button while connecting Pico to computer
2. Copy `pico-display.uf2` to the Pico drive
3. Pico will reboot and start the LED animations

## What This Program Does

### 🎭 Animation Patterns
The program cycles through 4 different patterns every ~5 seconds:

1. **🌈 Rainbow Wave**: Flowing rainbow colors across the strip
2. **🎯 Solid Color**: All LEDs the same color, cycling through spectrum  
3. **🏃 Color Chase**: Single LED chasing down the strip
4. **✨ Sparkle**: Random white sparkle effects

### 🔧 Technical Features
- **Hardware PIO timing**: Perfect WS2812 protocol timing (800kHz)
- **24-bit color depth**: Full RGB color control per LED
- **Non-blocking operation**: Animations run independently of main CPU
- **Status LED**: Onboard LED provides heartbeat indication

## Project Structure

- `src/main.rs` - Main NeoPixel controller with animations
- `src/pio_programs.rs` - WS2812 PIO program and RGB utilities
- `Cargo.toml` - Dependencies including PIO support
- `memory.x` - Memory layout for RP2040
- `.cargo/config.toml` - Build configuration

## Advanced Usage

### Custom Colors
```rust
use pio_programs::Rgb;

let red = Rgb::new(255, 0, 0);
let custom = Rgb::new(128, 64, 200);
let color_data = custom.to_grb24(); // Convert for WS2812
```

### Different GPIO Pins
To use a different GPIO pin, update both the constant and pin configuration:
```rust
const LED_PIN: u8 = 4; // Use GPIO4 instead

// And update the pin configuration line:
let _led_data_pin = pins.gpio4.into_mode::<bsp::hal::gpio::FunctionPio0>();
```

### Larger LED Strips
For strips with many LEDs, consider:
- Increasing delay between updates
- Using external power supply
- Adding current limiting

### Custom Animations
Add your own patterns in the main loop:
```rust
// Custom pattern example
4 => {
    // Breathing effect
    let brightness = (animation_step as f32 * 0.1).sin().abs();
    let color = Rgb::new((255.0 * brightness) as u8, 0, 0);
    strip.iter_mut().for_each(|led| *led = color);
}
```

## Troubleshooting

### LED Issues
- **No LEDs lighting**: Check wiring, power supply, and GPIO pin configuration
- **Wrong colors**: Verify WS2812 vs WS2811 (different color orders)
- **Flickering**: Add decoupling capacitors, check power supply stability
- **First LED wrong**: Ensure proper reset timing (>50μs gap)

### Build Issues
- **Target not found**: Install `thumbv6m-none-eabi` target
- **Linker errors**: Install `flip-link` and ensure it's in PATH
- **PIO errors**: Check PIO assembly syntax in `pio_programs.rs`

### Performance
- **Slow updates**: Decrease delay or optimize animation code
- **Memory issues**: Reduce `NUM_LEDS` or optimize data structures

## WS2812 Protocol Details

The PIO program generates precise timing for WS2812 LEDs:
- **0 bit**: 400ns high, 850ns low (±150ns tolerance)
- **1 bit**: 800ns high, 450ns low (±150ns tolerance)  
- **Reset**: >50μs low period between frames
- **Data format**: 24-bit GRB (Green-Red-Blue) per LED

## Resources

- [WS2812B Datasheet](https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf)
- [RP2040 PIO Documentation](https://datasheets.raspberrypi.org/rp2040/rp2040-datasheet.pdf)
- [Adafruit NeoPixel Guide](https://learn.adafruit.com/adafruit-neopixel-uberguide)
- [rp-pico Documentation](https://docs.rs/rp-pico) 