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

const ACCEL_XOUT_H: u8 = 0x3B;
const GYRO_XOUT_H: u8 = 0x43;

async fn read_sensor_data(spi: &mut Spi<'_, _, _>, cs: &mut Output<'_>, high_byte_reg: u8) -> i16 {
    let tx_buf = [high_byte_reg | 0x80, 0x00]; 
    let mut rx_buf = [0u8; 2];

    cs.set_low();
    spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
    cs.set_high();

    i16::from_be_bytes([rx_buf[0], rx_buf[1]]) 
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut config = SpiConfig::default();
    config.frequency = 1_000_000;
    config.phase = Phase::CaptureOnFirstTransition;
    config.polarity = Polarity::IdleLow;

    let miso = p.PIN_12;
    let mosi = p.PIN_15;
    let clk = p.PIN_14;

    let mut spi = Spi::new(p.SPI1, clk, mosi, miso, p.DMA_CH0, p.DMA_CH1, config);
    let mut cs = Output::new(p.PIN_13, Level::High);

    let delay = Duration::from_millis(500);

    loop {
        let raw_accel_x = read_sensor_data(&mut spi, &mut cs, ACCEL_XOUT_H).await;
        let raw_gyro_x = read_sensor_data(&mut spi, &mut cs, GYRO_XOUT_H).await;

        let accel_x_m_s2 = (raw_accel_x as f32 / 16384.0) * 9.81;
        let gyro_x_dps = raw_gyro_x as f32 / 32.8;

        info!("Acceleration X: {:.2} m/s²", accel_x_m_s2);
        info!("Gyroscope X: {:.2} °/s", gyro_x_dps);

        Timer::after(delay).await;
    }
}
