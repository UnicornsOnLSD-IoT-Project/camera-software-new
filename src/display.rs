use crate::structs::DisplayMessage;

use chrono::offset::Utc;
use chrono::DateTime;
use rppal::i2c::I2c;
use ssd1306::{displaysize::DisplaySize128x32, mode::TerminalMode, Builder, I2CDIBuilder};
use std::fmt::Write;
use std::sync::mpsc::Receiver;

pub fn display_thread(display_rx: Receiver<DisplayMessage>) {
    // Init the display via i2c
    let i2c = I2c::new().expect("Failed to open i2c interface!");

    let interface = I2CDIBuilder::new().init(i2c);
    let builder = Builder::new().size(DisplaySize128x32);

    let mut disp: TerminalMode<_, _> = builder.connect(interface).into();

    disp.init().expect("Failed to init display!");

    // Clear the display in case any old message was on it
    disp.clear().expect("Failed to clear display!");

    let mut status_message: Option<String> = None;
    let mut next_image_time: Option<DateTime<Utc>> = None;
    let mut next_conf_update: Option<DateTime<Utc>> = None;

    // This loop waits for messages and writes them to the display
    loop {
        let message = match display_rx.recv() {
            Ok(display_rx_ok) => display_rx_ok,
            Err(_) => DisplayMessage {
                status_message: Some("Failed to receive display message".to_string()),
                next_image_time: None,
                next_conf_update: None,
            },
        };

        // Here, we check what we received from the message. Options that do exist are set to the variables we created earlier.
        // If something is none, it is not set. For example, if a new conf time is set, the code doing that won't want to send a new next img time.
        if message.status_message.is_some() {
            status_message = message.status_message;
        }
        if message.next_image_time.is_some() {
            next_image_time = message.next_image_time;
        }
        if message.next_conf_update.is_some() {
            next_conf_update = message.next_conf_update;
        }

        let mut display_string = String::new();

        // Check if the message variables are set, and append them to display_string if they are.
        if status_message.is_some() {
            display_string.push_str(&status_message.clone().unwrap());
            display_string.push_str("\n");
        }
        if next_image_time.is_some() {
            display_string.push_str("img: ");
            display_string.push_str(&next_image_time.unwrap().format("%H:%M:%S").to_string());
            display_string.push_str("\n");
        }
        if next_conf_update.is_some() {
            display_string.push_str("conf: ");
            display_string.push_str(&next_conf_update.unwrap().format("%H:%M:%S").to_string());
        }

        disp.clear().expect("Failed to clear display!");
        disp.write_str(&display_string)
            .expect("Failed to write message to display!");
    }
}
