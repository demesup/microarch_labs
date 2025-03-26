#![no_std]
#![no_main]

use defmt::{info, warn};
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::init;
use embassy_rp::gpio::{Level, Output, Pull, Input};
use embassy_time::{Timer, Duration};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Default::default());
    
    let mut red_led = Output::new(peripherals.PIN_4, Level::High);
    let mut yellow_led = Output::new(peripherals.PIN_5, Level::High);
    let mut green_led = Output::new(peripherals.PIN_6, Level::High);
    let mut blue_led = Output::new(peripherals.PIN_7, Level::High);
    let mut button = Input::new(peripherals.PIN_9, Pull::Up);
    
    info!("Starting traffic light sequence");
    
    loop {
        green_led.set_low();
        
        button.wait_for_falling_edge().await;
        info!("Petedstrian button was pressed");
        yellow_led.set_low();
        green_led.set_high();
        Timer::after(Duration::from_secs(1)).await;

        yellow_led.set_high(); 
        red_led.set_low();
            
        for _ in 0..5 {
            blue_led.set_low();
            Timer::after(Duration::from_millis(500)).await;
            blue_led.set_high(); 
            Timer::after(Duration::from_millis(500)).await;
        }
            
        red_led.set_high(); 
    }
}
