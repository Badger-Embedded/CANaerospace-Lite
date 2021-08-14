use crate::types::{DataType, MessageCode, MessageType, ServiceCodeType};


#[derive(Clone, Debug)]
pub struct CANAerospaceFrame {
    pub message_type: MessageType,
    pub message: RawMessage
}

#[derive(Clone, Debug)]
pub struct RawMessage {
    pub node_id: u8,
    pub data_type: u8,
    pub service_code: u8,
    pub message_code: u8,
    pub payload: Payload
}

impl RawMessage {
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
            payload
        })
    }
    pub fn empty() -> Self {
        let payload = Payload::new(&[]).unwrap();
        Self {
            node_id: 0,
            data_type: 0,
            service_code: 0,
            message_code: 0,
            payload
        }
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

#[derive(Clone, Debug)]
pub struct Payload {
    pub len: u8,
    pub data: [u8; 4]
}

impl Payload {
    pub fn new(arr: &[u8]) -> Option<Self> {
        let mut data = [0 as u8; 4];
        data[..arr.len()].copy_from_slice(&arr);
        Some(Self {
            len: arr.len() as u8,
            data
        })
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
payload_from_array!(0,1,2,3,4);
