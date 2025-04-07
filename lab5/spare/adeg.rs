#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Level, Output},
    spi::{Config as SpiConfig, Phase, Polarity, Spi},
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use defmt::*;
use micromath::F32Ext;

const WHO_AM_I: u8 = 0x75;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    info!("MPU-6000 Initialized!");

    let mut config = SpiConfig::default();
    config.frequency = 1_000_000;
    config.phase = Phase::CaptureOnFirstTransition;
    config.polarity = Polarity::IdleLow;

    let miso = p.PIN_12;
    let mosi = p.PIN_15;
    let clk = p.PIN_14;

    let mut spi = Spi::new(p.SPI1, clk, mosi, miso, p.DMA_CH0, p.DMA_CH1, config);

    let mut cs = Output::new(p.PIN_13, Level::High);

    let delay = Duration::from_secs(1);

    cs.set_low();
    let tx_buf = [!(1 << 7) & 0x1B, 0x08]; 
    let mut rx_buf = [0u8; 2];
    spi.transfer(&mut rx_buf, &tx_buf).await;

    cs.set_low();
    let tx_buf = [!(1 << 7) & 0x1C, 0x00]; 
    let mut rx_buf = [0u8; 2];
    spi.transfer(&mut rx_buf, &tx_buf).await;

    loop {
        // Read Accelerometer Data
        cs.set_low();
        let tx_buf = [(1 << 7) | 0x3B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut rx_buf = [0u8; 7];
        spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
        cs.set_high();

        let accel_x_raw = i16::from_be_bytes([rx_buf[1], rx_buf[2]]);
        let accel_y_raw = i16::from_be_bytes([rx_buf[3], rx_buf[4]]);
        let accel_z_raw = i16::from_be_bytes([rx_buf[5], rx_buf[6]]);

        let accel_x = (accel_x_raw as f32) * 0.000061035 * 9.80665;
        let accel_y = (accel_y_raw as f32) * 0.000061035 * 9.80665;
        let accel_z = (accel_z_raw as f32) * 0.000061035 * 9.80665;

        let accel_x_rounded = (accel_x * 100.0).round() / 100.0;
        let accel_y_rounded = (accel_y * 100.0).round() / 100.0;
        let accel_z_rounded = (accel_z * 100.0).round() / 100.0;

        info!("Acceleration (m/s²): X: {}, Y: {}, Z: {}", accel_x_rounded, accel_y_rounded, accel_z_rounded);

        // Read Gyroscope Data
        cs.set_low();
        let tx_buf = [(1 << 7) | 0x43, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut rx_buf = [0u8; 7];
        spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
        cs.set_high();

        let gyro_x_raw = i16::from_be_bytes([rx_buf[1], rx_buf[2]]);
        let gyro_y_raw = i16::from_be_bytes([rx_buf[3], rx_buf[4]]);
        let gyro_z_raw = i16::from_be_bytes([rx_buf[5], rx_buf[6]]);

        let gyro_x = (gyro_x_raw as f32) * 0.0304878;
        let gyro_y = (gyro_y_raw as f32) * 0.0304878;
        let gyro_z = (gyro_z_raw as f32) * 0.0304878;

        let gyro_x_rounded = (gyro_x * 100.0).round() / 100.0;
        let gyro_y_rounded = (gyro_y * 100.0).round() / 100.0;
        let gyro_z_rounded = (gyro_z * 100.0).round() / 100.0;

        info!("Gyroscope (°/s): X: {}, Y: {}, Z: {}", gyro_x_rounded, gyro_y_rounded, gyro_z_rounded);

        Timer::after(delay).await;
    }
}
