use bxcan::Data;
use can_aerospace_lite::{message::{CANAerospaceFrame, RawMessage}, types::{DataType, MessageType, ServiceCodeEnum}};

fn main() {
    let raw_data: [u8; 8] = [0xA, DataType::BSHORT2(0,0).type_id(), 0x0, 0x0,    0xAD, 0xDE, 0xEF, 0xBE];
    let raw_message = RawMessage::from(raw_data);
    let frame = CANAerospaceFrame{
        message_type: MessageType::EED(10),
        message: raw_message
    };
    let data = Data::from(raw_data);
    let a = data[0];
    let data_type = DataType::from((data[1], &data[4..data.len()]));
    println!("Filled {:?}", frame);
    println!("bxcan Data {:#X?}", data);
    println!("DataType {:#X?}",  data_type);
    println!("bxcan re-Data {:#X?}", Data::from(data_type.to_be_bytes()));
    println!("enum {:?}", ServiceCodeEnum::IDS as i32)
}

