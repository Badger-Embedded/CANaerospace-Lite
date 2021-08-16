use can_aerospace_lite::{CANAerospaceLite, driver::CANAerospaceDriver, message::{CANAerospaceFrame, RawMessage}, types::{DataType, HardwareRevision, MessageType, SoftwareRevision}};

static mut COUNT: u32 = 0;

struct CANDriver;

impl CANAerospaceDriver for CANDriver {
    fn send_frame(&mut self, frame: CANAerospaceFrame) {
        println!("Send frame: {:#X?}", frame);
    }

    fn recv_frame(&mut self) -> Option<CANAerospaceFrame> {
        unsafe {
            return match COUNT {
                1 => Some(CANAerospaceFrame {
                    message_type: MessageType::NSH(128),
                    message: RawMessage::from([10, DataType::ULONG(0).type_id(), 2, 3, 0xBA, 0xBA, 0xDE, 0xDE]),
                }),
                0 => Some(CANAerospaceFrame {
                    message_type: MessageType::NOD(300),
                    message: RawMessage::from([0, DataType::BSHORT2(0,0).type_id(), 2, 3, 0xDE, 0xDE, 0xDE, 0xDE]),
                }),
                2 => Some(CANAerospaceFrame {
                    message_type: MessageType::UDH(200),
                    message: RawMessage::from([0, DataType::BSHORT2(0,0).type_id(), 2, 3, 0xA, 0xB, 0xC, 0xD]),
                }),
                3 => Some(CANAerospaceFrame {
                    message_type: MessageType::NSH(128),
                    message: RawMessage::from([10, DataType::NODATA.type_id(), 0, 3, 0xFB, 0xFB, 0xDE, 0xDE]),
                }),
                _ => None
            }
        }
    }
}

fn main() {
    let mut can_aerospace: CANAerospaceLite<CANDriver> = CANAerospaceLite::new(10, CANDriver{});
    can_aerospace.set_hw_revision(HardwareRevision(0x02));
    can_aerospace.set_sw_revision(SoftwareRevision(0x01));

    can_aerospace.notify_receive_event();
    unsafe { COUNT += 1; }
    can_aerospace.notify_receive_event();
    unsafe { COUNT += 1; }
    can_aerospace.notify_receive_event();
    unsafe { COUNT += 1; }
    can_aerospace.notify_receive_event();
    can_aerospace.notify_receive_event();

    let mut message = can_aerospace.read_message().unwrap();
    assert_eq!(message.message_type.id(), MessageType::NOD(300).id());
    println!("Message {:#X?}", message);
    message.message_type = MessageType::NSH(message.message_type.id() + 1);
    message.data = DataType::ULONG(0xDEAD_BEEF);
    can_aerospace.send_message(message);

    message = can_aerospace.read_message().unwrap();
    println!("Message {:#X?}", message);
    message = can_aerospace.read_message().unwrap();
    println!("Message {:#X?}", message);

}
