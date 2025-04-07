#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _; // global logger
use embassy_executor::Spawner;
use embassy_futures::select::{self, Either, select};
use embassy_rp::pwm::Config as ConfigPwm;
use embassy_rp::{
    gpio::{AnyPin, Input, Level, Output, Pull},
    pwm::Pwm,
};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, Receiver, Sender},
};
use embassy_time::{Duration, Instant, Timer};
use panic_probe as _;

static CHANNEL: Channel<ThreadModeRawMutex, LedIntensityCommand, 64> = Channel::new();

#[derive(Debug, Clone, Copy)]
enum LedIntensityCommand {
    Increase,
    Decrease,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut config: ConfigPwm = Default::default();
    config.top = 0x9088;
    config.compare_a = config.top;
    let mut pwm_led = Pwm::new_output_a(
        peripherals.PWM_SLICE1, // Channel 1
        peripherals.PIN_2,      // Pin 2
        config.clone(),
    );
    let sw4_pin = AnyPin::from(peripherals.PIN_15);
    let sw5_pin = AnyPin::from(peripherals.PIN_14);
    let mut button_sw4 = Input::new(sw4_pin, Pull::Up);
    let mut button_sw5 = Input::new(sw5_pin, Pull::Up);
    // Initialize brightness
    let mut brightness = 0;
    loop {
        // Wait for either button press
        match select(
            button_sw4.wait_for_falling_edge(),
            button_sw5.wait_for_falling_edge(),
        )
        .await
        {
            Either::First(_) => {
                brightness = (brightness + 1) % 10;
                config.compare_a = config.top.saturating_sub(
                    ((config.top as u32).saturating_mul(brightness as u32) / 10) as u16,
                );

                pwm_led.set_config(&config);
                info!("Increased brightness: {}", brightness);
            }
            Either::Second(_) => {
                brightness = (brightness + 9) % 10;
                config.compare_a = config.top.saturating_sub(
                    ((config.top as u32).saturating_mul(brightness as u32) / 10) as u16,
                );

                pwm_led.set_config(&config);
                info!("Decreased brightness: {}", brightness);
            }
        }
    }

    Timer::after_millis(50).await; // Debounce delay
}