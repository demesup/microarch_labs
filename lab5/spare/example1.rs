#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{self, Spi};
use embassy_time::Timer;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use cortex_m_rt::entry;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());
    
    // SPI configuration
    let mut config = spi::Config::default();
    config.frequency = 1_000_000;
    config.phase = spi::Phase::CaptureOnFirstTransition;
    config.polarity = spi::Polarity::IdleLow;

    // Define SPI pins
    let miso = p.PIN_0; // Replace with actual pin number
    let mosi = p.PIN_19; // Replace with actual pin number
    let clk = p.PIN_18;  // Replace with actual pin number

    // Create SPI instance
    let mut spi = Spi::new(p.SPI0, clk, mosi, miso, p.DMA_CH0, p.DMA_CH1, config);
    
    // Chip Select (CS) pin setup
    let mut cs = Output::new(p.PIN_4, Level::High); // Replace with actual pin number

    // Activate the sub by setting CS low
    cs.set_low();
    
    // Define TX and RX buffers
    let tx_buf = [1_u8, 2, 3, 4, 5, 6];
    let mut rx_buf = [0_u8; 6]; // Buffer to store received data

    // SPI data transfer
    spi.transfer(&mut rx_buf, &tx_buf).await;
    
    // Deactivate the sub by setting CS high
    cs.set_high();
    
    info!("SPI transfer complete. Received data: {:?}", rx_buf);
}
