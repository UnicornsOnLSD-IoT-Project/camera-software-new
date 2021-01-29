use serde::{Deserialize, Serialize};
use uuid;

/// This struct holds all the camera software settings and is loaded on startup.
#[derive(Serialize, Deserialize, Debug)]
pub struct CameraSoftwareSettings {
    base_url: String,
    camera_token: uuid::Uuid,
}
