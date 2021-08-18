// #[cfg(feature = "bxcan-support")]
#[cfg(test)]
mod frame {
    use bxcan::{Frame, Id, StandardId};

    use crate::{
        message::{CANAerospaceFrame, Payload, RawMessage},
        types::{DataType, MessageType},
    };

    #[test]
    fn test_from_canaerospaceframe() {
        // let f = Frame::from(&CANAerospaceFrame {
        //     message_type: MessageType::UDL(1801),
        //     message: RawMessage {
        //         node_id: 0x0,
        //         data_type: 0x0,
        //         service_code: 0x3,
        //         message_code: 0x0,
        //         payload: Payload::from(DataType::ACHAR(0x55).to_be_bytes()),
        //     },
        // });
        // assert!(matches!(f.id(), Id::Standard(_)));
        // if let Id::Standard(id) = f.id() {
        //     assert_eq!(id.as_raw(), MessageType::UDL(1801).id());
        // }
    }
}
