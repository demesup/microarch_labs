#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _; 
use embassy_executor::Spawner;
use embassy_futures::{select::Either, select::select};
use embassy_rp::{
    gpio::{AnyPin, Input, Pull},
    pwm::{Config as ConfigPwm, Pwm},
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use panic_probe as _;


static SIG: Signal<CriticalSectionRawMutex, u32> = Signal::new();

#[embassy_executor::task]
async fn manage_led(mut pwm_led: Pwm<'static>) {
    let mut config = ConfigPwm::default();
    config.top = 0x9088;

    loop {
        
        let brightness = SIG.wait().await;

        
        let compare_value = if brightness == 0 {
            0 
        } else {
            
            let scaled_brightness = ((config.top as u32) * (brightness as u32)) / 9;
            scaled_brightness as u16
        };

        
        config.compare_a = config.top.saturating_sub(compare_value);
        pwm_led.set_config(&config);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    
    
    let mut config = ConfigPwm::default();
    config.top = 0x9088;
    config.compare_a = config.top;

    let pwm_led = Pwm::new_output_a(
        peripherals.PWM_SLICE1, 
        peripherals.PIN_2,      
        config.clone(),
    );

    
    spawner.spawn(manage_led(pwm_led)).unwrap();

    
    let sw4_pin = AnyPin::from(peripherals.PIN_5);
    let sw5_pin = AnyPin::from(peripherals.PIN_6);
    let mut button_sw4 = Input::new(sw4_pin, Pull::Up);
    let mut button_sw5 = Input::new(sw5_pin, Pull::Up);

    info!("Hello, world!");

    let mut brightness = 0;

    loop {
        match select(
            button_sw4.wait_for_falling_edge(),
            button_sw5.wait_for_falling_edge(),
        )
        .await
        {
            Either::First(_) => {
                brightness = (brightness + 1) % 10;
                SIG.signal(brightness);
                info!("Increased brightness: {}", brightness);

                
                while button_sw4.is_low() {
                    Timer::after_millis(10).await;
                }
            }
            Either::Second(_) => {
                brightness = (brightness + 9) % 10;
                SIG.signal(brightness);
                info!("Decreased brightness: {}", brightness);

                
                while button_sw5.is_low() {
                    Timer::after_millis(10).await;
                }
            }
        }
    }
}