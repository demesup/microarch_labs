#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Input, Pull};
use embassy_time::{Duration, Timer};
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;

#[derive(Clone, Copy)]
enum LedCommand {
    Increase,
    Decrease,
}

static CHANNEL: Channel<ThreadModeRawMutex, LedCommand, 64> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_25, Level::Low);
    let mut button_increase = Input::new(p.PIN_15, Pull::Up);
    let mut button_decrease = Input::new(p.PIN_14, Pull::Up);

    spawner.spawn(main_task(led)).unwrap();
    spawner.spawn(button_increase_task(button_increase)).unwrap();
    spawner.spawn(button_decrease_task(button_decrease)).unwrap();
}

#[embassy_executor::task]
async fn main_task(mut led: Output<'static>) {
    let mut intensity: u8 = 0;

    loop {
        let command = CHANNEL.receive().await;
        match command {
            LedCommand::Increase => {
                if intensity < 255 {
                    intensity += 1;
                }
            }
            LedCommand::Decrease => {
                if intensity > 0 {
                    intensity -= 1;
                }
            }
        }

        led.set_high();
        Timer::after(Duration::from_micros(intensity as u64 * 10)).await;
        led.set_low();
        Timer::after(Duration::from_micros((255 - intensity) as u64 * 10)).await;
    }
}

#[embassy_executor::task]
async fn button_increase_task(mut button: Input<'static>) {
    loop {
        button.wait_for_low().await;
        CHANNEL.send(LedCommand::Increase).await;
        Timer::after(Duration::from_millis(50)).await;
        button.wait_for_high().await;
    }
}

#[embassy_executor::task]
async fn button_decrease_task(mut button: Input<'static>) {
    loop {
        button.wait_for_low().await;
        CHANNEL.send(LedCommand::Decrease).await;
        Timer::after(Duration::from_millis(50)).await;
        button.wait_for_high().await;
    }
}
