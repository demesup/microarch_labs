#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::adc::{Adc, Channel, Config as ConfigAdc, InterruptHandler};
use embassy_rp::gpio::Pull;
use embassy_rp::pwm::{Config as ConfigPwm, Pwm};
use embassy_time::{Duration, Timer};
use defmt::info;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // Create ADC driver
    let mut adc = Adc::new(peripherals.ADC, Irqs, ConfigAdc::default());
    let mut adc_pin = Channel::new_pin(peripherals.PIN_26, Pull::None);

    // PWM configuration for LED
    let mut config: ConfigPwm = Default::default();
    config.top = 0x9088; // in HEX, equals 37000 in decimal
    config.compare_a = config.top / 2;

    let mut pwm = Pwm::new_output_a(
        peripherals.PWM_SLICE1, // Channel 1
        peripherals.PIN_2,
        config.clone(),
    );

    loop {
        let level = adc.read(&mut adc_pin).await.unwrap();

        let brightness = (level as u16 * config.top) / 4095;

        config.compare_a = config.top - brightness;
        pwm.set_config(&config);

        info!("Potentiometer reading: {}", level);

        Timer::after(Duration::from_millis(100)).await;
    }
}
