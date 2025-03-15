#![no_std]

use core::result::Result;

// Import submodule versions of encode/decode functions
use bincode::{
    borrow_decode_from_slice, config::Configuration, error::{DecodeError as BincodeDecodeError, EncodeError as BincodeEncodeError}, BorrowDecode, Encode
};
use bincode::encode_into_slice;   // <--- from bincode::enc
use bincode::decode_from_slice;    // <--- from bincode::de

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

#[derive(Encode, BorrowDecode)]
pub struct MessageFrame {
    pub start: u8,         // e.g. 0x7E
    pub message: Message,  // from + to + message_type + data[55]
    pub end: u8,           // e.g. 0x7F
}

#[derive(Debug)]
pub struct Message {
    pub from: u16,
    pub to: u16,
    pub message_type: u8,
    pub data: [u8; 55],
}

// --- Bincode Encode/Decode for `Message` --- //

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

/// Encode the given payload into a `MessageFrame` stored in `out_serial_buf` without allocation.
///
/// Returns the number of bytes written to `out_serial_buf`.
pub fn encode_payload(
    payload: Payload,
    from: u16,
    to: u16,
    start_byte: u8,
    end_byte: u8,
    out_serial_buf: &mut [u8], // buffer for final encoding
) -> Result<usize, EncodeError> {
    let config = bincode::config::standard();

    // We'll do a local scratch buffer to hold the payload
    let mut local_payload_buf = [0u8; 64];  // enough for small structs

    // 1) encode payload to local scratch
    let payload_size = match payload {
        Payload::Attitude(att) => encode_into_slice(att, &mut local_payload_buf[..], config),
        Payload::Altitude(alt) => encode_into_slice(alt, &mut local_payload_buf[..], config),
        Payload::Gps(gps)      => encode_into_slice(gps, &mut local_payload_buf[..], config),
    }?;

    if payload_size > 55 {
        return Err(EncodeError::PayloadTooLarge(payload_size));
    }

    // 2) Copy into Message data
    let mut data = [0u8; 55];
    data[..payload_size].copy_from_slice(&local_payload_buf[..payload_size]);

    // 3) Build the `MessageFrame`
    let msg_type = match payload {
        Payload::Attitude(_) => MessageType::Attitude as u8,
        Payload::Altitude(_) => MessageType::Altitude as u8,
        Payload::Gps(_)      => MessageType::Gps as u8,
    };

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

    // 4) encode the frame into the caller’s buffer
    let written = encode_into_slice(&frame, out_serial_buf, config)?;
    Ok(written)
}

// ============================= DECODER ============================= //

#[derive(Debug)]
pub enum DecodeError {
    BincodeError,
    UnknownMessageType(u8),
}

impl From<BincodeDecodeError> for DecodeError {
    fn from(e: BincodeDecodeError) -> Self {
        DecodeError::BincodeError
    }
}

#[derive(Debug)]
pub enum DecodedPayload {
    Attitude(Attitude),
    Altitude(Altitude),
    Gps(Gps),
}

/// Decode a `MessageFrame` from bytes.
pub fn decode_frame(incoming: &[u8]) -> Result<(MessageFrame, usize), DecodeError> {
    let config = bincode::config::standard();
    let (frame, bytes_read) = borrow_decode_from_slice::<MessageFrame, _>(incoming, config)?;
    // optionally check frame.start, frame.end, etc.
    Ok((frame, bytes_read))
}

/// Decode the payload inside the frame’s `message` field, picking the right type.
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

// ============================= EXAMPLE USAGE ============================= //

/// Example usage (not a real main, but a demonstration of usage in no_std style).
pub fn example_usage() -> Result<(), DecodeError> {
    // Create a sample Attitude payload
    let att = Attitude { roll: 1.0, pitch: 2.0, yaw: 3.0 };

    // Prepare a buffer for encoded data
    let mut out_buf = [0u8; 128];

    // Encode the payload
    let written = encode_payload(
        Payload::Attitude(&att),
        /* from= */ 111,
        /* to=   */ 222,
        /* start_byte= */ 0x7E,
        /* end_byte=   */ 0x7F,
        &mut out_buf,
    ).map_err(|e| match e {
        EncodeError::Bincode(be) => DecodeError::BincodeError,
        EncodeError::PayloadTooLarge(_) => {
            // handle or convert however you wish
            DecodeError::UnknownMessageType(0)
        }
    })?;

    // out_buf[..written] has the complete serialized frame.

    // Decode the frame
    let (decoded_frame, bytes_read) = decode_frame(&out_buf[..written])?;

    // Decode the payload
    let payload = decode_payload(&decoded_frame)?;
    match payload {
        DecodedPayload::Attitude(a) => {
            // In no_std, we can’t print easily, but we can debug_assert or do something else
            debug_assert_eq!(a.roll, 1.0);
            debug_assert_eq!(a.pitch, 2.0);
            debug_assert_eq!(a.yaw, 3.0);
        }
        _ => {
            // If we expected Attitude and got something else, handle the error
            return Err(DecodeError::UnknownMessageType(decoded_frame.message.message_type));
        }
    }

    debug_assert_eq!(bytes_read, written);
    Ok(())
}
