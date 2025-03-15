use crate::{message::Message, telemetry::Attitude};

pub struct Decoder {
    buffer: [u8;64],
    typebyte: u8,
    idx: usize,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            buffer: [0u8;64],
            typebyte: 0,
            idx: 0,
        }
    }

    pub fn parse(&mut self, byte: u8) -> Option<Message> {
        if self.idx == 0 {
            if byte == crate::message::encoder::START {
                self.idx = 1;
                return None;
            }
        } else if self.idx == 1 {
                self.typebyte == byte;
                self.idx += 1;
                return None;
        } else {
            if byte == crate::message::encoder::END {
                let end_idx = self.idx;
                self.idx = 0;

                // let mut databuf = [0u8; 62].copy_from_slice(self.buffer[1..end_idx]);

                let decoded = match self.typebyte {
                    0x03 => {
                        let (decoded, len): (Message, usize) = bincode::borrow_decode_from_slice(&self.buffer[1..end_idx+1], bincode::config::standard()).unwrap();
                        decoded
                    }
                    _ => {
                        let (decoded, len): (Message, usize) = bincode::borrow_decode_from_slice(&self.buffer[1..end_idx+1], bincode::config::standard()).unwrap();
                        decoded
                    }
                };
                return Some(decoded);
            }
        }

        None
    }
}

