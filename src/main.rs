
//#![no_std]

use can_aerospace_lite::{CANAerospaceLite, types::{DataType, MessageCode, MessageType, ServiceCodeType}};


fn service_req_handler(canas_lite: CANAerospaceLite, msg_type: MessageType, service_code: ServiceCodeType, msg_code: MessageCode, data_type: DataType) {
    println!("New service request!")
}

fn main() {
    let canas_lite = CANAerospaceLite::new(11, service_req_handler);
    println!("Hello CANAerospaceLite {:?}", canas_lite);
}
