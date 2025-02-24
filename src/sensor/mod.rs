use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gyro {
    pub x: f32, // rad/s
    pub y: f32, // rad/s
    pub z: f32, // rad/s
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Accel {
    pub x: f32, // m/s^2
    pub y: f32, // m/s^2
    pub z: f32, // m/s^2
}
