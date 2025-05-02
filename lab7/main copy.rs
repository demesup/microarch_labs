// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app prints a "Hello world" message with defmt.

#![no_std]
#![no_main]

use cyw43::JoinOptions;
use embassy_executor::Spawner;
use embassy_net::StackResources;
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// Use the logging macros provided by defmt.
use defmt::*;

// Import interrupts definition module
mod irqs;

const SOCK: usize = 4;
const WIFI_NETWORK: &str = "Galaxy A51 444E";
const WIFI_PASSWORD: &str = "hxcp9146";
static RESOURCES: StaticCell<StackResources<SOCK>> = StaticCell::<StackResources<SOCK>>::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let (net_device, mut _control) = embassy_lab_utils::init_wifi!(&spawner, peripherals).await;

    let config = embassy_net::Config::dhcpv4(Default::default());

    let _stack = embassy_lab_utils::init_network_stack(&spawner, net_device, &RESOURCES, config);

    info!("Hello world!");

    loop {
        match _control
            .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
            }
        }
    }

    // Wait for DHCP, not necessary when using static IP
    info!("waiting for DHCP...");
    while !_stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up!");
    info!("Assigned IP: {}", _stack.config_v4().unwrap().address);

    let delay = Duration::from_secs(1);
    loop {
        Timer::after(delay).await;
    }
}