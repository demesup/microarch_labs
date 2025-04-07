#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::spi::{self, Spi};
use embassy_rp::gpio::{Output, Level};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Configure SPI
    let mut config = spi::Config::default();
    let miso = p.PIN_0;
    let mosi = p.PIN_19;
    let clk = p.PIN_18;
    let mut spi = Spi::new(p.SPI0, clk, mosi, miso, p.DMA_CH0, p.DMA_CH1, config);
    let mut cs = Output::new(p.PIN_1, Level::High);

    const WHO_AM_I: u8 = 0x75;
    cs.set_low();
    let tx_buf = [(1 << 7) | WHO_AM_I, 0x00];
    let mut rx_buf = [0u8; 2];
    spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
    cs.set_high();
    let who_am_i_value = rx_buf[1];
    info!("WHO_AM_I register value: {=u8}", who_am_i_value);
}
