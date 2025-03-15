use std::error::Error;

use skylink::message::{decode_frame, decode_payload, encode_payload, Altitude, Attitude, DecodedPayload, Gps, Payload};

fn test_streaming_decode() -> Result<(), Box<dyn Error>> {
    println!("==== TEST STREAMING DECODE ====\n");

    // Step 1) Build a valid frame
    let att = Attitude { roll: 110.0, pitch: 20.0, yaw: 30.5 };
    let mut out_buf = [0u8; 128];
    let written = encode_payload(
        Payload::Attitude(&att),
        111,
        222,
        0x7E,  // start
        0x7F,  // end
        &mut out_buf,
    ).unwrap();

    // Step 2) Build a "wire" of data that has some junk before the frame, and maybe some after
    let mut wire = Vec::new();
    // Junk bytes
    wire.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
    wire.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
    wire.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
    wire.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
    wire.extend_from_slice(&[0xAA, 0xBB, 0xCC]);

    // The valid frame
    wire.extend_from_slice(&out_buf[..written]);
    // Some trailing junk
    wire.extend_from_slice(&[0xDE, 0xAD]);
    wire.extend_from_slice(&[0xDE, 0xAD]);
    wire.extend_from_slice(&[0xDE, 0xAD]);
    wire.extend_from_slice(&[0xDE, 0xAD]);

    println!("Streaming input (with junk): {:?}", wire);

    // Step 3) We'll parse the wire byte by byte
    let mut parse_buf = [0u8; 128]; // buffer to collect data for decode
    let mut parse_index = 0usize;
    let mut found_start = false;

    for &b in wire.iter() {
        if !found_start {
            // We haven't seen the start byte yet, so let's skip until we see 0x7E
            if b == 0x7E {
                found_start = true;
                parse_index = 0;
                parse_buf[parse_index] = b;
                parse_index += 1;
            } else {
                // skip
            }
        } else {
            // We have found the start byte, so let's collect data
            parse_buf[parse_index] = b;
            parse_index += 1;

            // Now let's try to decode if possible
            // We'll do a quick attempt at decode_frame. If it fails with an error that
            // implies incomplete data, we continue.
            match decode_frame(&parse_buf[..parse_index]) {
                Ok((frame, bytes_read)) => {
                    // We successfully decoded a full frame
                    println!(
                        "Decoded frame from stream! start=0x{:02X}, end=0x{:02X}, from={}, to={}, type=0x{:02X}",
                        frame.start,
                        frame.end,
                        frame.message.from,
                        frame.message.to,
                        frame.message.message_type
                    );
                    // Decode the payload
                    match decode_payload(&frame).unwrap() {
                        DecodedPayload::Attitude(a) => {
                            println!("Decoded Attitude: roll={}, pitch={}, yaw={}", a.roll, a.pitch, a.yaw);
                        }
                        other => println!("Got unexpected payload: {:?}", other),
                    }
                    println!("Bytes read from parse_buf: {}", bytes_read);

                    // If you expect only one frame, you could break here.
                    // Or, if you expect possibly more frames in the stream, you'd remove
                    // the consumed bytes from your buffer and keep going.
                    break;
                }
                Err(_) => {
                    // Probably incomplete data or a decode error. We'll just keep going,
                    // hoping we get more bytes that complete the frame.
                }
            }
        }
    }

    println!("\n==== END TEST STREAMING DECODE ====\n");
    Ok(())
}

// ============================= Test #1: All Payloads ============================= //

fn test_all_payloads() -> Result<(), Box<dyn Error>> {
    println!("==== TEST ALL PAYLOADS ====\n");

    // Test Attitude
    {
        let att = Attitude { roll: 1.1, pitch: 2.2, yaw: 3.3 };
        let mut out_buf = [0u8; 128];
        let written = encode_payload(
            Payload::Attitude(&att), 100, 200, 0x7E, 0x7F, &mut out_buf
        ).unwrap();
        println!("-- Attitude Test --");
        println!("Encoded bytes: {:?}", &out_buf[..written]);

        let (frame, bytes_read) = decode_frame(&out_buf[..written]).unwrap();
        println!(
            "Decoded Frame: start=0x{:02X}, end=0x{:02X}, from={}, to={}, type=0x{:02X}",
            frame.start, frame.end, frame.message.from, frame.message.to, frame.message.message_type
        );

        match decode_payload(&frame).unwrap() {
            DecodedPayload::Attitude(a) => {
                println!("Decoded Attitude: roll={}, pitch={}, yaw={}", a.roll, a.pitch, a.yaw);
            }
            other => println!("Got unexpected payload: {:?}", other),
        }
        println!("Bytes read during decode: {}\n", bytes_read);
    }

    // Test Altitude
    {
        let alt = Altitude { altitude: 1234.56, climb_rate: 9.87 };
        let mut out_buf = [0u8; 128];
        let written = encode_payload(
            Payload::Altitude(&alt), 101, 201, 0x7E, 0x7F, &mut out_buf
        ).unwrap();
        println!("-- Altitude Test --");
        println!("Encoded bytes: {:?}", &out_buf[..written]);

        let (frame, bytes_read) = decode_frame(&out_buf[..written]).unwrap();
        println!(
            "Decoded Frame: start=0x{:02X}, end=0x{:02X}, from={}, to={}, type=0x{:02X}",
            frame.start, frame.end, frame.message.from, frame.message.to, frame.message.message_type
        );

        match decode_payload(&frame).unwrap() {
            DecodedPayload::Altitude(a) => {
                println!("Decoded Altitude: altitude={}, climb_rate={}", a.altitude, a.climb_rate);
            }
            other => println!("Got unexpected payload: {:?}", other),
        }
        println!("Bytes read during decode: {}\n", bytes_read);
    }

    // Test Gps
    {
        let gps = Gps { lat: 40.12345, lon: -74.98765, alt: 567.89 };
        let mut out_buf = [0u8; 128];
        let written = encode_payload(
            Payload::Gps(&gps), 102, 202, 0x7E, 0x7F, &mut out_buf
        ).unwrap();
        println!("-- GPS Test --");
        println!("Encoded bytes: {:?}", &out_buf[..written]);

        let (frame, bytes_read) = decode_frame(&out_buf[..written]).unwrap();
        println!(
            "Decoded Frame: start=0x{:02X}, end=0x{:02X}, from={}, to={}, type=0x{:02X}",
            frame.start, frame.end, frame.message.from, frame.message.to, frame.message.message_type
        );

        match decode_payload(&frame).unwrap() {
            DecodedPayload::Gps(g) => {
                println!("Decoded GPS: lat={}, lon={}, alt={}", g.lat, g.lon, g.alt);
            }
            other => println!("Got unexpected payload: {:?}", other),
        }
        println!("Bytes read during decode: {}\n", bytes_read);
    }

    println!("==== END TEST ALL PAYLOADS ====\n");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Run the standard “all payloads” test
    test_all_payloads()?;

    // Run a streaming test example, where we place a valid frame *after* some junk bytes,
    // and feed it to a streaming function one byte at a time.
    test_streaming_decode()?;

    Ok(())
}
