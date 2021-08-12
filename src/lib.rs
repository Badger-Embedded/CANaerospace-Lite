use types::{DataType, MessageCode, MessageType, ServiceCodeType};

pub mod types;
pub mod message;


pub type ServiceRequestCallback = fn(CANAerospaceLite, MessageType, ServiceCodeType, MessageCode, DataType);

#[derive(Debug)]
pub struct CANAerospaceLite {
    node_id:u8,
    nod_count: u8,
    req_callback: ServiceRequestCallback
}

impl CANAerospaceLite {
    pub fn new(node_id: u8, req_callback: ServiceRequestCallback) -> Self { Self { node_id, nod_count: 0, req_callback } }

    //// EED messages
    pub fn send_urgent_message(msg_type: MessageType, error_code: i16, operation_id: u8, location_id: u8) {
        todo!("Set data type to ERROR")
    }

    //// NOD messages
    //// Message Code will be incremented by 1 for each NOD messages.
    pub fn send_normal_message(msg_type: MessageType, data: DataType) {

        todo!("Set service code to 0");
        // todo!("Increment message code by 1");
    }

    pub fn send_extended_normal_message(msg_type: MessageType, data: DataType, ext_code: u8) {
        todo!("Not implemented yet!");
    }

    //// NSH and NSL messages
    pub fn send_service_request(target_id: u8, msg_type: MessageType, data: DataType) {

    }

    pub fn send_service_response(target_id: u8, msg_type: MessageType, data: DataType) {

    }

    
    pub fn send_extended_service_request(target_id: u8, msg_type: MessageType, data: DataType, ext_code: u8) {
        todo!("Not implemented yet!");
    }

    pub fn send_custom_message(msg_type: MessageType, data: DataType) {
        todo!("Not implemented yet!");
    }

    pub fn send_debug_message(msg_type: MessageType, data: DataType) {
        todo!("Not implemented yet!");
    }

}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
