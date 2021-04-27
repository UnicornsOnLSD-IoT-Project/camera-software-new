use crate::structs::CameraSoftwareSettings;
use crate::structs::Config;
use crate::structs::DisplayMessage;
use chrono::offset::Local;
use chrono::DateTime;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use std::convert::TryInto;
use std::process::Command;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

pub fn time_lapse_loop(display_tx: &Sender<DisplayMessage>) {
    const WIDTH: u16 = 4056;
    const HEIGHT: u16 = 3040;
    const SLEEP_INTERVAL: Duration = Duration::from_secs(5);

    let reqwest_client = Client::new();
    let camera_software_settings: CameraSoftwareSettings =
        confy::load("camera-software-new").expect("Unable to read config!");

    let mut last_image_time: DateTime<Local> = DateTime::from(SystemTime::UNIX_EPOCH);

    loop {
        let config_response = reqwest_client
            .get(
                Url::parse(&format!(
                    "{}/Cameras/GetConfigCamera",
                    camera_software_settings.base_url
                ))
                .expect("Failed to parse URL!"),
            )
            .header(
                "camera_token",
                camera_software_settings.camera_token.to_string(),
            )
            .send();

        if config_response.is_err() {
            display_tx
                .send(DisplayMessage {
                    status_message: Some("NETWORK ERROR, RESTART REQUIRED".to_string()),
                    next_image_time: None,
                    next_conf_update: None,
                })
                .expect("Failed to send message to display thread!");
            panic!();
        }

        let config: Config = config_response
            .unwrap()
            .json()
            .expect("Failed to deserialise config!");

        let image_interval = Duration::from_secs((config.interval * 60).try_into().unwrap());

        if last_image_time
            .checked_add_signed(
                chrono::Duration::from_std(image_interval)
                    .expect("Failed to convert std duration to chrono duration!"),
            )
            .expect("Failed to add delay!")
            < DateTime::<Local>::from(SystemTime::now())
        {
            display_tx
                .send(DisplayMessage {
                    status_message: Some("Taking photo".to_string()),
                    next_image_time: None,
                    next_conf_update: None,
                })
                .expect("Failed to send message to display thread!");

            let raspistill_output = Command::new("raspistill")
                .args(&[
                    "-o",
                    "-",
                    "-w",
                    &WIDTH.to_string(),
                    "-h",
                    &HEIGHT.to_string(),
                ])
                .output()
                .expect("Failed to execute raspistill!");

            let url = format!("{}/UploadImage", camera_software_settings.base_url);

            display_tx
                .send(DisplayMessage {
                    status_message: Some("Uploading photo".to_string()),
                    next_image_time: None,
                    next_conf_update: None,
                })
                .expect("Failed to send message to display thread!");

            let response = reqwest_client
                .post(Url::parse(&url).expect("Failed to parse URL!"))
                .body(raspistill_output.stdout)
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

            match SystemTime::now().checked_add(image_interval) {
                Some(next_image_time) => {
                    last_image_time = DateTime::<Local>::from(SystemTime::now());
                    let datetime: DateTime<Local> = next_image_time.into();
                    let display_message = DisplayMessage {
                        status_message: Some("Ready".to_string()),
                        next_image_time: Some(datetime),
                        next_conf_update: Some(
                            SystemTime::now()
                                .checked_add(SLEEP_INTERVAL)
                                .expect("Failed to calculate sleep interval!")
                                .into(),
                        ),
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
        } else {
            display_tx
                .send(DisplayMessage {
                    status_message: None,
                    next_image_time: None,
                    next_conf_update: Some(
                        SystemTime::now()
                            .checked_add(SLEEP_INTERVAL)
                            .expect("Failed to calculate sleep interval!")
                            .into(),
                    ),
                })
                .expect("Failed to send message to display thread!")
        }
        sleep(SLEEP_INTERVAL);
    }
}
