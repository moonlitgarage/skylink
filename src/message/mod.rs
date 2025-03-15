pub mod encoder;
pub mod decoder;

// use postcard::{from_bytes, to_vec, to_slice_cobs};
use crate::{errors, telemetry};
use bincode::{BorrowDecode, Decode, Encode};


pub enum MessageType {
    Heartbeat = 0x01,
    Attitude = 0x02,
    Altitude = 0x03,
}

#[derive(Encode, BorrowDecode)]
pub struct MessageFrame {
    pub start: u8,          // 1
    pub message: Message,   // 60
    pub crc8: u8,           // 1
    // total size:      // 62
}

#[derive(Debug)]
pub struct Message {
    pub from: u16,          // 2
    pub to: u16,            // 2
    pub message_type: u8,   // 1
    pub data: [u8; 55],     // 55
    // total size:      // 60
}

impl bincode::Encode for Message {
    fn encode<E: bincode::enc::Encoder>(
        &self, 
        encoder: &mut E
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.from, encoder)?;
        Ok(())
    }
}

impl<'de, Context> bincode::BorrowDecode<'de, Context> for Message {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D
    ) -> Result<Self, bincode::error::DecodeError> {
        Ok(Self { 
            from: bincode::BorrowDecode::borrow_decode(decoder)?, 
            to:  bincode::BorrowDecode::borrow_decode(decoder)?, 
            message_type:  bincode::BorrowDecode::borrow_decode(decoder)?, 
            data:  bincode::BorrowDecode::borrow_decode(decoder)?, 
        })
    }
}

