#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Level, Output},
    peripherals::{DMA_CH0, DMA_CH1, SPI1},
    spi::{Config as SpiConfig, Phase, Polarity, Spi, Async},
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use defmt::*;


const WHO_AM_I: u8 = 0x75;
const PWR_MGMT_1: u8 = 0x6B;
const ACCEL_CONFIG: u8 = 0x1C;
const GYRO_CONFIG: u8 = 0x1B;
const ACCEL_XOUT_H: u8 = 0x3B; 
const GYRO_XOUT_H: u8 = 0x43;  

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

    let mut spi = Spi::<SPI1, Async>::new(p.SPI1, clk, mosi, miso, p.DMA_CH0, p.DMA_CH1, config);
    let mut cs = Output::new(p.PIN_13, Level::High);
    let delay = Duration::from_secs(1);

    
    cs.set_low();
    spi.write(&[PWR_MGMT_1, 0x00]).await.unwrap();
    cs.set_high();

    
    cs.set_low();
    spi.write(&[ACCEL_CONFIG, 0x00]).await.unwrap(); 
    cs.set_high();

    
    cs.set_low();
    spi.write(&[GYRO_CONFIG, 0x10]).await.unwrap(); 
    cs.set_high();

    info!("MPU-6000 Initialized!");

    let delay = Duration::from_millis(500);

    loop {
        
        let raw_accel_x = read_sensor_data(&mut spi, &mut cs, ACCEL_XOUT_H).await;
        info!("Raw Acceleration X: {}", raw_accel_x);

        
        let raw_gyro_x = read_sensor_data(&mut spi, &mut cs, GYRO_XOUT_H).await;
        info!("Raw Gyroscope X: {}", raw_gyro_x);

        Timer::after(delay).await;
    }
}

async fn read_sensor_data(
    spi: &mut Spi<'_, SPI1, Async>,
    cs: &mut Output<'_>,
    high_byte_reg: u8,
) -> i16 {
    let tx_buf = [(1 << 7) | high_byte_reg, 0x00, 0x00]; 
    let mut rx_buf = [0u8; 3];

    cs.set_low();
    spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
    cs.set_high();

    i16::from_be_bytes([rx_buf[1], rx_buf[2]])
}
