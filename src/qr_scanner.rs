use crate::pi_camera::{setup_camera, take_qr_settings};

/// Gets camera settings from a QR and saves it to the conf file
pub fn initial_setup() {
    let camera = setup_camera(4).expect("Failed to setup camera!");
    take_qr_settings(camera);
    // TODO: WiFi setup
}
