#![no_std]
#![no_main]

// we use as _ to avoid a compiler warning
// saying that the crate is not used
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _; // global logger
use embassy_executor::Spawner;
use embassy_rp::pwm::{Config as ConfigPwm, Pwm};
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    info!("Hello, world!");
    let mut config: ConfigPwm = Default::default();

    config.top = 0x9088; // in HEX, equals 37000 in decimal
    config.compare_a = config.top / 4;
    let mut pwm = Pwm::new_output_a(
        // Output B
        peripherals.PWM_SLICE1, // Channel 1
        peripherals.PIN_2,      // Pin 2
        config.clone(),
    );
    let mut brightness = 0;
    loop {
        config.compare_a = (config.top as u32 - ((config.top as u32 * brightness as u32) / 10)) as u16;
        pwm.set_config(&config);
        brightness = (brightness + 1) % 10;
        Timer::after(Duration::from_millis(1000)).await;
    }
}
