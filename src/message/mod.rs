use serde::{Deserialize, Serialize};
use postcard::{from_bytes, to_vec};
use crate::errors::Error;
use crate::telemetry::{Attitude, Altitude};
use crate::control::ControlInputRaw;
use crate::sensor::{Gyro, Accel};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    // Telemetry messages
    TelAttitude(Attitude),
    TelAltitude(Altitude),
    
    // Control messages
    ConControlInputRaw(ControlInputRaw),
    ConGroundSpeed(f32),
    ConAltitude(f32),
    ConHeading(f32),

    // Sensor messages
    SenGyro(Gyro),
    SenAccel(Accel),
}

impl Message {
    pub fn to_vec(&self) -> Result<heapless::Vec<u8, 64>, Error> {
        let encoded: heapless::Vec<u8, 64> = to_vec(self).unwrap();
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Message, postcard::Error> {
        from_bytes(bytes)
    }
}
