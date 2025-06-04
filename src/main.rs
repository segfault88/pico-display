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
const LED_PIN: u8 = 15;    // GPIO pin connected to the LED strip data line (GPIO15 = physical pin 20)

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
    // WS2812 requires specific timing: The clock should be set for proper bit timing
    // With side-set and delays in the PIO program, we need ~800kHz effective rate
    let (mut sm, _, mut tx) = bsp::hal::pio::PIOBuilder::from_program(installed)
        .side_set_pin_base(LED_PIN)
        .out_shift_direction(bsp::hal::pio::ShiftDirection::Left)
        .autopull(true)
        .pull_threshold(24) // Pull every 24 bits (one RGB pixel)
        .clock_divisor_fixed_point(6, 25) // Slower clock for proper WS2812 timing
        .build(sm0);

    // Configure the LED data pin for PIO output
    let _led_data_pin = pins.gpio15.into_mode::<bsp::hal::gpio::FunctionPio0>();
    
    // Start the PIO state machine
    sm.set_pindirs([(LED_PIN, bsp::hal::pio::PinDir::Output)]);
    let sm = sm.start();

    info!("✅ WS2812 PIO program running on GPIO{}", LED_PIN);
    info!("🎨 Controlling {} NeoPixel LEDs", NUM_LEDS);
    info!("📍 Status LED on GPIO25 (onboard)");

    info!("🚀 Starting LED test...");

    // Simple test: all LEDs red
    let test_color = Rgb::new(255, 0, 0); // Bright red
    let mut strip = [test_color; NUM_LEDS];

    loop {
        // Status LED heartbeat - slower for easier observation
        led_pin.set_high().unwrap();
        delay.delay_ms(100);
        led_pin.set_low().unwrap();
        delay.delay_ms(100);

        info!("Sending red to {} LEDs", NUM_LEDS);

        // Send colors to LED strip
        for led in &strip {
            let grb_data = led.to_grb24();
            info!("Sending GRB data: 0x{:06X}", grb_data);
            
            // Send 24-bit color data to PIO (will block if FIFO is full)
            while !tx.write(grb_data) {
                cortex_m::asm::nop();
            }
        }

        // Important: Add reset delay for WS2812 (>50μs)
        delay.delay_ms(1);

        delay.delay_ms(1000); // Wait 1 second between updates
    }
}
