use skylink::{
    control::RPYT,
    message::{decoder::Decoder, encoder::Encoder, Message, MessageFrame},
    telemetry::{Altitude, Attitude},
};

fn test_message_encoding_decoding(message: MessageFrame, test_name: &str) {
    println!("\n## Testing {}", test_name);

    let encoder = Encoder{};
    
    // Encode the message
    let encoded = match encoder.encode(message) {
        Ok(data) => {
            println!("✓ Successfully encoded: {:?}", data);
            data
        },
        Err(e) => {
            println!("✗ Encoding failed: {:?}", e);
            return;
        }
    };
    
    // Create a parser with buffer
    let mut parse_buf = [0u8; 64];
    let mut parser = Decoder::new();
    
    // Parse each byte
    for (i, byte) in encoded.iter().enumerate() {
        if let Some(msg) = parser.parse(*byte) {
            println!("decoded this message: {:?}", msg);
        } else {
            // println!("still looking...");
        }
    }
}

// fn test_message_with_gaps(message: Message, test_name: &str, gap_size: usize) {
//     println!("\n## Testing {} with {} byte gap", test_name, gap_size);
    
//     // Encode the message
//     let encoded = match message.encode() {
//         Ok(data) => {
//             println!("✓ Successfully encoded: {:?}", data);
//             data
//         },
//         Err(e) => {
//             println!("✗ Encoding failed: {:?}", e);
//             return;
//         }
//     };
    
//     // Create gapped data
//     let mut gapped = vec![0u8; encoded.len() + gap_size];
//     gapped[gap_size..].copy_from_slice(&encoded);
//     println!("Created gapped data with {} leading zeros", gap_size);
//     println!("Gapped data: {:?}", gapped);
    
//     // Create a parser with buffer
//     let mut parse_buf = [0u8; 64];
//     let mut parser = Parser::new(&mut parse_buf);
    
//     // Parse each byte
//     let mut success = false;
//     for (i, byte) in gapped.iter().enumerate() {
//         if let Some(n) = parser.parse(*byte) {
//             println!("✓ Successfully parsed {} bytes at position {}", n, i);
            
//             // Decode the message
//             match postcard::from_bytes::<Message>(&parse_buf) {
//                 Ok(decoded) => {
//                     println!("✓ Successfully decoded: {:?}", decoded);
//                     // assert_eq!(format!("{:?}", message), format!("{:?}", decoded), 
//                     //            "Original and decoded messages should match");
//                     success = true;
//                 },
//                 Err(e) => println!("✗ Decoding failed: {:?}", e),
//             }
            
//             break;
//         }
//     }
    
//     if !success {
//         println!("✗ Failed to parse message with gaps");
//     }
// }

fn main() {
    println!("# Skylink Message Encoding/Decoding Tests");
    
    // Test 1: Attitude message
    let attitude = Attitude {
        roll: 0.7,
        pitch: 0.8,
        yaw: 0.9,
    };
    let altitudemsg  = Message {

    };

    let msg = MessageFrame {
        start: 0xAA,
        message: msg,
        crc8: 0xFF,
    };
    test_message_encoding_decoding(msg, "Attitude Message");
    // test_message_with_gaps(attitude_msg, "Attitude Message", 5);
    
    // // Test 2: RPYT control message
    // let rpyt = RPYT {
    //     roll: 0.7,
    //     pitch: 0.8,
    //     yaw: 0.9,
    //     throttle: 0.5,
    // };
    // let rpyt_msg = Message::Control(skylink::message::Control::RPYT(rpyt));
    // test_message_encoding_decoding(rpyt_msg.clone(), "RPYT Control Message");
    // test_message_with_gaps(rpyt_msg, "RPYT Control Message", 5);
    
    // Test 3: Altitude message
    let altitude = Altitude {
        altitude: 100.0,
    };
    let altitude_msg = Message::Altitude(altitude);
    test_message_encoding_decoding(altitude_msg.clone(), "Altitude Message");
    // test_message_with_gaps(altitude_msg, "Altitude Message", 5);
    
    println!("\n# All tests completed");
}
