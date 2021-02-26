use rascam::*;
use std::{thread, time};

pub fn setup_camera(resolution_divider: u32) -> Result<SimpleCamera, CameraError> {
    let info = info().unwrap();
    if info.cameras.len() < 1 {
        panic!("QR Scanner found 0 cameras");
    }

    // Raspberry Pi HQ Cam resolution / resolution_divider
    let width: u32 = 4056 / resolution_divider;
    let height: u32 = 3040 / resolution_divider;

    let mut camera = SimpleCamera::new(info.cameras[0].clone())?;
    let settings = CameraSettings {
        width: width,
        height: height,
        encoding: MMAL_ENCODING_JPEG,
        ..CameraSettings::default()
    };
    camera.configure(settings);
    camera.activate()?;

    // We sleep here to "warm up" the camera
    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    Ok(camera)
}

pub fn take_photo(camera: &mut SimpleCamera) -> Result<Vec<u8>, CameraError> {
    camera.take_one()
}
