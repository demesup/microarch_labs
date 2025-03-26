#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    // Trigger a panic by dividing by zero (for demonstration purposes)
    let result = divide(23, 0);
    hprintln!("Result: {}", result);

    loop {}
}

// Divide the two given numbers
fn divide(a: u32, b: u32) -> u32 {
    a / b
}

// Define the panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Print a panic message
    hprintln!("Panic occurred: {:?}", info);

    // Enter an infinite loop to halt the system after a panic
    loop {}
}