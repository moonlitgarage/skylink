const START: u8 = 0xCC;
const END: u8 = 0xFF;

pub struct Message {
    start: u8, // 0
    pub payload: [u8; 100], // 1..101
    crc: u8, // 101
    end: u8,
}

pub struct Payload {
    pub kind: u8,
    pub data: [u8; 99]
}

impl Message {
    pub fn new(payload: Payload) -> Self {
        let mut payload_buf: [u8; 100] = [0u8; 100];
        payload_buf[0] = payload.kind;
        for i in 0..payload.data.len() {
            payload_buf[i+1] = payload.data[i];
        }

        Message { 
            start: START, 
            payload: payload_buf, 
            crc: 0, 
            end: END,
        }
    }

    pub fn to_bytes(&self) -> [u8;103] {
        let mut buffer = [0u8; 103];

        buffer[0] = self.start;
        buffer[1..101].copy_from_slice(&self.payload);
        buffer[101] = self.crc;
        buffer[102] = self.end;

        buffer
    }

    pub fn from_bytes(bytes: &[u8; 103]) -> Message {
        // Ensure that the first and last bytes are valid marker

        let mut payload = [0u8; 100];
        payload.copy_from_slice(&bytes[1..101]);

        Message {
            start: bytes[0],
            payload,
            crc: bytes[101],
            end: bytes[102],
        }
    }
}

