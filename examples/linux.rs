use std::{
    thread,
    time::{Duration, Instant},
};

use can_aerospace_lite::{
    driver::CANAerospaceDriver,
    message::{CANAerospaceFrame, CANAerospaceMessage, RawMessage},
    types::{DataType, MessageType},
    CANAerospaceLite,
};
use socketcan::{CANFrame, CANSocket};

struct CANDriverLinux {
    socket: CANSocket,
}

impl CANAerospaceDriver for CANDriverLinux {
    fn send_frame(&mut self, frame: CANAerospaceFrame) {
        let can_frame = from_canas_to_socketcan(frame);
        match self.socket.write_frame(&can_frame) {
            Ok(_) => {
                println!("CAN frame({}) sent.", can_frame.id())
            }
            Err(e) => {
                println!("Unable to send CAN frame. {:?}", e);
            }
        }
    }

    fn recv_frame(&mut self) -> Option<CANAerospaceFrame> {
        print!("Reading... ");
        match self.socket.read_frame() {
            Ok(frame) => {
                let message_type = MessageType::from(frame.id() as u16);
                let aero_frame = CANAerospaceFrame {
                    message_type: message_type,
                    message: RawMessage::new(frame.data()).unwrap(),
                };
                println!(
                    "Frame {:?} Data: {:#X?}",
                    aero_frame.message_type, aero_frame.message
                );
                Some(aero_frame)
            }
            Err(e) => {
                println!("Error occured: {:?}", e);
                None
            }
        }
    }
}

fn from_canas_to_socketcan(aero_frame: CANAerospaceFrame) -> CANFrame {
    let raw_message: [u8; 8] = aero_frame.message.into();
    CANFrame::new(
        aero_frame.message_type.id().into(),
        &raw_message,
        false,
        false,
    )
    .unwrap()
}

fn main() {
    let socket = CANSocket::open("vcan0").unwrap();
    socket.set_nonblocking(true).unwrap();
    let driver = CANDriverLinux { socket: socket };
    let mut can_aero = CANAerospaceLite::new(0xA, driver);
    can_aero.send_message(CANAerospaceMessage::new(
        MessageType::EED(0x0),
        0xA,
        0x0,
        0x0,
        DataType::ULONG(0xDEAD_BEEF),
    ));
    let instant = Instant::now();
    let mut delta = Duration::ZERO;
    // Use `cangen vcan0 -g 1000 -I 80 -L 8 -D 0A000000DEADBEEF` to generate can messages
    // This is a basic IDS request
    loop {
        if delta >= Duration::from_secs(10) {
            break;
        }
        can_aero.notify_receive_event();
        thread::sleep(Duration::from_secs(1));
        delta = instant.elapsed();
    }
}
