use rppal::i2c::I2c;
use ssd1306::{displaysize::DisplaySize128x32, mode::TerminalMode, Builder, I2CDIBuilder};
use std::fmt::Write;
use std::sync::mpsc::Receiver;

pub fn display_thread(display_rx: Receiver<String>) {
    // Init the display via i2c
    let i2c = I2c::new().expect("Failed to open i2c interface!");

    let interface = I2CDIBuilder::new().init(i2c);
    let builder = Builder::new().size(DisplaySize128x32);

    let mut disp: TerminalMode<_, _> = builder.connect(interface).into();

    disp.init().expect("Failed to init display!");

    // Clear the display in case any old message was on it
    disp.clear().expect("Failed to clear display!");

    // This loop waits for messages and writes them to the display
    loop {
        let message = match display_rx.recv() {
            Ok(display_rx_ok) => display_rx_ok,
            Err(_) => "Failed to recieve message for display".to_string(),
        };

        disp.clear().expect("Failed to clear display!");
        disp.write_str(&message)
            .expect("Failed to write message to display!");
    }
}
