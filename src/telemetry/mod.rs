use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attitude {
    pub roll: f32,  // degrees
    pub pitch: f32, // degrees
    pub yaw: f32,   // degrees
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Altitude {
    pub altitude: f32, // meters
}
