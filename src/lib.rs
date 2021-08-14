#![no_std]

use driver::CANAerospaceDriver;
use types::{DataType, MessageCode, MessageType, ServiceCodeType};

pub mod types;
pub mod message;
pub mod driver;
// #[cfg(feature = "stm32f1xx")]
pub mod bxcan;

const CAN_AEROSPACE_MESSAGE_HEADER_LEN: u8 = 4;


pub trait CANAerospaceCallbackHandler {
    fn handle_service_request<D: CANAerospaceDriver, H: CANAerospaceCallbackHandler>(&self, canas_lite: CANAerospaceLite<D, H>, 
                    message_type: MessageType, service_code: ServiceCodeType, msg_code: MessageCode, data: DataType);
}

#[derive(Debug)]
pub struct CANAerospaceLite<D,H> where
    D: CANAerospaceDriver,
    H: CANAerospaceCallbackHandler {
    pub node_id:u8,
    pub nod_count: u8,
    pub driver: D,
    pub callback_handler: H
}



impl<D, H> CANAerospaceLite<D, H> where
    D: CANAerospaceDriver,
    H: CANAerospaceCallbackHandler {

    pub fn new(node_id: u8, driver: D, callback_handler: H) -> Self { Self { node_id, nod_count: 0, driver, callback_handler } }

    /// EED messages
    pub fn send_urgent_message(&self, msg_type: MessageType, error_code: i16, operation_id: u8, location_id: u8) {
        todo!("Set data type to ERROR")
    }

    /// NOD messages
    /// Message Code will be incremented by 1 for each NOD messages.
    pub fn send_normal_message(&self, msg_type: MessageType, data: DataType) {

        todo!("Set service code to 0");
        // todo!("Increment message code by 1");
    }

    pub fn send_extended_normal_message(&self, msg_type: MessageType, data: DataType, ext_code: u8) {
        todo!("Not implemented yet!");
    }

    /// NSH and NSL messages
    pub fn send_service_request(&self, target_id: u8, msg_type: MessageType, data: DataType) {

    }

    pub fn send_service_response(&self, target_id: u8, msg_type: MessageType, data: DataType) {

    }

    
    pub fn send_extended_service_request(&self, target_id: u8, msg_type: MessageType, data: DataType, ext_code: u8) {
        todo!("Not implemented yet!");
    }

    pub fn send_custom_message(&self, msg_type: MessageType, data: DataType) {
        todo!("Not implemented yet!");
    }

    pub fn send_debug_message(&self, msg_type: MessageType, data: DataType) {
        todo!("Not implemented yet!");
    }

    pub fn notify_receive_event(&self) {
        // call driver to receive the frame
        todo!("Not implemented yet!");
    }

    pub fn notify_transmit_event(&self) {
        todo!("Not implemented yet!")
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
