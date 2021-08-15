#![no_std]

use driver::CANAerospaceDriver;
use handler::CANAerospaceHandler;
use message::{CANAerospaceFrame, Payload, RawMessage};
use types::{DataType, MessageCode, MessageType, ServiceCode};

use crate::message::CANAerospaceMessage;

pub mod types;
pub mod message;
pub mod driver;
pub mod handler;
// #[cfg(feature = "stm32f1xx")]
pub mod bxcan;



#[derive(Debug)]
pub struct CANAerospaceLite<D,H> where
    D: CANAerospaceDriver,
    H: CANAerospaceHandler {
    pub node_id:u8,
    pub nod_count: u8,
    pub driver: D,
    pub(crate) handler: Option<H>,
}



impl<D, H> CANAerospaceLite<D, H> where
    D: CANAerospaceDriver,
    H: CANAerospaceHandler {

    pub fn new(node_id: u8, driver: D) -> Self { Self { node_id, nod_count: 0, driver, handler: None } }

    pub fn set_handler(&mut self, handler: H) {
        self.handler = Some(handler);
    }

    pub fn get_handler(&mut self) -> Option<&mut H> {
        self.handler.as_mut()
    }

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

    pub fn send_service_response(&mut self, message :CANAerospaceMessage) {
        self.driver.send_frame(CANAerospaceFrame::from(message))
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

    pub fn receive(&mut self) -> Option<CANAerospaceFrame> {
        self.driver.recv_frame()
    }

    pub fn notify_receive_event(&mut self) {
        // call driver to receive the frame
        if let Some(handler) = &mut self.handler {
            if let Some(frame) = self.driver.recv_frame() {
                let message = CANAerospaceMessage::from(frame);
                match message.message_type {
                    MessageType::NSH(message_type) | MessageType::NSL(message_type) => {
                        let service_code = message.service_code;
                        let message_code = message.message_code;

                        let (code, data) = handler.handle_service_request(message);

                        if !data.is_none() {
                            let msg_code = code.unwrap_or(message_code);
                            let response = CANAerospaceMessage {
                                message_type: MessageType::from(message_type + 1),
                                node_id: self.node_id,
                                service_code: service_code,
                                message_code: msg_code,
                                data: data.unwrap(),
                            };
                            self.send_service_response(response);
                        }
                    },
                    MessageType::EED(_) | MessageType::UDH(_) |
                    MessageType::NOD(_) | MessageType::UDL(_) |
                    MessageType::DSD(_) => {
                        handler.handle_messages(message);
                    },
                    MessageType::INVALID => todo!(),
                }
            }
        }
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
