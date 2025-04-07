#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
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

static TRAFFIC_STATE: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy)]
enum TrafficState {
    Green = 0,
    Yellow = 1,
    Red = 2,
}

impl TrafficState {
    fn next(self) -> TrafficState {
        match self {
            TrafficState::Green => TrafficState::Yellow,
            TrafficState::Yellow => TrafficState::Red,
            TrafficState::Red => TrafficState::Green,
        }
    }
}

#[embassy_executor::task]
async fn buzzer_task(mut pwm_buzzer: Pwm<'static>) {
    let mut config: ConfigPwm = Default::default();
    
    loop {
        match TRAFFIC_STATE.load(Ordering::Relaxed) {
            0 | 1 => {
                let freq = 200u32;
                let top = (125_000_000 / (64 * freq)) - 1;
                config.top = top as u16;
                config.compare_b = (top / 2) as u16;
                pwm_buzzer.set_config(&config);
                Timer::after(Duration::from_millis(100)).await;
            }
            2 => {
                let freq = 400u32;
                let top = (125_000_000 / (64 * freq)) - 1;
                config.top = top as u16;
                config.compare_b = (top / 2) as u16;
                pwm_buzzer.set_config(&config);
                Timer::after(Duration::from_millis(200)).await;

                config.compare_b = config.top;
                pwm_buzzer.set_config(&config);
                Timer::after(Duration::from_millis(200)).await;

                config.compare_b = 0;
                pwm_buzzer.set_config(&config);
            }
            _ => (),
        }
    }
}

#[embassy_executor::task]
async fn servo_task(mut servo: Pwm<'static>) {
    let mut servo_config: ConfigPwm = Default::default();
    servo_config.top = 0xB71A;
    servo_config.divider = 64.into();

    const PERIOD_US: usize = 20_000;
    const MIN_PULSE_US: usize = 500;
    const MAX_PULSE_US: usize = 2500;

    let min_pulse = (MIN_PULSE_US * servo_config.top as usize) / PERIOD_US;
    let max_pulse = (MAX_PULSE_US * servo_config.top as usize) / PERIOD_US;

    loop {
        let compare_val = match TRAFFIC_STATE.load(Ordering::Relaxed) {
            0 => max_pulse,
            1 => (min_pulse + max_pulse) / 2,
            2 => min_pulse,
            _ => min_pulse,
        };

        servo_config.compare_a = compare_val as u16;
        servo.set_config(&servo_config);

        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut red_led = Output::new(peripherals.PIN_0, Level::High);
    let mut yellow_led = Output::new(peripherals.PIN_1, Level::High);
    let mut green_led = Output::new(peripherals.PIN_2, Level::High);

    let mut pwm_buzzer = Pwm::new_output_b(
        peripherals.PWM_SLICE2,
        peripherals.PIN_5,
        Default::default(),
    );

    let servo = Pwm::new_output_a(
        peripherals.PWM_SLICE4,
        peripherals.PIN_8,
        Default::default(),
    );

    spawner.spawn(servo_task(servo)).unwrap();
    spawner.spawn(buzzer_task(pwm_buzzer)).unwrap();

    let mut button_sw4 = Input::new(AnyPin::from(peripherals.PIN_3), Pull::Up);
    let mut button_sw7 = Input::new(AnyPin::from(peripherals.PIN_4), Pull::Up);

    let mut state = TrafficState::Green;
    TRAFFIC_STATE.store(state as u32, Ordering::Relaxed);

    loop {
        set_led_state(state, &mut red_led, &mut yellow_led, &mut green_led);

        let timer = Timer::after(match state {
            TrafficState::Green => Duration::from_secs(5),
            TrafficState::Yellow => Duration::from_millis(1000),
            TrafficState::Red => Duration::from_secs(2),
        });

        let buttons = join(
            button_sw4.wait_for_falling_edge(),
            button_sw7.wait_for_falling_edge(),
        );

        select(timer, buttons).await;

        state = state.next();
        TRAFFIC_STATE.store(state as u32, Ordering::Relaxed);
    }
}

async fn set_led_state(state: TrafficState, red: &mut Output<'_>, yellow: &mut Output<'_>, green: &mut Output<'_>) {
    match state {
        TrafficState::Green => {
            green.set_low();
            yellow.set_high();
            red.set_high();
        }
        TrafficState::Yellow => {
            green.set_high();
            red.set_high();

            loop {
                yellow.set_low();
                Timer::after(Duration::from_millis(125)).await;
                if TRAFFIC_STATE.load(Ordering::Relaxed) != TrafficState::Yellow as u32 {
                    break;
                }

                yellow.set_high();
                Timer::after(Duration::from_millis(125)).await;
                if TRAFFIC_STATE.load(Ordering::Relaxed) != TrafficState::Yellow as u32 {
                    break;
                }
            }
            yellow.set_high();
        }
        TrafficState::Red => {
            green.set_high();
            yellow.set_high();
            red.set_low();
        }
    }
}
