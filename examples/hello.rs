use bxcan::Data;
use can_aerospace_lite::{message::{CANAerospaceFrame, RawMessage}, types::{MessageType}};

fn main() {
    let data: [u8; 7] = [1,2,3,4,5,6,7];
    let raw_message = RawMessage::from(data);
    let frame = CANAerospaceFrame{
        frame_type: MessageType::EED(10),
        message: raw_message
    };

    println!("Filled {:?}", frame);
    println!("Empty {:?}", RawMessage::empty());
    println!("bxcan Data {:?}", Data::from(frame.message));
}

