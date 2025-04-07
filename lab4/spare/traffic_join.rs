#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::{join::join, select::select};
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pull};
use embassy_rp::pwm::{Config as ConfigPwm, Pwm};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use panic_probe as _;

static SIG: Signal<CriticalSectionRawMutex, u32> = Signal::new();

enum TrafficState {
    Green,
    Yellow,
    Red,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut red_led = Output::new(peripherals.PIN_6, Level::High);
    let mut yellow_led = Output::new(peripherals.PIN_1, Level::High);
    let mut green_led = Output::new(peripherals.PIN_2, Level::High);

    let mut config: ConfigPwm = Default::default();

    config.top = 0x9088;
    config.compare_a = config.top;

    let sw4_pin = AnyPin::from(peripherals.PIN_15);
    let mut button_sw4 = Input::new(sw4_pin, Pull::Up);

    let sw7_pin = AnyPin::from(peripherals.PIN_14);
    let mut button_sw7 = Input::new(sw7_pin, Pull::Up);

    let mut state = TrafficState::Green;

    loop {
        match state {
            TrafficState::Green => {
                green_led.set_low();
                red_led.set_high();
                yellow_led.set_high();

                let timer = Timer::after(Duration::from_secs(5));
                let buttons = join(
                    button_sw4.wait_for_falling_edge(),
                    button_sw7.wait_for_falling_edge(),
                );
                if select(timer, buttons).await.is_second() {
                    state = TrafficState::Yellow;
                } else {
                    state = TrafficState::Yellow;
                }
                green_led.set_high();
            }
            TrafficState::Yellow => {
                red_led.set_high();
                green_led.set_high();

                for _ in 0..4 {
                    yellow_led.set_low();
                    Timer::after(Duration::from_millis(125)).await;
                    yellow_led.set_high();
                    Timer::after(Duration::from_millis(125)).await;
                }
                state = TrafficState::Red;
            }
            TrafficState::Red => {
                red_led.set_low();
                green_led.set_high();
                yellow_led.set_high();

                let timer = Timer::after(Duration::from_secs(2));
                let buttons = join(
                    button_sw4.wait_for_falling_edge(),
                    button_sw7.wait_for_falling_edge(),
                );
                if select(timer, buttons).await.is_second() {
                    state = TrafficState::Green;
                } else {
                    state = TrafficState::Green;
                }
                red_led.set_high();
            }
        }
    }
}