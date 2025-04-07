#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _; // Global logger
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::pwm::{Config as ConfigPwm, Pwm};
use embassy_time::{Duration, Timer};
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use panic_probe as _;

#[derive(Clone, Copy)]
enum LedCommand {
    Increase,
    Decrease,
}

static CHANNEL: Channel<ThreadModeRawMutex, LedCommand, 64> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // PWM Configuration
    let mut config: ConfigPwm = Default::default();
    config.top = 0x9088; // 37000 in decimal
    config.compare_a = config.top / 10; // Start at 10% brightness

    // Simple LED on Pin 2
    let mut pwm = Pwm::new_output_a(peripherals.PWM_SLICE1, peripherals.PIN_2, config.clone());

    // Buttons
    let button_increase = Input::new(peripherals.PIN_15, Pull::Up);
    let button_decrease = Input::new(peripherals.PIN_14, Pull::Up);

    // Spawn tasks
    spawner.spawn(main_task(pwm)).unwrap();
    spawner.spawn(button_increase_task(button_increase)).unwrap();
    spawner.spawn(button_decrease_task(button_decrease)).unwrap();
}

#[embassy_executor::task]
async fn main_task(mut pwm: Pwm<'static>) {
    let mut config: ConfigPwm = pwm.config();
    let mut brightness: u8 = 5; // Start at 50% brightness

    loop {
        let command = CHANNEL.receive().await;
        match command {
            LedCommand::Increase => {
                if brightness < 10 {
                    brightness += 1;
                }
            }
            LedCommand::Decrease => {
                if brightness > 0 {
                    brightness -= 1;
                }
            }
        }

        config.compare_a = (config.top as u32 * brightness as u32 / 10) as u16;
        pwm.set_config(&config);
        info!("Brightness: {}%", brightness * 10);
    }
}

#[embassy_executor::task]
async fn button_increase_task(mut button: Input<'static>) {
    loop {
        button.wait_for_low().await;
        info!("Button Increase pressed");
        CHANNEL.send(LedCommand::Increase).await;
        Timer::after(Duration::from_millis(50)).await; // Debounce
        button.wait_for_high().await;
    }
}

#[embassy_executor::task]
async fn button_decrease_task(mut button: Input<'static>) {
    loop {
        button.wait_for_low().await;
        info!("Button Decrease pressed");
        CHANNEL.send(LedCommand::Decrease).await;
        Timer::after(Duration::from_millis(50)).await; // Debounce
        button.wait_for_high().await;
    }
}
