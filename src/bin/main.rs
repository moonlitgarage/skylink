use std::error::Error;

use skylink::message::{
    Attitude, Payload, DecodedPayload,
    encode_payload, decode_frame, decode_payload
};

fn main() -> Result<(), Box<dyn Error>> {
    // 1) Create an Attitude payload
    let att = Attitude {
        roll: 1.1,
        pitch: 2.0,
        yaw: 3.0,
    };

    // 2) Prepare a buffer for our encoded data (no dynamic allocation)
    let mut out_buf = [0u8; 128];

    // 3) Encode the payload into `out_buf[..]` with start=0x7E, end=0x7F
    let written = encode_payload(
        Payload::Attitude(&att),
        /* from= */ 100,
        /* to=   */ 200,
        /* start_byte= */ 0x7E,
        /* end_byte=   */ 0x7F,
        &mut out_buf,
    ).unwrap();

    println!("Encoded Attitude bytes: {:?}", &out_buf[..written]);

    // 4) Decode the full frame from `out_buf[..written]`
    let (frame, bytes_read) = decode_frame(&out_buf[..written]).unwrap();
    println!(
        "Decoded Frame: start=0x{:02X}, end=0x{:02X}, from={}, to={}, type=0x{:02X}",
        frame.start,
        frame.end,
        frame.message.from,
        frame.message.to,
        frame.message.message_type
    );

    // 5) Decode the payload using `decode_payload(&frame)`
    let payload = decode_payload(&frame).unwrap();
    match payload {
        DecodedPayload::Attitude(a) => {
            println!(
                "Got Attitude: roll={}, pitch={}, yaw={}",
                a.roll, a.pitch, a.yaw
            );
        }
        other => {
            println!("Got unexpected payload: {:?}", other);
        }
    }

    // (Optional) Check how many bytes were consumed
    println!("Bytes read during decode: {}", bytes_read);

    Ok(())
}
