#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, Timer};
use embassy_futures::select::{self, Either, select};

#[derive(Clone, Copy, PartialEq)]
enum TrafficLightState {
    Green,
    Yellow,
    Red,
}

#[embassy_executor::task]
async fn traffic_light(mut green: Output<'static>, mut yellow: Output<'static>, mut red: Output<'static>, mut button: Input<'static>) {
    let mut state = TrafficLightState::Green;

    loop {
        match state {
            TrafficLightState::Green => {
                red.set_high();
                yellow.set_high();
                green.set_low();
                info!("State: Green");
                let timer = Timer::after_secs(5);
                select(timer, button.wait_for_falling_edge()).await;
                green.set_high();
                state = TrafficLightState::Yellow;
    
            }
            TrafficLightState::Yellow => {
                green.set_high();
                red.set_high();
                for _ in 0..4 {
                    yellow.toggle();
                    let timer = Timer::after_millis(250);
                    select(timer, button.wait_for_falling_edge()).await;
                }
                yellow.set_high();
                state = TrafficLightState::Red;
            }
            TrafficLightState::Red => {
                green.set_high();
                yellow.set_high();
                red.set_low();
                info!("State: Red");
                let timer = Timer::after_secs(2);
                match select(
                    timer,
                    button.wait_for_falling_edge(),
                )
                .await
                {
                    Either::First(_) => {
                        red.set_high();
                        state = TrafficLightState::Green;
                    }
                    Either::Second(_) => {
                        state = TrafficLightState::Red;
                        info!("Back to Red");
                        Timer::after(Duration::from_millis(150)).await;
                    }
                }
               
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    
    let green_led = Output::new(peripherals.PIN_4, Level::Low);
    let yellow_led = Output::new(peripherals.PIN_5, Level::Low);
    let red_led = Output::new(peripherals.PIN_6, Level::Low);
    let button = Input::new(peripherals.PIN_7, Pull::Up);
    
    spawner.spawn(traffic_light(green_led, yellow_led, red_led, button)).unwrap();
}
