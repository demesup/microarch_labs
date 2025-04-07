#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config as ConfigAdc, InterruptHandler}; 
use embassy_rp::gpio::{Level, Output, Pull, Input};
use embassy_rp::pwm::{Config as ConfigPwm, Pwm};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
async fn button_pressed(mut led: Output<'static>, mut button: Input<'static>) {
    loop {
	      info!("waiting for button press");
        button.wait_for_falling_edge().await;
        led.toggle();
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let button = Input::new(peripherals.PIN_2, Pull::None);
    let led2 = Output::new(peripherals.PIN_3, Level::Low);

    spawner.spawn(button_pressed(led2, button)).unwrap();

    let mut led = Output::new(peripherals.PIN_4, Level::Low);

    loop {
        led.toggle();
        Timer::after_millis(200).await;
    }
}