#![no_std]

use driver::CANAerospaceDriver;
use heapless::{BinaryHeap, binary_heap::{Min}};
use message::{CANAerospaceFrame};

use crate::message::CANAerospaceMessage;

pub mod types;
pub mod message;
pub mod driver;
#[cfg(feature = "stm32f1xx")]
pub mod bxcan;



#[derive(Debug)]
pub struct CANAerospaceLite<D> where
    D: CANAerospaceDriver {
    pub node_id:u8,
    pub nod_count: u8,
    pub driver: D,
    pub(crate) rx_queue: BinaryHeap<CANAerospaceFrame, Min, 10>
}



impl<D> CANAerospaceLite<D> where
    D: CANAerospaceDriver {

    pub fn new(node_id: u8, driver: D) -> Self { Self { node_id, nod_count: 0, driver, rx_queue: BinaryHeap::new() } }

    pub fn send_message(&mut self, message: CANAerospaceMessage) {
        self.driver.send_frame(CANAerospaceFrame::from(message));
    }

    pub fn read_message(&mut self) -> Option<CANAerospaceMessage> {
        if let Some(frame) = self.rx_queue.pop() {
            Some(CANAerospaceMessage::from(frame))
        } else {
            None
        }
    }

    pub fn notify_receive_event(&mut self) {
        if let Some(frame) = self.driver.recv_frame() {
            self.rx_queue.push(frame).unwrap();
        }
        // TODO: handle ids service request automatically
        // call driver to receive the frame
        // if let Some(handler) = &mut self.handler {
        //     if let Some(frame) = self.driver.recv_frame() {
        //         let message = CANAerospaceMessage::from(frame);
        //         match message.message_type {
        //             MessageType::NSH(message_type) | MessageType::NSL(message_type) => {
        //                 let service_code = message.service_code;
        //                 let message_code = message.message_code;

        //                 let (code, data) = handler.handle_service_request(message);

        //                 if !data.is_none() {
        //                     let msg_code = code.unwrap_or(message_code);
        //                     let response = CANAerospaceMessage {
        //                         message_type: MessageType::from(message_type + 1),
        //                         node_id: self.node_id,
        //                         service_code: service_code,
        //                         message_code: msg_code,
        //                         data: data.unwrap(),
        //                     };
        //                     self.send_service_response(response);
        //                 }
        //             },
        //             MessageType::EED(_) | MessageType::UDH(_) |
        //             MessageType::NOD(_) | MessageType::UDL(_) |
        //             MessageType::DSD(_) => {
        //                 handler.handle_messages(message);
        //             },
        //             MessageType::INVALID => todo!(),
        //         }
        //     }
        // }
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
