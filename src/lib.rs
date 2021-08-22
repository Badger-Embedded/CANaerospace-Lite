//! # CANAerospace{Lite}
//!
//! CANAerospace Lite is a lightweight can bus protocol implementation for aerospace vehicles. It comes with built-in bxCAN support
//! which makes integration with bxCAN devices easy.
//!
//! [CANAerospaceLite] is the struct that you need to use to have CANAerospace functionality. Please take a look at the usage
//! example in documentation.

#![no_std]
// #![feature(doc_cfg)]
use heapless::{binary_heap::Min, BinaryHeap};

use crate::message::{CANAerospaceFrame, CANAerospaceMessage};
use crate::types::{
    DataType, HardwareRevision, IDSConfiguration, IDSHeaderConfiguration, IDSResponse,
    ServiceCodeEnum, SoftwareRevision,
};
use crate::{driver::CANAerospaceDriver, types::MessageType};

pub mod driver;
pub mod message;
mod tests;
pub mod types;

#[cfg(not(tarpaulin_include))]
#[cfg(feature = "bxcan-support")]
// #[cfg(any(feature = "bxcan-support", doc))]
// #[doc(cfg(feature = "bxcan-support"))]
pub mod bxcan;

pub const IDS_CONF_STANDARD: IDSConfiguration = IDSConfiguration(0);
pub const IDS_MSG_HEADER_STANDARD: IDSHeaderConfiguration = 0;

/// Main struct of the library. All logic is implemented around this struct.
///
/// Must be initialized with `node_id`, `driver` which is a [CANAerospaceDriver]
///
/// # Example
///```rust
/// # use can_aerospace_lite::{CANAerospaceLite, driver::CANAerospaceDriver, message::{CANAerospaceFrame, RawMessage}, types::{DataType, HardwareRevision, MessageType, ServiceCodeEnum, SoftwareRevision}};
/// # static mut COUNT: u32 = 0;
/// struct CANDriver;
///
/// impl CANAerospaceDriver for CANDriver {
///     fn send_frame(&mut self, frame: CANAerospaceFrame) {
///        assert_eq!(frame.message_type.id(), MessageType::NSH(129).id());
///     }
///     // implementation of send/recv frame...
/// #    fn recv_frame(&mut self) -> Option<CANAerospaceFrame> {
/// #        unsafe {
/// #            return match COUNT {
/// #                1 => Some(CANAerospaceFrame {
/// #                    message_type: MessageType::NSH(128),
/// #                    message: RawMessage::from([10, DataType::ULONG(0).type_id(), 2, 3, 0xBA, 0xBA, 0xDE, 0xDE]),
/// #                }),
/// #                0 => Some(CANAerospaceFrame {
/// #                    message_type: MessageType::NOD(300),
/// #                    message: RawMessage::from([0, DataType::BSHORT2(0,0).type_id(), 2, 3, 0xDE, 0xDE, 0xDE, 0xDE]),
/// #                }),
/// #                2 => Some(CANAerospaceFrame {
/// #                    message_type: MessageType::UDH(200),
/// #                    message: RawMessage::from([0, DataType::BSHORT2(0,0).type_id(), 2, 3, 0xA, 0xB, 0xC, 0xD]),
/// #                }),
/// #                3 => Some(CANAerospaceFrame {
/// #                    message_type: MessageType::NSH(128),
/// #                    message: RawMessage::from([10, DataType::NODATA.type_id(), 0, 3, 0xFB, 0xFB, 0xDE, 0xDE]),
/// #                }),
/// #                _ => None
/// #            }
/// #        }
/// #    }
/// }
///
/// let mut can_aerospace: CANAerospaceLite<CANDriver> = CANAerospaceLite::new(10, CANDriver{});
///
/// can_aerospace.set_hw_revision(HardwareRevision(0x02));
/// can_aerospace.set_sw_revision(SoftwareRevision(0x01));
///
/// can_aerospace.notify_receive_event(); // Notify function must be called when a can message is recieved/wanted to be received.
///
/// let mut message = can_aerospace.read_message().unwrap();
/// // According to our driver we should receive NOD message
/// assert_eq!(message.message_type.id(), MessageType::NOD(300).id());
///
/// # unsafe { COUNT += 1; }
/// # can_aerospace.notify_receive_event();
/// # unsafe { COUNT += 1; }
/// # can_aerospace.notify_receive_event();
/// # unsafe { COUNT += 1; }
/// # can_aerospace.notify_receive_event();
/// # can_aerospace.notify_receive_event();
/// message = can_aerospace.read_message().unwrap();
/// assert_eq!(message.message_type.id(), MessageType::NSH(128).id());
///
/// message = can_aerospace.read_message().unwrap();
/// assert_eq!(message.message_type.id(), MessageType::UDH(200).id());
///```
///
#[derive(Debug)]
pub struct CANAerospaceLite<D>
where
    D: CANAerospaceDriver,
{
    pub node_id: u8,
    identification: IDSResponse,
    nod_count: u8,
    driver: D,
    pub(crate) rx_queue: BinaryHeap<CANAerospaceFrame, Min, 10>,
}

