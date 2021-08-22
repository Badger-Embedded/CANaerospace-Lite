// #[cfg(feature = "bxcan-support")]
#[cfg(test)]
mod frame {
    use bxcan::{Frame, Id, StandardId};

    use crate::{
        message::{CANAerospaceFrame, Payload, RawMessage},
        types::{DataType, MessageType, ServiceCodeEnum},
    };

    #[test]
    fn test_into_canaerospaceframe() {
        let f = CANAerospaceFrame::from(Frame::new_data(
            StandardId::new(MessageType::NSH(128).id()).unwrap(),
            [
                0xA,
                DataType::ERROR(0).type_id(),
                ServiceCodeEnum::UNKNOWN as u8,
                0x20,
                0xDE,
                0xAD,
                0xBE,
                0xEF,
            ],
        ));

        assert!(matches!(f.message_type, MessageType::NSH(128)));
        assert_eq!(f.message.node_id, 0xA);
        assert_eq!(f.message.data_type, DataType::ERROR(0).type_id());
        assert_eq!(f.message.service_code, ServiceCodeEnum::UNKNOWN as u8);
        assert_eq!(f.message.message_code, 0x20);
        assert_eq!(
            f.message.payload.data,
            DataType::ERROR(0xDEADBEEF).to_be_bytes()
        );
    }

    #[test]
    fn test_from_canaerospaceframe() {
        let f = Frame::from(&CANAerospaceFrame {
            message_type: MessageType::UDL(1801),
            message: RawMessage {
                node_id: 0x0,
                data_type: 0x0,
                service_code: 0x3,
                message_code: 0x0,
                payload: Payload::from(DataType::ACHAR(0x55).to_be_bytes()),
            },
        });
        assert!(matches!(f.id(), Id::Standard(_)));
        if let Id::Standard(id) = f.id() {
            assert_eq!(id.as_raw(), MessageType::UDL(1801).id());
        }
    }
}

#[cfg(test)]
mod data {
    use bxcan::Data;

    use crate::{
        message::{Payload, RawMessage},
        types::{DataType, ServiceCodeEnum},
    };

    #[test]
    fn test_from_rawmessage() {
        let data = Data::from(&RawMessage {
            node_id: 0xBB,
            data_type: DataType::ERROR(0xDEADDEAD).type_id(),
            service_code: ServiceCodeEnum::NSS as u8,
            message_code: 0xFD,
            payload: Payload::from(DataType::ERROR(0xDEADDEAD).to_be_bytes()),
        });
        let p = DataType::ERROR(0xDEADDEAD).to_be_bytes();
        assert_eq!(data[0], 0xBB);
        assert_eq!(data[1], DataType::ERROR(0xDEADDEAD).type_id());
        assert_eq!(data[2], ServiceCodeEnum::NSS as u8);
        assert_eq!(data[3], 0xFD);
        assert_eq!(data[4], p[0]);
        assert_eq!(data[5], p[1]);
        assert_eq!(data[6], p[2]);
        assert_eq!(data[7], p[3]);
    }
}
