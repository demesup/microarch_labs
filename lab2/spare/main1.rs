#![no_std]
#![no_main]

use defmt::{info, warn};
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::init;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _peripherals = init(Default::default());
    info!("Device started");
}
