use bardecoder;
use rascam::*;
use std::{thread, time};

use crate::structs;

/// Gets camera settings from a QR and saves it to the conf file
pub fn save_settings_qr() {
    let info = info().unwrap();
    if info.cameras.len() < 1 {
        panic!("QR Scanner found 0 cameras");
    }

    // Raspberry Pi HQ Cam resolution / 5
    const WIDTH: u32 = 811;
    const HEIGHT: u32 = 608;

    let mut camera = SimpleCamera::new(info.cameras[0].clone()).unwrap();
    let settings = CameraSettings {
        width: WIDTH,
        height: HEIGHT,
        ..CameraSettings::default()
    };
    camera.configure(settings);
    camera.activate().unwrap();

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    loop {
        println!("Taking QR image");
        let img_result = camera.take_one();

        match img_result {
            Ok(img) => {
                let image_file_result = image::load_from_memory(&img);
                match image_file_result {
                    Ok(image_file) => {
                        let decoder = bardecoder::default_decoder();
                        println!("Decoding QR image");

                        let results = decoder.decode(&image_file);
                        for result in results {
                            match result {
                                Ok(msg) => {
                                    let deserialized_msg_option: Result<
                                        structs::CameraSoftwareSettings,
                                        serde_json::Error,
                                    > = serde_json::from_str(msg.as_str());

                                    match deserialized_msg_option {
                                        Ok(deserialized_msg) => {
                                            confy::store("CameraServerSettings", deserialized_msg)
                                                .expect("Failed to store config file");
                                            break;
                                        }
                                        Err(e) => println!(
                                            "Error decoding QR into CameraSoftwareSettings: {}",
                                            e
                                        ),
                                    }
                                }
                                Err(e) => {
                                    println!("{}", e)
                                }
                            }
                        }
                    }
                    Err(e) => println!("Decoding image failed with error: {}", e),
                }
            }
            Err(e) => println!("Taking image failed with error: {}", e),
        }
    }
}
