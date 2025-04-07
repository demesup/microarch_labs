#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::Instant;

#[embassy_executor::task(pool_size = 2)]
async fn blink_led(mut led: Output<'static>, interval_ms: u64) {
    loop {
        led.toggle();
        let start_time = Instant::now();
        while start_time.elapsed().as_millis() < interval_ms {}
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    
    let red_led = Output::new(peripherals.PIN_4, Level::Low);
    let blue_led = Output::new(peripherals.PIN_5, Level::Low);
    
    spawner.spawn(blink_led(red_led, 1000)).unwrap();
    spawner.spawn(blink_led(blue_led, 1000)).unwrap();
}

//One of the tasks is not running because busy waiting blocks the executor. Since each task enters a while loop that continuously checks the elapsed time, it never yields control back to the async runtime. Embassy’s async executor is cooperative, meaning tasks must yield (e.g., using await) to allow other tasks to run. However, since our tasks are purely busy-waiting, only the first spawned task gets CPU time, preventing the second one from running.