use crate::display::display_thread;
use crate::time_lapse_loop::time_lapse_loop;
use std::sync::mpsc;
use std::thread;

mod display;
mod pi_camera;
mod qr_scanner;
mod structs;
mod time_lapse_loop;

fn main() {
    let (display_tx, display_rx) = mpsc::channel::<String>();
    thread::spawn(move || display_thread(display_rx));

    // If the base url isn't set, assume that initial setup hasn't been completed
    if structs::CameraSoftwareSettings::load_from_confy()
        .expect("Failed to read config!")
        .base_url
        == "NOT_SET".to_string()
    {
        qr_scanner::initial_setup();
    }

    time_lapse_loop(&display_tx);
}
