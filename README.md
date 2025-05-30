# Pico Display - Raspberry Pi Pico Rust Project

This project is configured to run on the **Raspberry Pi Pico** (RP2040 chip) using Rust and the embedded HAL.

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

## Building the Project

To build the project for the Raspberry Pi Pico:

```bash
cargo build --release
```

This will create a binary in `target/thumbv6m-none-eabi/release/pico-display`.

## Converting to UF2 Format

To run the code on your Pico, you need to convert the ELF binary to UF2 format:

```bash
elf2uf2-rs target/thumbv6m-none-eabi/release/pico-display pico-display.uf2
```

## Flashing to the Pico

### Method 1: USB Mass Storage (Bootsel Mode)

1. Hold the BOOTSEL button while connecting the Pico to your computer
2. The Pico will appear as a USB mass storage device
3. Copy the `pico-display.uf2` file to the Pico
4. The Pico will automatically reboot and run your program

### Method 2: Using probe-rs (if you have a debug probe)

If you have a debug probe connected, uncomment the runner line in `.cargo/config.toml` and use:

```bash
cargo run --release
```

## What This Program Does

The current program demonstrates basic Pico functionality by:
- Initializing the RP2040 microcontroller
- Setting up clocks and GPIO
- Blinking the onboard LED every 500ms
- Outputting debug messages via defmt/RTT

## Project Structure

- `src/main.rs` - Main application code
- `Cargo.toml` - Dependencies and project configuration
- `memory.x` - Memory layout for the RP2040
- `.cargo/config.toml` - Build configuration for cross-compilation
- `build.rs` - Build script for linker setup

## Adding More Functionality

To extend this project, you can:
- Add more GPIO pins for sensors or actuators
- Implement I2C/SPI communication
- Add display drivers
- Implement timing and interrupts
- Add USB serial communication

Refer to the [rp-pico documentation](https://docs.rs/rp-pico) and [rp2040-hal documentation](https://docs.rs/rp2040-hal) for more examples and APIs.

## Troubleshooting

- **Build errors**: Make sure you have the `thumbv6m-none-eabi` target installed
- **Linker errors**: Ensure `flip-link` is installed and accessible in your PATH
- **USB connection issues**: Try different USB cables and ports
- **Debug output**: Use `probe-rs` or RTT viewer to see debug messages from `defmt` 