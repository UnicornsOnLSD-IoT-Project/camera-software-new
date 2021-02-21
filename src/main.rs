use crate::time_lapse_loop::time_lapse_loop;
use std::sync::mpsc;
use std::thread;

mod pi_camera;
mod qr_scanner;
mod structs;
mod time_lapse_loop;

fn main() {
    // If the base url isn't set, assume that initial setup hasn't been completed
    if structs::CameraSoftwareSettings::load_from_confy()
        .expect("Failed to read config!")
        .base_url
        == "NOT_SET".to_string()
    {
        qr_scanner::initial_setup();
    }

    time_lapse_loop();
}
