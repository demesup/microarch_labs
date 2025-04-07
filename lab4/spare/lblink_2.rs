#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Instant;

#[embassy_executor::task]
async fn blink_red_led(mut led: Output<'static>) {
    loop {
        led.toggle();
        let start_time = Instant::now();
        while start_time.elapsed().as_millis() < 1000 {}
    }
}

#[embassy_executor::task]
async fn blink_blue_led(mut led: Output<'static>) {
    loop {
        led.toggle();
        let start_time = Instant::now();
        while start_time.elapsed().as_millis() < 1000 {}
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    
    let red_led = Output::new(peripherals.PIN_4, Level::Low);
    let blue_led = Output::new(peripherals.PIN_5, Level::Low);
    
    spawner.spawn(blink_red_led(red_led)).unwrap();
    spawner.spawn(blink_blue_led(blue_led)).unwrap();
}