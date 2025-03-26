#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use defmt::{error, info};   // Import defmt macros for logging
use defmt_rtt as _;         // Import defmt_rtt for RTT logging
use rp235x_hal::block::ImageDef;  // Assuming you are using this for secure execution block

// Define your static IMAGE_DEF if needed
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[entry]
fn main() -> ! {
    // Log some information
    info!("Starting main function");

    // Trigger a panic by dividing by zero (for demonstration purposes)
    let result = divide(23, 0);  // This will cause a panic due to divide by zero
    info!("Result: {}", result);

    loop {}
}

// Divide the two given numbers
fn divide(a: u32, b: u32) -> u32 {
    a / b  // Will panic if b == 0
}

// Define the panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Print a panic message using defmt error
    error!("Panic occurred: {:?}", info);

    // Enter an infinite loop to halt the system after a panic
    loop {}
}
