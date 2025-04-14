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
    const REG_CALIBRATION_DATA: u8 = 0x88;

    
    let setup = [REG_CTRL_MEAS, 0b00100011]; 
    i2c.write(BMP280_ADDR, &setup).await.unwrap();
    info!("BMP280 configured");

    
    let mut data = [0u8; 6];
    i2c.write_read(BMP280_ADDR, &[REG_CALIBRATION_DATA], &mut data).await.unwrap();
    
    
    let dig_t1: u16 = ((data[1] as u16) << 8) | (data[0] as u16);
    let dig_t2: i16 = ((data[3] as i16) << 8) | (data[2] as i16);
    let dig_t3: i16 = ((data[5] as i16) << 8) | (data[4] as i16);

    info!("Calibration values: dig_t1 = {}, dig_t2 = {}, dig_t3 = {}", dig_t1, dig_t2, dig_t3);

    loop {
        let mut buf = [0u8; 3];

        
        i2c.write_read(BMP280_ADDR, &[REG_TEMP_MSB], &mut buf).await.unwrap();

        
        let raw_temp: u32 = ((buf[0] as u32) << 12) | ((buf[1] as u32) << 4) | ((buf[2] as u32) >> 4);
        info!("Raw temperature: {}", raw_temp);

        
        let var1 = ((((raw_temp >> 3) as i32 - ((dig_t1 as i32) << 1))) * (dig_t2 as i32)) >> 11;
        let var2 = (((((raw_temp >> 4) as i32 - (dig_t1 as i32)) * ((raw_temp >> 4) as i32 - (dig_t1 as i32))) >> 12)
            * (dig_t3 as i32)) >> 14;
        let t_fine = var1 + var2;
        let actual_temp = (t_fine * 5 + 128) >> 8; 

        
        info!(
            "Temperature {}.{}°C",
            actual_temp / 100,
            actual_temp.abs() % 100
        );

        
        Timer::after(Duration::from_secs(1)).await;
    }
}
