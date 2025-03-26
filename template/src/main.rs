#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {}