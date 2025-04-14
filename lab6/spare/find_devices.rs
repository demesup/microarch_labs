#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::i2c::{I2c, InterruptHandler as I2CInterruptHandler, Config as I2cConfig};
use embedded_hal_async::i2c::{Error, I2c as _};
use embassy_rp::peripherals::I2C0;
use embassy_rp::bind_interrupts;

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
        I2C0_IRQ => I2CInterruptHandler<I2C0>;
    });
   
 

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let sda = peripherals.PIN_16;
    let scl = peripherals.PIN_17;

    let mut i2c = I2c::new_async(peripherals.I2C0, scl, sda, Irqs, I2cConfig::default());

    let mut rx_buf = [0x00u8; 2];

    defmt::info!("Starting I2C scan...");

    
    for addr in 0x03..=0x77 {
        let addr = addr as u8;
        if i2c.read(addr, &mut rx_buf).await.is_ok() {
        defmt::info!("Found device at address: 0x{:02X}", addr);
    }
    }


    defmt::info!("I2C scan complete.");
}