impl<D> CANAerospaceLite<D>
where
    D: CANAerospaceDriver,
{
    /// Creates new instance of [CANAerospaceLite].
    /// ```
    /// # use can_aerospace_lite::{CANAerospaceLite, driver::CANAerospaceDriver, message::{CANAerospaceFrame, RawMessage}, types::{DataType, HardwareRevision, MessageType, SoftwareRevision}};
    /// struct CANDriver;
    /// impl CANAerospaceDriver for CANDriver {
    /// #    fn send_frame(&mut self, frame: CANAerospaceFrame) { todo!(); }
    /// #    fn recv_frame(&mut self) -> Option<CANAerospaceFrame> { todo!(); }
    /// }
    /// let can_aerospace = CANAerospaceLite::new(0xFB, CANDriver{});
    /// assert_eq!(can_aerospace.node_id, 0xFB);
    /// ```
    pub fn new(node_id: u8, driver: D) -> Self {
        Self {
            node_id,
            identification: IDSResponse {
                hw_rev: HardwareRevision(0),
                sw_rev: SoftwareRevision(0),
                configuration: IDS_CONF_STANDARD,
                header: IDS_MSG_HEADER_STANDARD,
            },
            nod_count: 0,
            driver,
            rx_queue: BinaryHeap::new(),
        }
    }

    /// Sets hardware revision information for response of [ServiceCodeEnum::IDS] service request.
    /// # Example
    /// ```ignore
    /// can_aerospace.set_hw_revision(HardwareRevision(0x01));
    /// ```
    pub fn set_hw_revision(&mut self, rev: HardwareRevision) {
        self.identification.hw_rev = rev;
    }

    /// Sets software revision information for response of [ServiceCodeEnum::IDS] service request.
    /// # Example
    /// ```ignore
    /// can_aerospace.set_sw_revision(SoftwareRevision(0x01));
    /// ```
    pub fn set_sw_revision(&mut self, rev: SoftwareRevision) {
        self.identification.sw_rev = rev;
    }

    /// Sets configuration information for response of [ServiceCodeEnum::IDS] service request.
    /// # Example
    /// ```ignore
    /// can_aerospace.set_ids_configuration(IDS_CONF_STANDARD);
    /// ```
    pub fn set_ids_configuration(&mut self, conf: IDSConfiguration) {
        self.identification.configuration = conf;
    }

    /// Sets message header information for response of [ServiceCodeEnum::IDS] service request.
    /// # Example
    /// ```ignore
    /// can_aerospace.set_message_header_conf(IDS_MSG_HEADER_STANDARD);
    /// ```
    pub fn set_message_header_conf(&mut self, conf: IDSHeaderConfiguration) {
        self.identification.header = conf;
    }

    /// Sends a CAN message using driver.
    /// # Example
    /// ```ignore
    /// let m = CANAerospaceMessage {
    ///     message_type: MessageType::NOD(300),
    ///     node_id: can_aerospace.node_id, // personal id for NOD messages
    ///     service_code: ServiceCodeEnum::UNKNOWN,
    ///     message_code: 0,
    ///     data: DataType::ULONG(0xDEAD_BEEF),
    /// };
    /// can_aerospace.send_message(m);
    /// ```
    pub fn send_message(&mut self, message: CANAerospaceMessage) {
        // TODO: overwrite node id of the messages according to message type
        self.driver.send_frame(CANAerospaceFrame::from(message));
    }

    /// Reads a CAN message using driver.
    /// Be aware that it will return None if there is no message
    /// # Example
    /// ```ignore
    /// if let Some(m) = can_aerospace.read_message() {
    ///     // do stuff
    /// }
    /// ```
    pub fn read_message(&mut self) -> Option<CANAerospaceMessage> {
        self.rx_queue.pop().map(CANAerospaceMessage::from)
    }

    /// Notifies CANAerospace regarding the arrival of new CAN frame.
    /// This function must be called each time when a new message comes.
    pub fn notify_receive_event(&mut self) {
        if let Some(frame) = self.driver.recv_frame() {
            match frame.message_type {
                types::MessageType::NSH(id) | types::MessageType::NSL(id) => {
                    if id % 2 == 0 {
                        // Event ids are requests
                        self.handle_service_request(frame);
                    } else {
                        self.rx_queue.push(frame).unwrap_or(());
                    }
                }
                types::MessageType::INVALID => { /* do nothing */ }
                _ => {
                    self.rx_queue.push(frame).unwrap_or(());
                }
            };
        }
    }

    /// Handles all the service requests and filters them according to `node_id`
    fn handle_service_request(&mut self, frame: CANAerospaceFrame) {
        if frame.message.node_id == self.node_id || frame.message.node_id == 0 {
            match ServiceCodeEnum::from(frame.message.service_code) {
                ServiceCodeEnum::IDS => {
                    let response_message_type = match frame.message_type {
                        MessageType::NSH(id) => MessageType::NSH(id + 1),
                        MessageType::NSL(id) => MessageType::NSL(id + 1),
                        _ => frame.message_type,
                    };
                    let message = CANAerospaceMessage {
                        message_type: response_message_type,
                        node_id: self.node_id,
                        service_code: ServiceCodeEnum::IDS,
                        message_code: frame.message.message_code,
                        data: DataType::UCHAR4(
                            self.identification.hw_rev.0,
                            self.identification.sw_rev.0,
                            self.identification.configuration.0,
                            self.identification.header,
                        ),
                    };
                    self.send_message(message);
                }
                _ => {
                    self.rx_queue.push(frame).unwrap_or(());
                }
            };
        }
    }
}
