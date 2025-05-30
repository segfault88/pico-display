#![no_std]
#![no_main]

mod pio_programs;

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use panic_halt as _;

// Board Support Package for Raspberry Pi Pico
use rp_pico as bsp;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    pio::PIOExt,
    sio::Sio,
    watchdog::Watchdog,
};

use pio_programs::{Rgb, rainbow};

// Configuration for the LED strip
const NUM_LEDS: usize = 8; // Change this to match your LED strip length
const LED_PIN: u8 = 2;     // GPIO pin connected to the LED strip data line

#[entry]
fn main() -> ! {
    info!("🌈 WS2812 NeoPixel Controller Starting!");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the Pico board is 12MHz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Setup onboard LED for status indication
    let mut led_pin = pins.led.into_push_pull_output();

    // === WS2812 PIO Setup ===
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    
    // Install WS2812 program
    let ws2812_program = pio_programs::ws2812();
    let installed = pio.install(&ws2812_program).unwrap();
    
    // Configure state machine for WS2812 timing
    // WS2812 requires specific timing: ~800kHz for proper bit timing
    let (mut sm, _, mut tx) = bsp::hal::pio::PIOBuilder::from_program(installed)
        .side_set_pin_base(LED_PIN)
        .out_shift_direction(bsp::hal::pio::ShiftDirection::Left)
        .autopull(true)
        .pull_threshold(24) // Pull every 24 bits (one RGB pixel)
        .clock_divisor_fixed_point(3, 0) // ~125MHz / 3 = ~41.7MHz, gives proper WS2812 timing
        .build(sm0);

    // Configure the LED data pin for PIO output
    let _led_data_pin = pins.gpio2.into_mode::<bsp::hal::gpio::FunctionPio0>();
    
    // Start the PIO state machine
    let _sm = sm.start();

    info!("✅ WS2812 PIO program running on GPIO{}", LED_PIN);
    info!("🎨 Controlling {} NeoPixel LEDs", NUM_LEDS);
    info!("📍 Status LED on GPIO25 (onboard)");

    // Animation variables
    let mut animation_step = 0u8;
    let mut color_offset = 0u8;

    // LED strip buffer
    let mut strip = [Rgb::BLACK; NUM_LEDS];

    info!("🚀 Starting rainbow animation...");

    loop {
        // Status LED heartbeat
        led_pin.set_high().unwrap();
        delay.delay_ms(50);
        led_pin.set_low().unwrap();

        // Generate rainbow pattern
        for (i, led) in strip.iter_mut().enumerate() {
            let pos = color_offset.wrapping_add((i * 255 / NUM_LEDS) as u8);
            *led = rainbow(pos);
        }

        // Send colors to LED strip
        for led in &strip {
            let grb_data = led.to_grb24();
            
            // Send 24-bit color data to PIO (will block if FIFO is full)
            while !tx.write(grb_data) {
                cortex_m::asm::nop();
            }
        }

        // Update animation
        animation_step = animation_step.wrapping_add(1);
        if animation_step % 4 == 0 {
            color_offset = color_offset.wrapping_add(8);
        }

        // Different patterns based on animation step
        match (animation_step / 50) % 4 {
            0 => {
                // Rainbow wave
                info!("🌈 Rainbow wave pattern");
            }
            1 => {
                // All same color, cycling through spectrum
                let color = rainbow(color_offset);
                strip.iter_mut().for_each(|led| *led = color);
                info!("🎯 Solid color: R:{} G:{} B:{}", color.r, color.g, color.b);
            }
            2 => {
                // Color chase
                let chase_pos = (animation_step / 5) % NUM_LEDS as u8;
                strip.iter_mut().for_each(|led| *led = Rgb::BLACK);
                if let Some(led) = strip.get_mut(chase_pos as usize) {
                    *led = rainbow(color_offset);
                }
                info!("🏃 Color chase at position {}", chase_pos);
            }
            3 => {
                // Sparkle effect
                strip.iter_mut().for_each(|led| *led = Rgb::BLACK);
                let sparkle_pos = (animation_step * 17) % NUM_LEDS as u8; // Pseudo-random
                if let Some(led) = strip.get_mut(sparkle_pos as usize) {
                    *led = Rgb::WHITE;
                }
                info!("✨ Sparkle at position {}", sparkle_pos);
            }
            _ => {}
        }

        // Send updated colors to strip
        for led in &strip {
            let grb_data = led.to_grb24();
            while !tx.write(grb_data) {
                cortex_m::asm::nop();
            }
        }

        delay.delay_ms(100);
    }
}
