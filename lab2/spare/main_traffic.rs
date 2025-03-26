#![no_std]
#![no_main]

use defmt::{info, warn};
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::init;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Timer, Duration};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Default::default());
    
    let mut red_led = Output::new(peripherals.PIN_4, Level::Low);
    let mut yellow_led = Output::new(peripherals.PIN_5, Level::Low);
    let mut green_led = Output::new(peripherals.PIN_6, Level::Low);

    
    Timer::after(Duration::from_secs(2)).await;
  
    info!("Turning on all LEDs");
    red_led.set_low();
    green_led.set_low();
    yellow_led.set_low();
    Timer::after(Duration::from_secs(2)).await;
    info!("Turning off all LEDs");
    red_led.set_high();
    green_led.set_high();
    yellow_led.set_high();
    Timer::after(Duration::from_secs(5)).await;

    info!("Starting traffic light sequence");

    loop {
        red_led.set_low();
        Timer::after(Duration::from_secs(3)).await;
        red_led.set_high();
        
        green_led.set_low();
        Timer::after(Duration::from_secs(3)).await;
        green_led.set_high();
        
        yellow_led.set_low();
        Timer::after(Duration::from_secs(1)).await;
        yellow_led.set_high();
    }
}