#![no_std]
#![no_main]

// we use as _ to avoid a compiler warning
// saying that the crate is not used
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config as ConfigAdc, InterruptHandler};
use embassy_rp::gpio::{self, Pull};
use embassy_rp::pwm::{Config as ConfigPwm, Pwm};
use embassy_rp::usb::In;
use embassy_time::{Duration, Timer};
use panic_probe as _;

use embassy_rp::bind_interrupts;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

enum LightIntensity {
    Low,
    Medium,
    High,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    info!("Hello, world!");
    let mut config: ConfigPwm = Default::default();

    config.top = 0x9088;
    config.compare_a = config.top;
    let mut pwm_red_blue = Pwm::new_output_ab(
        peripherals.PWM_SLICE1, // Channel 1
        peripherals.PIN_2,
        peripherals.PIN_3,
        config.clone(),
    );
    let mut pwm_green = Pwm::new_output_a(
        // Output A
        peripherals.PWM_SLICE2,
        peripherals.PIN_4,
        config.clone(),
    );
    let mut adc = Adc::new(peripherals.ADC, Irqs, ConfigAdc::default());
    let mut adc_pin = Channel::new_pin(peripherals.PIN_26, Pull::None);
    let mut button_pin = gpio::Input::new(peripherals.PIN_10, Pull::Up);
    let mut intensity = LightIntensity::Low;
    loop {
        let adc_value = adc.read(&mut adc_pin).await.unwrap_or(0);
        info!("Light intensity: {}", adc_value);
        intensity = match adc_value {
            0..=1200 => LightIntensity::Low,
            1201..=2730 => LightIntensity::Medium,
            _ => LightIntensity::High,
        };
        match intensity {
            LightIntensity::Low => {
                config.compare_a = 0;
                config.compare_b = config.top;
                pwm_red_blue.set_config(&config);
                config.compare_a = config.top;
                pwm_green.set_config(&config);
            }
            LightIntensity::Medium => {
                config.compare_a = config.top;
                config.compare_b = config.top;
                pwm_red_blue.set_config(&config);
                config.compare_a = 0;
                pwm_green.set_config(&config);
            }
            LightIntensity::High => {
                config.compare_a = config.top;
                config.compare_b = 0;
                pwm_red_blue.set_config(&config);
                config.compare_a = config.top;
                pwm_green.set_config(&config);
            }
        }

        Timer::after(Duration::from_millis(250)).await;
    }
}