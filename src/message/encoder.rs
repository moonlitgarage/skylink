use crate::message::Message;
use crate::errors::SkylinkError;

pub const START: u8 = 0xA7;
pub const END:u8 = 0xAA;

pub struct Encoder {

}

impl Encoder {

    pub fn encode(&self, msg: Message) -> Result<[u8;64], SkylinkError> {
        let mut buf = [0u8;61];
        let used = bincode::encode_into_slice(&msg, &mut buf, bincode::config::standard()).unwrap();

        let mut encoded_buf = [0u8;64];
        
        encoded_buf[0] = START;
        // encoded_buf[1] = Self::get_type_byte(msg);
        encoded_buf[2..used+2].copy_from_slice(&buf[0..used]);
        encoded_buf[63] = END;

        Ok(encoded_buf)
    }
}
