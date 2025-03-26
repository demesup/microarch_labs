#![no_std]
#![no_main]

use defmt::{info, warn};
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::init;
use embassy_time::{Timer, Duration};

const LETTERS: [&str; 26] = [
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
    "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
];

fn letter_to_morse(c: char) -> Option<&'static str> {
    if ('a'..='z').contains(&c) {
        Some(LETTERS[(c as u8 - b'a') as usize])
    } else if ('A'..='Z').contains(&c) {
        Some(LETTERS[(c as u8 - b'A') as usize])
    } else {
        None 
    }
}


async fn blink_morse(
    morse_code: &str,
    left_led: &mut Output<'_>,
    middle_led: &mut Output<'_>,
    right_led: &mut Output<'_>,
) {
    for symbol in morse_code.chars() {
        match symbol {
            '.' => {
                middle_led.set_low();
                Timer::after(Duration::from_millis(200)).await;
                middle_led.set_high();
            }
            '-' => {
                left_led.set_low();
                middle_led.set_low();
                right_led.set_low();
                Timer::after(Duration::from_millis(600)).await;
                left_led.set_high();
                middle_led.set_high();
                right_led.set_high();
            }
            _ => {}
        }
        Timer::after(Duration::from_millis(200)).await; 
    }
    Timer::after(Duration::from_millis(600)).await;
}

async fn blink_message(
    message: &str,
    left_led: &mut Output<'_>,
    middle_led: &mut Output<'_>,
    right_led: &mut Output<'_>,
) {
    for (i, c) in message.chars().enumerate() {
        if c == ' ' {
            Timer::after(Duration::from_millis(1000)).await; // Word pause
            continue;
        }

        if let Some(morse_code) = letter_to_morse(c) {
            info!("Displaying Morse for '{}': {}", c, morse_code);
            blink_morse(morse_code, left_led, middle_led, right_led).await;
        }

        // Letter spacing (400ms), except after the last character
        if i < message.len() - 1 {
            Timer::after(Duration::from_millis(400)).await;
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Default::default());

    let mut left_led = Output::new(peripherals.PIN_4, Level::High);
    let mut middle_led = Output::new(peripherals.PIN_5, Level::High);
    let mut right_led = Output::new(peripherals.PIN_6, Level::High);

    info!("Starting Morse Code display");

    let message = "HELLO WORLD";
    blink_message(message, &mut left_led, &mut middle_led, &mut right_led).await;
}
