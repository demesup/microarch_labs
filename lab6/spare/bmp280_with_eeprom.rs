#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::binary_info::consts::ID_RP_PROGRAM_BUILD_DATE_STRING;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pull};
use embassy_rp::i2c::{Config as I2cConfig, I2c, InterruptHandler as I2CInterruptHandler};
use embassy_rp::peripherals::I2C0;
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::{Error, I2c as _};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => I2CInterruptHandler<I2C0>;
});

const BMP280_ADDR: u16 = 0x76;
const EEPROM_ADDR: u16 = 0x50;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let sda = peripherals.PIN_16;
    let scl = peripherals.PIN_17;
    let mut i2c = I2c::new_async(peripherals.I2C0, scl, sda, Irqs, I2cConfig::default());

    let ctrl_message = 0b010_000_11;
    i2c.write(BMP280_ADDR, &[0xF4, ctrl_message]).await.unwrap();
    let mut buf = [0u8; 3];
    let mut calibration_data = [0u8; 6];
    i2c.write_read(BMP280_ADDR, &[0x88], &mut calibration_data)
        .await
        .unwrap();
    let dig_t1: u16 = ((calibration_data[1] as u16) << 8) | (calibration_data[0] as u16);
    let dig_t2: i16 = ((calibration_data[3] as i16) << 8) | (calibration_data[2] as i16);
    let dig_t3: i16 = ((calibration_data[5] as i16) << 8) | (calibration_data[4] as i16);

    let log_addr = 0xACDC as u16;

    let addr_bytes = log_addr.to_be_bytes();
    let mut data = [0u8; 4];
    i2c.write_read(EEPROM_ADDR, &addr_bytes, &mut data)
        .await
        .unwrap();
    let stored_temp = i32::from_be_bytes(data);
    info!(
        "Last temp: {}.{:02}°C",
        stored_temp / 100,
        stored_temp.abs() % 100
    );

    loop {
        i2c.write_read(BMP280_ADDR, &[0xFA], &mut buf)
            .await
            .unwrap();
        let temp_msb = buf[0] as i32;
        let temp_lsb = buf[1] as i32;
        let temp_xlsb = buf[2] as i32;
        let raw_temp: i32 = (temp_msb << 12) + (temp_lsb << 4) + (temp_xlsb >> 4);
        info!("Raw temperature: {}", raw_temp);
        let var1 = (((raw_temp >> 3) - ((dig_t1 as i32) << 1)) * (dig_t2 as i32)) >> 11;
        let var2 = (((((raw_temp >> 4) - (dig_t1 as i32)) * ((raw_temp >> 4) - (dig_t1 as i32)))
            >> 12)
            * (dig_t3 as i32))
            >> 14;
        let t_fine = var1 + var2;
        let actual_temp = (t_fine * 5 + 128) >> 8;
        info!(
            "Temperature {}.{}°C",
            actual_temp / 100,
            actual_temp.abs() % 100
        );
        let addr_bytes = log_addr.to_be_bytes();
        let temp_bytes = actual_temp.to_be_bytes();
        let mut tx_buf = [0u8; 6];
        tx_buf[0..2].copy_from_slice(&addr_bytes);
        tx_buf[2..6].copy_from_slice(&temp_bytes);
        i2c.write(EEPROM_ADDR, &tx_buf).await.unwrap();
        Timer::after(Duration::from_millis(1000)).await;
    }
}