#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
async fn blink_led(mut led: Output<'static>, interval_ms: u64) {
    loop {
        led.toggle();
        Timer::after(Duration::from_hz(interval_ms)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let yellow_led = Output::new(peripherals.PIN_2, Level::High);
    let red_led = Output::new(peripherals.PIN_3, Level::High);
    let green_led = Output::new(peripherals.PIN_4, Level::High);
    let blue_led = Output::new(peripherals.PIN_5, Level::Low);

    spawner.spawn(blink_led(yellow_led, 3)).unwrap(); // 3 Hz -> 1000ms / 3
    spawner.spawn(blink_led(red_led, 4)).unwrap();    // 4 Hz -> 1000ms / 4
    spawner.spawn(blink_led(green_led, 5)).unwrap();  // 5 Hz -> 1000ms / 5
    spawner.spawn(blink_led(blue_led, 1)).unwrap();  // 1 Hz -> 1000ms / 1
}
