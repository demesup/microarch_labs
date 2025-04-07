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

static SIG: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static TRAFFIC_STATE: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy)]
enum TrafficState {
    Green,
    Yellow,
    Red,
}

#[embassy_executor::task]
async fn buzzer_task(mut pwm_buzzer: Pwm<'static>) {
    let mut config: embassy_rp::pwm::Config = Default::default();
    
    loop {
        let state = TRAFFIC_STATE.load(Ordering::Relaxed);
        if state == 0 || state == 1 {
            let freq = 200u32;
            let top = (125_000_000 / (64 * freq)) - 1;
            config.top = top as u16;
            config.compare_b = (top / 2) as u16;
            pwm_buzzer.set_config(&config);
            Timer::after(Duration::from_millis(100)).await;
        } else if state == 2 {
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
    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut red_led = Output::new(peripherals.PIN_0, Level::High);
    let mut yellow_led = Output::new(peripherals.PIN_1, Level::High);
    let mut green_led = Output::new(peripherals.PIN_2, Level::High);

    let mut config: ConfigPwm = Default::default();
    config.top = 0x9088; 
    config.compare_a = config.top;

    let mut pwm_buzzer = Pwm::new_output_b(
        peripherals.PWM_SLICE2,
        peripherals.PIN_5,
        config.clone(),
    );

    spawner.spawn(buzzer_task(pwm_buzzer)).unwrap();

    let sw4_pin = AnyPin::from(peripherals.PIN_3);
    let mut button_sw4 = Input::new(sw4_pin, Pull::Up);
    let sw7_pin = AnyPin::from(peripherals.PIN_4);
    let mut button_sw7 = Input::new(sw7_pin, Pull::Up);

    let mut state = TrafficState::Green;
    TRAFFIC_STATE.store(0, Ordering::Relaxed);

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
                TRAFFIC_STATE.store(1, Ordering::Relaxed);
                green_led.set_high();
            }
            TrafficState::Yellow => {
                red_led.set_high();
                green_led.set_high();
                TRAFFIC_STATE.store(1, Ordering::Relaxed);

                for _ in 0..4 {
                    yellow_led.set_low();
                    Timer::after(Duration::from_millis(125)).await;
                    yellow_led.set_high();
                    Timer::after(Duration::from_millis(125)).await;
                }
                state = TrafficState::Red;
                TRAFFIC_STATE.store(2, Ordering::Relaxed);
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
                TRAFFIC_STATE.store(0, Ordering::Relaxed);
                red_led.set_high();
            }
        }
    }
}