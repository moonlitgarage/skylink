use bincode::{Decode, Encode};

#[derive(Debug, Decode, Encode, Clone)]
pub struct RPYT {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub throttle: f32,
}
