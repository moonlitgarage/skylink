use bincode::{Decode, Encode};

#[derive(Debug, Decode, Encode, Clone)]
pub struct Attitude {
    pub roll: f32,  // degrees
    pub pitch: f32, // degrees
    pub yaw: f32,   // degrees
}

#[derive(Debug, Decode, Encode, Clone)]
pub struct Altitude {
    pub altitude: f32, // meters
}
