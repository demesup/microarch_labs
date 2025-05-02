// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app prints a "Hello world" message with defmt.

#![no_std]
#![no_main]

use cyw43::JoinOptions;
use embassy_executor::Spawner;
use embassy_net::{Ipv4Address, Ipv4Cidr, StackResources, tcp::TcpSocket};
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use heapless::Vec;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// Use the logging macros provided by defmt.
use defmt::*;

// Import interrupts definition module
mod irqs;

const SOCK: usize = 4;
static RESOURCES: StaticCell<StackResources<SOCK>> = StaticCell::<StackResources<SOCK>>::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let (net_device, mut _control) = embassy_lab_utils::init_wifi!(&spawner, peripherals).await;

    let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 14, 1), 24),
        gateway: Some(Ipv4Address::new(192, 168, 14, 1)),
        dns_servers: Vec::new(),
    });

    let _stack = embassy_lab_utils::init_network_stack(&spawner, net_device, &RESOURCES, config);

    info!("Hello world!");

    _control
        .start_ap_wpa2("LAloalaloa", "Hehehehe", 5)
        .await;

    let delay = Duration::from_secs(1);
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut socket = TcpSocket::new(_stack, &mut rx_buffer, &mut tx_buffer);
    // If we want to keep the connection open regardless of inactivity, we can set the timeout
    // to None
    socket.set_timeout(None);

    loop {
        info!("Listening on TCP:6000...");
        if let Err(e) = socket.accept(6000).await {
            warn!("accept error: {:?}", e);
            continue;
        }
        info!("Received connection from {:?}", socket.remote_endpoint());
        let mut buf = [0; 4096];
        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    warn!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    warn!("read error: {:?}", e);
                    break;
                }
            };

            info!("rxd {}", core::str::from_utf8(&buf[..n]).unwrap());

            match socket.write_all(&buf[..n]).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("write error: {:?}", e);
                    break;
                }
            };
        }
    }
}