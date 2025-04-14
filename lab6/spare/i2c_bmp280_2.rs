#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};

use embassy_rp::i2c::{I2c, InterruptHandler as I2cInterruptHandler, Config as I2cConfig};
use embassy_rp::peripherals::I2C0;
use embassy_rp::bind_interrupts;
use embedded_hal_async::i2c::I2c as _;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => I2cInterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_16;
    let scl = p.PIN_17;
    let mut i2c = I2c::new_async(p.I2C0, scl, sda, Irqs, I2cConfig::default());

    const BMP280_ADDR: u8 = 0x76;
    const REG_CTRL_MEAS: u8 = 0xF4;
    const REG_TEMP_MSB: u8 = 0xFA;

    
    let setup = [REG_CTRL_MEAS, 0b00100011]; 
    i2c.write(BMP280_ADDR, &setup).await.unwrap();
    info!("BMP280 configured");

    loop {
        let mut buf = [0u8; 3];

        
        i2c.write_read(BMP280_ADDR, &[REG_TEMP_MSB], &mut buf).await.unwrap();

        
        let raw_temp: u32 = ((buf[0] as u32) << 12) | ((buf[1] as u32) << 4) | ((buf[2] as u32) >> 4);
        info!("Raw temperature: {}", raw_temp);

        Timer::after(Duration::from_secs(1)).await;
    }
}
