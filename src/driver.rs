//! # CANAerospace - Driver
//!
//! CANAerospace requires a driver to interract with CAN hardware

use crate::{message::{CANAerospaceFrame}};

/// CANAerospaceDriver trait is act like a gate to hardware for CANAerospaceLite
pub trait CANAerospaceDriver {
    /// Takes [CANAerospaceFrame] to send it using the hardware
    fn send_frame(&mut self, frame: CANAerospaceFrame);
    /// Returns Option<[CANAerospaceFrame]> if the value is None then no action will be taken.
    /// if the value is present then frame will be handled by [crate::CANAerospaceLite].
    fn recv_frame(&mut self) -> Option<CANAerospaceFrame>;
}
