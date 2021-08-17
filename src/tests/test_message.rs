#[cfg(test)]
mod canaerospacemessage {
    use crate::{
        message::{CANAerospaceFrame, CANAerospaceMessage, Payload, RawMessage},
        types::{DataType, MessageType, ServiceCodeEnum},
    };

    #[test]
    fn test_new() {
        let message =
            CANAerospaceMessage::new(MessageType::EED(100), 0xD, 0xFF, 0xA, DataType::FLOAT(2.5));
        assert_eq!(message.message_type, MessageType::EED(100));
        assert_eq!(message.node_id, 0xD);
        assert_eq!(message.service_code as u8, 0xFF);
        assert_eq!(message.message_code, 0xA);
        assert_eq!(message.data, DataType::FLOAT(2.5));
        assert_eq!(message.data.to_be_bytes(), 2.5f32.to_be_bytes());
    }

    #[test]
    fn test_from_frame() {
        let message = CANAerospaceMessage::from(CANAerospaceFrame {
            message_type: MessageType::INVALID,
            message: RawMessage {
                node_id: 0xFB,
                data_type: DataType::ACHAR4(0x1, 0x2, 0x3, 0x4).type_id(),
                service_code: 0xAA,
                message_code: 0xCC,
                payload: Payload::from(DataType::ACHAR4(0x1, 0x2, 0x3, 0x4).to_be_bytes()),
            },
        });
        assert_eq!(message.message_type, MessageType::INVALID);
        assert_eq!(message.node_id, 0xFB);
        assert_eq!(
            message.data.type_id(),
            DataType::ACHAR4(0, 0, 0, 0).type_id()
        );
        assert_eq!(message.data, DataType::ACHAR4(0x1, 0x2, 0x3, 0x4));
        assert_eq!(message.service_code, ServiceCodeEnum::UNKNOWN);
        assert_eq!(message.message_code, 0xCC);
    }
}
