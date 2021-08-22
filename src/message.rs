//! # CANAeropsace - Message
//! All wrappers of CANAerospace is defined in this module.
use core::cmp::Ordering;

use crate::types::{DataType, MessageCode, MessageType, NodeId, ServiceCode, ServiceCodeEnum};

/// Higher level abstraction of [CANAerospaceFrame].
///
/// Constructable from [CANAerospaceFrame] by using [From] trait.
#[derive(Clone, Debug)]
pub struct CANAerospaceMessage {
    /// Type of the message, typically CAN identifier of the message
    pub message_type: MessageType,

    /// If `message_type` is one of [MessageType::NSH] or [MessageType::NSL]
    /// then it indicates the target, otherwise it indicates the sender node
    pub node_id: NodeId,

    /// Indicates service code for service messages
    ///
    /// [ServiceCodeEnum::IDS] service code will be automatically replied by [crate::CANAerospaceLite]
    pub service_code: ServiceCodeEnum,

    /// Sequence number of message
    pub message_code: MessageCode,

    /// Payload of the message
    pub data: DataType,
}

impl CANAerospaceMessage {
    /// Creates new instance of [CANAerospaceMessage]
    pub fn new(
        message_type: MessageType,
        node_id: NodeId,
        service_code: ServiceCode,
        message_code: MessageCode,
        data: DataType,
    ) -> Self {
        Self {
            message_type,
            node_id,
            service_code: ServiceCodeEnum::from(service_code),
            message_code,
            data,
        }
    }
}

impl From<CANAerospaceFrame> for CANAerospaceMessage {
    fn from(frame: CANAerospaceFrame) -> Self {
        Self {
            message_type: frame.message_type,
            node_id: frame.message.node_id,
            service_code: ServiceCodeEnum::from(frame.message.service_code),
            message_code: frame.message.message_code,
            data: DataType::from((frame.message.data_type, &frame.message.payload.data[..])),
        }
    }
}

/// Higher level abstraction of [RawMessage]
///
/// Takes [RawMessage] as `message` and appends `message_type` information.
/// Can be ordered by `message_type` and constructable from [CANAerospaceMessage] by using [From] trait.
#[derive(Clone, Debug)]
pub struct CANAerospaceFrame {
    /// Type of the message, typically CAN identifier of the message
    pub message_type: MessageType,

    /// Raw message information
    pub message: RawMessage,
}

impl Ord for CANAerospaceFrame {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.message_type.id().cmp(&other.message_type.id())
    }
}

impl PartialOrd for CANAerospaceFrame {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CANAerospaceFrame {
    fn eq(&self, other: &Self) -> bool {
        self.message_type.id().cmp(&other.message_type.id()) == Ordering::Equal
    }
}

impl Eq for CANAerospaceFrame {}

impl From<CANAerospaceMessage> for CANAerospaceFrame {
    fn from(message: CANAerospaceMessage) -> Self {
        Self {
            message_type: message.message_type,
            message: RawMessage {
                node_id: message.node_id,
                data_type: message.data.type_id(),
                service_code: message.service_code as u8,
                message_code: message.message_code,
                payload: Payload::from(message.data.to_be_bytes()),
            },
        }
    }
}

/// Raw payload data of a CAN frame in total 8 bytes.
///
/// It contains CANAerospace message header and also a payload
/// Can be constructable from [u8] arrays with min 4 and max 8 lengths.
#[derive(Clone, Debug)]
pub struct RawMessage {
    pub node_id: u8,
    pub data_type: u8,
    pub service_code: u8,
    pub message_code: u8,
    pub payload: Payload,
}

impl RawMessage {
    /// Returns new instance of RawMessage using incoming data array
    pub fn new(data: &[u8]) -> Option<Self> {
        let node_id = data[0];
        let data_type = data[1];
        let service_code = data[2];
        let message_code = data[3];
        let payload = Payload::new(&data[4..]).unwrap();
        Some(Self {
            node_id,
            data_type,
            service_code,
            message_code,
            payload,
        })
    }

    /// Returns an all zero empty [RawMessage]
    pub fn empty() -> Self {
        let payload = Payload::new(&[]).unwrap();
        Self {
            node_id: 0,
            data_type: 0,
            service_code: 0,
            message_code: 0,
            payload,
        }
    }
}

impl From<RawMessage> for [u8; 8] {
    /// Converts [RawMessage] into an byte array with length of 8
    ///
    ///```
    /// use can_aerospace_lit::message::RawMessage;
    /// let raw = RawMessage::new(&[0,1,2,3,3,5,4,7]);
    /// let in_bytes = raw.into();
    /// assert_eq!(in_bytes[0], 0);
    /// assert_eq!(in_bytes[1], 1);
    /// assert_eq!(in_bytes[2], 2);
    /// assert_eq!(in_bytes[3], 3);
    /// assert_eq!(in_bytes[4], 3);
    /// assert_eq!(in_bytes[5], 5);
    /// assert_eq!(in_bytes[6], 4);
    /// assert_eq!(in_bytes[7], 7);
    ///```
    fn from(raw: RawMessage) -> Self {
        let mut data = [0_u8; 8];
        data[0] = raw.node_id;
        data[1] = raw.data_type;
        data[2] = raw.service_code;
        data[3] = raw.message_code;
        data[4..].copy_from_slice(&raw.payload.data);
        data
    }
}

macro_rules! raw_message_from_array {
    ( $($len:literal),+ ) => {
        $(
            impl From<[u8; $len]> for RawMessage {
                #[inline]
                fn from(arr: [u8; $len]) -> Self {
                    let node_id = arr[0];
                    let data_type = arr[1];
                    let service_code = arr[2];
                    let message_code = arr[3];
                    let payload = Payload::new(&arr[4..]).unwrap();
                    Self {
                        node_id,
                        data_type,
                        service_code,
                        message_code,
                        payload
                    }
                }
            }
        )+
    };
}

/// Raw byte representation of payload of CANAerospace message
#[derive(Clone, Debug)]
pub struct Payload {
    pub len: u8,
    pub data: [u8; 4],
}

impl Payload {
    pub fn new(arr: &[u8]) -> Option<Self> {
        let mut data = [0; 4];
        data[..arr.len()].copy_from_slice(arr);
        Some(Self {
            len: arr.len() as u8,
            data,
        })
    }
}

impl From<&DataType> for Payload {
    fn from(data: &DataType) -> Self {
        Self {
            len: 4,
            data: data.to_be_bytes(),
        }
    }
}

macro_rules! payload_from_array {
    ( $($len:literal),+ ) => {
        $(
            impl From<[u8; $len]> for Payload {
                #[inline]
                fn from(arr: [u8; $len]) -> Self {
                    let mut data = [0 as u8; 4];
                    data[..$len].copy_from_slice(&arr);
                    Self {
                        len: $len,
                        data
                    }
                }
            }
        )+
    };
}

raw_message_from_array!(4, 5, 6, 7, 8);
payload_from_array!(0, 1, 2, 3, 4);
