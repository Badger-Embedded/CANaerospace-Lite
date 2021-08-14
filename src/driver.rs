use crate::{message::{CANAerospaceFrame}, types::MessageType};

pub trait CANAerospaceDriver {
    fn send_frame(&self, frame: CANAerospaceFrame);
    fn recv_frame(&self) -> CANAerospaceFrame;
}
