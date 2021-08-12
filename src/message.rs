use crate::types::{DataType, MessageType};


#[derive(Debug)]
struct RawMessage<'a> {
    pub node_id: u8,
    pub data_type: u8,
    pub service_code: u8,
    pub message_code: u8,
    payload: &'a [u8]
}
