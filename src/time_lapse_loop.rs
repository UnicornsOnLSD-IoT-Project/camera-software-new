use crate::pi_camera::*;
use crate::structs::CameraSoftwareSettings;
use crate::structs::DisplayMessage;
use chrono::offset::Utc;
use chrono::DateTime;
use rascam::*;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

pub fn time_lapse_loop(display_tx: &Sender<DisplayMessage>) {
    let reqwest_client = Client::new();
    let camera_software_settings: CameraSoftwareSettings =
        confy::load("camera-software-new").expect("Unable to read config!");

    // 30 minutes
    const DELAY: Duration = Duration::from_millis(3600000 / 2);

    const WIDTH: u32 = 4056;
    const HEIGHT: u32 = 3040;

    let mut counter = 0;

    let mut camera = setup_camera(1).expect("Failed to setup camera!");

    loop {
        counter = counter + 1;
        println!("{}", counter);

        camera.configure(CameraSettings {
            width: WIDTH,
            height: HEIGHT,
            encoding: MMAL_ENCODING_JPEG,
            ..CameraSettings::default()
        });

        display_tx
            .send(DisplayMessage {
                status_message: Some("Taking photo".to_string()),
                next_image_time: None,
                next_conf_update: None,
            })
            .expect("Failed to send message to display thread!");

        let image = take_photo(&mut camera).expect("Failed to take photo!");
        let url = format!("{}/UploadImage", camera_software_settings.base_url);

        display_tx
            .send(DisplayMessage {
                status_message: Some("Uploading photo".to_string()),
                next_image_time: None,
                next_conf_update: None,
            })
            .expect("Failed to send message to display thread!");

        let response = reqwest_client
            .post(Url::parse(url.as_str()).expect("Failed to parse URL!"))
            .body(image)
            .header(
                "camera_token",
                camera_software_settings.camera_token.to_string(),
            )
            .header(CONTENT_TYPE, "image/jpeg")
            .send();

        match response {
            Ok(response_ok) => println!("{}", response_ok.text().unwrap()),
            Err(response_err) => println!("{}", response_err),
        }

        match SystemTime::now().checked_add(DELAY) {
            Some(next_image_time) => {
                let datetime: DateTime<Utc> = next_image_time.into();
                let display_message = DisplayMessage {
                    status_message: Some("Ready".to_string()),
                    next_image_time: Some(datetime),
                    next_conf_update: None,
                };
                display_tx
                    .send(display_message)
                    .expect("Failed to send message to display thread!")
            }
            None => {
                let display_message = DisplayMessage {
                    status_message: Some("Ready (failed to get next image time)".to_string()),
                    next_image_time: None,
                    next_conf_update: None,
                };
                display_tx
                    .send(display_message)
                    .expect("Failed to send message to display thread!")
            }
        };

        println!();

        sleep(DELAY)
    }
}
