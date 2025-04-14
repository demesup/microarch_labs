#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;

use embassy_rp::i2c::{I2c, InterruptHandler as I2CInterruptHandler, Config as I2cConfig};
use embedded_hal_async::i2c::I2c as _;
use embassy_rp::peripherals::I2C0;
use embassy_rp::bind_interrupts;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => I2CInterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let sda = peripherals.PIN_0; 
    let scl = peripherals.PIN_1; 

    let mut i2c = I2c::new_async(peripherals.I2C0, scl, sda, Irqs, I2cConfig::default());

    const TARGET_ADDR: u16 = 0x44;

    let mut rx_buf = [0x00u8; 2];
    i2c.read(TARGET_ADDR, &mut rx_buf).await.unwrap();
    info!("Read data: {:?}", rx_buf);

    let tx_buf = [0x01, 0x05];
    i2c.write(TARGET_ADDR, &tx_buf).await.unwrap();
    info!("Write done");

    i2c.write_read(TARGET_ADDR, &tx_buf, &mut rx_buf).await.unwrap();
    info!("Write-Read data: {:?}", rx_buf);
}
