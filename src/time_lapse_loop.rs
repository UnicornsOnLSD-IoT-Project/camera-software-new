use crate::pi_camera::*;
use crate::structs::CameraSoftwareSettings;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::Method;
use reqwest::Url;
use std::thread::sleep;
use std::time::Duration;

pub fn time_lapse_loop() {
    let reqwest_client = Client::new();
    let camera_software_settings: CameraSoftwareSettings =
        confy::load("camera-software-new").expect("Unable to read config!");

    let mut camera = setup_camera(1).expect("Failed to setup camera!");

    // 10 minutes
    const DELAY: Duration = Duration::from_millis(600000);

    loop {
        let image = take_photo(&mut camera).expect("Failed to take photo!");
        let url = format!("{}/UploadImage", camera_software_settings.base_url);

        let response = reqwest_client
            .post(Url::parse(&url[..]).expect("Failed to parse URL!"))
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

        sleep(DELAY)
    }
}
