#![no_std]
#![no_main]

use defmt::{info, warn};
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::init;
use embassy_rp::gpio::{Level, Output, Input, Pull};
use embassy_time::Timer;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Default::default());
    let mut led = Output::new(peripherals.PIN_4, Level::Low);
    let mut button = Input::new(peripherals.PIN_5, Pull::Up);

    info!("Waiting for button press on SW5");
    
    loop {
        button.wait_for_falling_edge().await;
        info!("The button was pressed");
        led.set_high();
        Timer::after_millis(100).await;
        led.set_low();
    }
}