#![no_std]
#![no_main]

// we use as _ to avoid a compiler warning
// saying that the crate is not used
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _; // global logger
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config as ConfigAdc, InterruptHandler}; // ADC config
use embassy_rp::gpio::{self, Pull};
use embassy_rp::pwm::{Config as PwmConfig, Pwm};
use embassy_rp::usb::In;
use embassy_time::{Duration, Timer};
use panic_probe as _;

use embassy_rp::bind_interrupts;
use fixed::traits::ToFixed;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

enum LightIntensity {
    Low,
    Medium,
    High,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
// Initialize the RP2350 peripherals
    let peripherals = embassy_rp::init(Default::default());    use embassy_rp::pwm::Config as ConfigPwm; 

    // Configure PWM for servo control
    let mut servo_config: PwmConfig = Default::default();

    // Set the calculated TOP value for 50 Hz PWM
    servo_config.top = 0xB71A; 

    // Set the clock divider to 64
    servo_config.divider = 64_i32.to_fixed(); // Clock divider = 64

    // Servo timing constants
    const PERIOD_US: usize = 20_000; // 20 ms period for 50 Hz
    const MIN_PULSE_US: usize = 500; // 0.5 ms pulse for 0 degrees
    const MAX_PULSE_US: usize = 2500; // 2.5 ms pulse for 180 degrees

    // Calculate the PWM compare values for minimum and maximum pulse widths
    let min_pulse = (MIN_PULSE_US * servo_config.top as usize) / PERIOD_US;
    let max_pulse = (MAX_PULSE_US * servo_config.top as usize) / PERIOD_US;

    // Initialize PWM for servo control
    let mut servo = Pwm::new_output_a(
        peripherals.PWM_SLICE1, 
        peripherals.PIN_2, 
        servo_config.clone()
    );
    info!("Hello, world!");

    // Main loop to move the servo back and forth
    loop {
        let mut config = servo_config.clone();
        info!("Moving servo to max position (180 degrees)");
        config.compare_a = max_pulse as u16;
        config.compare_b = 0;
        servo.set_config(&config);

        // Wait for 1 second (servo stays at max position)
        Timer::after(Duration::from_secs(1)).await;

        // Move servo to minimum position (0 degrees)
        info!("Moving servo to minimum position (0 degrees)");
        config.compare_a = min_pulse as u16;
        config.compare_b = 0;
        servo.set_config(&config);

        // Wait for 1 second (servo stays at min position)
        Timer::after(Duration::from_secs(1)).await;
    }
}