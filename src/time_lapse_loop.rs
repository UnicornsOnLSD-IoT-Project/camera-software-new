use crate::pi_camera::*;
use crate::structs::CameraSoftwareSettings;
use chrono::offset::Utc;
use chrono::DateTime;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

pub fn time_lapse_loop(display_tx: &Sender<String>) {
    let reqwest_client = Client::new();
    let camera_software_settings: CameraSoftwareSettings =
        confy::load("camera-software-new").expect("Unable to read config!");

    // 10 minutes
    const DELAY: Duration = Duration::from_millis(600000);

    loop {
        let mut camera = setup_camera(1).expect("Failed to setup camera!");

        display_tx
            .send("Taking photo".to_string())
            .expect("Failed to send message to display thread!");

        let image = take_photo(&mut camera).expect("Failed to take photo!");
        let url = format!("{}/UploadImage", camera_software_settings.base_url);

        display_tx
            .send("Uploading photo".to_string())
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
                display_tx
                    .send(format!("Next image:\n{}", datetime.format("%H:%M:%S")))
                    .expect("Failed to send message to display thread!")
            }
            None => display_tx
                .send("Waiting...".to_string())
                .expect("Failed to send message to display thread!"),
        };

        sleep(DELAY)
    }
}
