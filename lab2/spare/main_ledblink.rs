#![no_std]
#![no_main]

use defmt::{info, warn};
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::init;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Timer, Duration};


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Default::default());
    let mut led = Output::new(peripherals.PIN_4, Level::Low);
    
    info!("Starting LED blink on GP2 every 300ms");
    
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(300)).await;
        led.set_low();
        Timer::after(Duration::from_millis(300)).await;
    }
}