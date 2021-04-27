use crate::pi_camera::{setup_camera, take_photo};
use crate::structs::CameraSoftwareSettings;
use crate::structs::DisplayMessage;
use std::sync::mpsc::Sender;

/// Gets camera settings from a QR and saves it to the conf file
pub fn initial_setup(display_tx: &Sender<DisplayMessage>) {
    let mut camera = setup_camera(4).expect("Failed to setup camera!");

    let mut is_done = false;

    display_tx
        .send(DisplayMessage {
            status_message: Some("Scan first QR code".to_string()),
            next_image_time: None,
            next_conf_update: None,
        })
        .expect("Failed to send message to display thread!");

    while is_done == false {
        // This println! is just here so that it's easier to see each iteration
        println!();

        println!("Taking QR image");

        // Take an image using the provided SimpleCamera
        let image_vec = match take_photo(&mut camera) {
            Ok(image_vec_ok) => image_vec_ok,
            Err(image_vec_err) => {
                println!("Failed to take photo! The error was {}", image_vec_err);
                continue;
            }
        };

        println!("Decoding QR image");

        // Decode the image vec into a DynamicImage (needed by bardecoder)
        let decoded_image = match image::load_from_memory(&image_vec) {
            Ok(decoded_image_ok) => decoded_image_ok,
            Err(decoded_image_err) => {
                println!(
                    "Failed to decode photo! The error was {}",
                    decoded_image_err
                );
                continue;
            }
        };

        println!("Looking for QR codes");

        // Create a QR decoder and search for QR codes
        let qr_decoder = bardecoder::default_decoder();
        let qr_results = qr_decoder.decode(&decoded_image);

        // This vec will hold all successfully decoded QRs for future use
        let mut decoded_qr_codes: Vec<String> = Vec::new();

        // Iterate through all the found QR codes, and append any successful codes to the decoded_qr_codes vec
        for qr_result in qr_results {
            match qr_result {
                Ok(qr_result_ok) => decoded_qr_codes.push(qr_result_ok),
                Err(qr_result_err) => {
                    println!("Failed to decode QR code! The error was {}", qr_result_err);
                }
            }
        }

        // Iterate through all the decoded QR codes and parse them into CameraSoftwareSettings.
        // The first successful decode will be saved and this function will end.
        for decoded_qr_code in decoded_qr_codes {
            println!("{}", decoded_qr_code);
            let parsed_camera_software_settings =
                match CameraSoftwareSettings::from_qr_string(decoded_qr_code) {
                    Ok(parsed_camera_software_settings_ok) => parsed_camera_software_settings_ok,
                    Err(parsed_camera_software_settings_err) => {
                        println!(
                            "Failed to parse QR code to CameraSoftwareSettings! The error was {}",
                            parsed_camera_software_settings_err
                        );
                        continue;
                    }
                };

            match confy::store("camera-software-new", parsed_camera_software_settings) {
                Ok(_) => is_done = true,
                Err(confy_err) => {
                    println!(
                        "Failed to store CameraSoftwareSettings with confy! The error was {}",
                        confy_err
                    );
                    continue;
                }
            }
        }
    }
}
