
use bincode::{
    borrow_decode_from_slice, config::Configuration, error::{DecodeError as BincodeDecodeError, EncodeError as BincodeEncodeError}, BorrowDecode, Encode
};
use bincode::encode_into_slice;
use bincode::decode_from_slice;



// ============================= PAYLOAD TYPES ============================= //

#[derive(Encode, BorrowDecode, Debug)]
pub struct Attitude {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Encode, BorrowDecode, Debug)]
pub struct Altitude {
    pub altitude: f32,
    pub climb_rate: f32,
}

#[derive(Encode, BorrowDecode, Debug)]
pub struct Gps {
    pub lat: f64,
    pub lon: f64,
    pub alt: f32,
}

// Possible message types
#[derive(Debug)]
pub enum MessageType {
    Heartbeat = 0x01,
    Attitude  = 0x02,
    Altitude  = 0x03,
    Gps       = 0x04,
}

// ============================= MESSAGE + FRAME ============================= //

/// Our frame includes a start byte, a `Message`, and an end byte.
#[derive(Encode, BorrowDecode)]
pub struct MessageFrame {
    pub start: u8,        // e.g. 0x7E
    pub message: Message, // from + to + message_type + data[55]
    pub end: u8,          // e.g. 0x7F
}

#[derive(Debug)]
pub struct Message {
    pub from: u16,
    pub to: u16,
    pub message_type: u8,
    pub data: [u8; 55],
}

impl bincode::Encode for Message {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E
    ) -> Result<(), BincodeEncodeError> {
        bincode::Encode::encode(&self.from, encoder)?;
        bincode::Encode::encode(&self.to,   encoder)?;
        bincode::Encode::encode(&self.message_type, encoder)?;
        bincode::Encode::encode(&self.data, encoder)?;
        Ok(())
    }
}

impl<'de, Context> bincode::BorrowDecode<'de, Context> for Message {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D
    ) -> Result<Self, BincodeDecodeError> {
        Ok(Self {
            from: bincode::BorrowDecode::borrow_decode(decoder)?,
            to:   bincode::BorrowDecode::borrow_decode(decoder)?,
            message_type: bincode::BorrowDecode::borrow_decode(decoder)?,
            data: bincode::BorrowDecode::borrow_decode(decoder)?,
        })
    }
}

// ============================= ENCODER ============================= //

#[derive(Debug)]
pub enum EncodeError {
    Bincode(BincodeEncodeError),
    PayloadTooLarge(usize),
}

impl From<BincodeEncodeError> for EncodeError {
    fn from(e: BincodeEncodeError) -> Self {
        EncodeError::Bincode(e)
    }
}

/// Enum that references each possible payload type
pub enum Payload<'a> {
    Attitude(&'a Attitude),
    Altitude(&'a Altitude),
    Gps(&'a Gps),
}

/// Encode the given payload into a `MessageFrame`, writing into `out_serial_buf`.
///
/// Returns the number of bytes written to `out_serial_buf`.
pub fn encode_payload(
    payload: Payload,
    from: u16,
    to: u16,
    start_byte: u8,
    end_byte: u8,
    out_serial_buf: &mut [u8],
) -> Result<usize, EncodeError> {
    let config = bincode::config::standard();

    // We'll do a local scratch buffer to hold just the encoded payload
    let mut local_payload_buf = [0u8; 64];  // enough for small payloads
    let payload_size = match payload {
        Payload::Attitude(att) => encode_into_slice(att, &mut local_payload_buf[..], config),
        Payload::Altitude(alt) => encode_into_slice(alt, &mut local_payload_buf[..], config),
        Payload::Gps(gps)      => encode_into_slice(gps, &mut local_payload_buf[..], config),
    }?;

    if payload_size > 55 {
        return Err(EncodeError::PayloadTooLarge(payload_size));
    }

    // Copy payload bytes into the 55-byte data array
    let mut data = [0u8; 55];
    data[..payload_size].copy_from_slice(&local_payload_buf[..payload_size]);

    // Determine message type
    let msg_type = match payload {
        Payload::Attitude(_) => MessageType::Attitude as u8,
        Payload::Altitude(_) => MessageType::Altitude as u8,
        Payload::Gps(_)      => MessageType::Gps as u8,
    };

    // Construct the frame
    let message = Message {
        from,
        to,
        message_type: msg_type,
        data,
    };

    let frame = MessageFrame {
        start: start_byte,
        message,
        end: end_byte,
    };

    // Encode to the caller’s buffer
    let written = encode_into_slice(&frame, out_serial_buf, config)?;
    Ok(written)
}

// ============================= DECODER ============================= //

#[derive(Debug)]
pub enum DecodeError {
    BincodeError(BincodeDecodeError),
    UnknownMessageType(u8),
}

impl From<BincodeDecodeError> for DecodeError {
    fn from(e: BincodeDecodeError) -> Self {
        DecodeError::BincodeError(e)
    }
}

#[derive(Debug)]
pub enum DecodedPayload {
    Attitude(Attitude),
    Altitude(Altitude),
    Gps(Gps),
}

/// Decode a `MessageFrame` from bytes, returning `(frame, bytes_read)`.
pub fn decode_frame(incoming: &[u8]) -> Result<(MessageFrame, usize), DecodeError> {
    let config = bincode::config::standard();
    let (frame, bytes_read) = borrow_decode_from_slice::<MessageFrame, _>(incoming, config)?;
    Ok((frame, bytes_read))
}

/// Decode the 55-byte data in `frame.message` into the correct payload type.
pub fn decode_payload(frame: &MessageFrame) -> Result<DecodedPayload, DecodeError> {
    let config = bincode::config::standard();

    match frame.message.message_type {
        x if x == MessageType::Attitude as u8 => {
            let (att, _) = borrow_decode_from_slice::<Attitude, _>(&frame.message.data, config)?;
            Ok(DecodedPayload::Attitude(att))
        }
        x if x == MessageType::Altitude as u8 => {
            let (alt, _) = borrow_decode_from_slice::<Altitude, _>(&frame.message.data, config)?;
            Ok(DecodedPayload::Altitude(alt))
        }
        x if x == MessageType::Gps as u8 => {
            let (gps, _) = borrow_decode_from_slice::<Gps, _>(&frame.message.data, config)?;
            Ok(DecodedPayload::Gps(gps))
        }
        _ => Err(DecodeError::UnknownMessageType(frame.message.message_type)),
    }
}




