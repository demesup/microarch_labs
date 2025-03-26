#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::adc::{Adc, Channel, Config as ConfigAdc, InterruptHandler};
use embassy_rp::gpio::Pull;
use embassy_time::Timer;
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

    loop {
        // Read a value from the pin
        let level = adc.read(&mut adc_pin).await.unwrap();
        
        // Print the value over serial
        info!("Light sensor reading: {}", level);

        // Wait a bit before reading and printing another value
        Timer::after_secs(1).await;
    }
}
