#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};

use embassy_rp::i2c::{I2c, Config as I2cConfig, InterruptHandler};
use embassy_rp::peripherals::I2C0;
use embassy_rp::bind_interrupts;
use embedded_hal_async::i2c::I2c as _;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
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

    
    let config = [REG_CTRL_MEAS, 0b01000011];
    i2c.write(BMP280_ADDR, &config).await.unwrap();
    info!("BMP280 configured");

    loop {
        
        let mut temp_buf = [0u8; 3];
        i2c.write_read(BMP280_ADDR, &[REG_TEMP_MSB], &mut temp_buf).await.unwrap();

        
        let raw_temp: u32 = ((temp_buf[0] as u32) << 12)
                          | ((temp_buf[1] as u32) << 4)
                          | ((temp_buf[2] as u32) >> 4);

        info!("Raw temperature: {}", raw_temp);

        Timer::after(Duration::from_secs(1)).await;
    }
}
