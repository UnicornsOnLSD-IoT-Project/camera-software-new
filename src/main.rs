mod camera_client;
mod pi_camera;
mod qr_scanner;
mod structs;

fn main() {
    // If the base url isn't set, assume that initial setup hasn't been completed
    if structs::CameraSoftwareSettings::load_from_confy()
        .expect("Failed to read config!")
        .base_url
        == "NOT_SET"
    {
        qr_scanner::initial_setup();
    }
}
