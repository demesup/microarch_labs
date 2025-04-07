#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::spi::{Config, Phase, Polarity, Spi};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_sync::blocking_mutex::{Mutex, raw::NoopRawMutex};
use embassy_time::Delay;
use display_interface_spi::SPIInterface;
use mipidsi::models::ST7735s;
use mipidsi::options::{Orientation, Rotation};
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    
    let mut screen_config = Config::default();
    screen_config.frequency = 32_000_000u32;
    screen_config.phase = Phase::CaptureOnSecondTransition;
    screen_config.polarity = Polarity::IdleHigh;

    
    let mosi = p.PIN_15;
    let clk = p.PIN_14;
    let miso = p.PIN_12;
    
    
    let screen_rst = Output::new(p.PIN_16, Level::Low);  
    let screen_dc = Output::new(p.PIN_17, Level::Low);   
    let screen_cs = Output::new(p.PIN_9, Level::High);  

    
    let spi = Spi::new_blocking(p.SPI1, clk, mosi, miso, screen_config);
    
    
    let spi_bus: Mutex<NoopRawMutex, RefCell<_>> = Mutex::new(RefCell::new(spi));
    let display_spi = SpiDevice::new(&spi_bus, screen_cs);

    
    let di = SPIInterface::new(display_spi, screen_dc);
    let mut screen = mipidsi::Builder::new(ST7735s, di)
        .reset_pin(screen_rst)
        .orientation(Orientation::new().rotate(Rotation::Deg180))
        .init(&mut Delay)
        .unwrap();

    
    screen.clear(Rgb565::BLACK).unwrap();

    
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
   loop {
        Text::new("Hello!", Point::new(20, 20), style)
            .draw(&mut screen)
            .unwrap();
   }
}
