use bxcan::{Data, Frame, Id, StandardId};

use crate::{message::{CANAerospaceFrame, RawMessage}, types::MessageType};

impl From<Frame> for CANAerospaceFrame {
    fn from(frame: Frame) -> Self {
        let raw_id = match frame.id() {
            Id::Standard(id) => id.as_raw() as u32,
            Id::Extended(id) => id.as_raw() as u32
        };
        let message: RawMessage = match frame.data() {
            Some(data) => RawMessage::new(data).unwrap(),
            None => RawMessage::empty(),
        };
        let frame_type = MessageType::from(raw_id as u16);
        Self {
            frame_type,
            message
        }
    }
}

impl From<CANAerospaceFrame> for Frame {
    fn from(canas_frame: CANAerospaceFrame) -> Self {
        let id = StandardId::new(canas_frame.frame_type.id()).unwrap_or(StandardId::MAX);
        Frame::new_data(id, Data::from(canas_frame.message))
    }
}

impl From<RawMessage> for Data {
    fn from(message: RawMessage) -> Self {
        // TODO: Special treatment can be done in future to reduce data size for some message types
        let mut bytes = [0u8; 8];
        bytes[0] = message.node_id;
        bytes[1] = message.data_type;
        bytes[2] = message.service_code;
        bytes[3] = message.message_code;
        bytes[4..].copy_from_slice(&message.payload.data);
        Data::new(&bytes).unwrap_or(Data::empty())        
    }
}