
use crate::{message::{CANAerospaceFrame}};

pub trait CANAerospaceDriver {
    fn send_frame(&mut self, frame: CANAerospaceFrame);
    fn recv_frame(&mut self) -> Option<CANAerospaceFrame>;
}
