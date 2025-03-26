#![no_std]
#![no_main]

use defmt::{info};
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::init;
use embassy_rp::gpio::{Level, Output};


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Default::default());
    let mut _led = Output::new(peripherals.PIN_2, Level::High);
    
    info!("LED on GP2 is set to HIGH");
}