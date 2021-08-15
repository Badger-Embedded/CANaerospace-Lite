use bxcan::Data;
use can_aerospace_lite::{CANAerospaceLite, driver::CANAerospaceDriver, handler::CANAerospaceHandler, message::{CANAerospaceFrame, CANAerospaceMessage, RawMessage}, types::{DataType, MessageType, ServiceCodeEnum}};

static mut COUNT: u32 = 0;

struct CANDriver;

impl CANAerospaceDriver for CANDriver {
    fn send_frame(&mut self, frame: CANAerospaceFrame) {
        println!("Send frame: {:#X?}", frame);
    }

    fn recv_frame(&mut self) -> Option<CANAerospaceFrame> {
        unsafe {
            return match COUNT {
                0 => Some(CANAerospaceFrame {
                    message_type: MessageType::NSH(128),
                    message: RawMessage::from([0, DataType::ULONG(0).type_id(), 2, 3, 0xBA, 0xBA, 0xDE, 0xDE]),
                }),
                1 => Some(CANAerospaceFrame {
                    message_type: MessageType::NOD(300),
                    message: RawMessage::from([0, DataType::BSHORT2(0,0).type_id(), 2, 3, 0xDE, 0xDE, 0xDE, 0xDE]),
                }),
                2 => Some(CANAerospaceFrame {
                    message_type: MessageType::UDH(200),
                    message: RawMessage::from([0, DataType::BSHORT2(0,0).type_id(), 2, 3, 0xA, 0xB, 0xC, 0xD]),
                }),
                _ => None
            }
        }
    }
}

struct CANHandler;

impl CANAerospaceHandler for CANHandler {
    fn handle_service_request(&mut self, message: CANAerospaceMessage) -> (Option<can_aerospace_lite::types::MessageCode>, Option<DataType>) {
        println!("handle_service_request: Type: {:#X?}", message);

        if let MessageType::NSH(_) = message.message_type {
            return (Some(0xF), Some(DataType::ULONG(0xDEAD_BEEF)));
        }

        (None, None)
    }

    fn handle_messages(&mut self, message: CANAerospaceMessage) {
        println!("handle_messages: Type: {:#X?}", message);
    }
}

fn main() {
    let mut can_aerospace: CANAerospaceLite<CANDriver, CANHandler> = CANAerospaceLite::new(10, CANDriver{});
    can_aerospace.set_handler(CANHandler{});
    can_aerospace.notify_receive_event();
    unsafe { COUNT += 1; }
    can_aerospace.notify_receive_event();
    unsafe { COUNT += 1; }
    can_aerospace.notify_receive_event();
}
