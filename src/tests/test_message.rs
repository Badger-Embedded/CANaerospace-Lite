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
        assert_eq!(message.service_code.as_u8(), 0xFF);
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
        assert_eq!(message.service_code, ServiceCodeEnum::CUSTOM(0xAA));
        assert_eq!(message.message_code, 0xCC);
    }
}

#[cfg(test)]
mod canaerospaceframe {
    use core::cmp::Ordering;

    use crate::{
        message::{CANAerospaceFrame, CANAerospaceMessage, RawMessage},
        types::{DataType, MessageType, ServiceCodeEnum},
    };

    #[test]
    fn test_cmp() {
        let frame_1 = CANAerospaceFrame {
            message_type: MessageType::UDH(297),
            message: RawMessage::empty(),
        };
        let frame_2 = CANAerospaceFrame {
            message_type: MessageType::UDL(1809),
            message: RawMessage::empty(),
        };

        let frame_3 = CANAerospaceFrame {
            message_type: MessageType::UDH(297),
            message: RawMessage::empty(),
        };
        assert_eq!(frame_1.cmp(&frame_2), Ordering::Less);
        assert_eq!(frame_2.cmp(&frame_1), Ordering::Greater);
        assert_eq!(frame_1.cmp(&frame_3), Ordering::Equal);
        assert_eq!(frame_3.cmp(&frame_1), Ordering::Equal);
    }

    #[test]
    fn test_partial_cmp() {
        let frame_1 = CANAerospaceFrame {
            message_type: MessageType::UDH(297),
            message: RawMessage::empty(),
        };
        let frame_2 = CANAerospaceFrame {
            message_type: MessageType::UDL(1809),
            message: RawMessage::empty(),
        };

        let frame_3 = CANAerospaceFrame {
            message_type: MessageType::UDH(297),
            message: RawMessage::empty(),
        };
        assert_eq!(frame_1.partial_cmp(&frame_2), Some(Ordering::Less));
        assert_eq!(frame_2.partial_cmp(&frame_1), Some(Ordering::Greater));
        assert_eq!(frame_1.partial_cmp(&frame_3), Some(Ordering::Equal));
        assert_eq!(frame_3.partial_cmp(&frame_1), Some(Ordering::Equal));
    }

    #[test]
    fn test_eq() {
        let frame_1 = CANAerospaceFrame {
            message_type: MessageType::UDH(297),
            message: RawMessage::empty(),
        };
        let frame_2 = CANAerospaceFrame {
            message_type: MessageType::UDL(1809),
            message: RawMessage::empty(),
        };

        let frame_3 = CANAerospaceFrame {
            message_type: MessageType::UDH(297),
            message: RawMessage::empty(),
        };
        assert_eq!(frame_1.eq(&frame_2), false);
        assert_eq!(frame_2.eq(&frame_1), false);
        assert_eq!(frame_1.eq(&frame_3), true);
        assert_eq!(frame_3.eq(&frame_1), true);
    }

    #[test]
    fn test_from_message() {
        let frame = CANAerospaceFrame::from(CANAerospaceMessage {
            message_type: MessageType::NOD(305),
            node_id: 0xAC,
            service_code: ServiceCodeEnum::UNKNOWN,
            message_code: 0x1,
            data: DataType::ULONG(0xDEAD_BEEF),
        });

        assert_eq!(frame.message_type, MessageType::NOD(305));
        assert_eq!(frame.message.node_id, 0xAC);
        assert_eq!(frame.message.service_code, ServiceCodeEnum::UNKNOWN.as_u8());
        assert_eq!(frame.message.message_code, 0x1);
        assert_eq!(
            frame.message.payload.data,
            DataType::ULONG(0xDEAD_BEEF).to_be_bytes()
        );
    }
}

#[cfg(test)]
mod rawmessage {
    use crate::message::RawMessage;

    #[test]
    fn test_new_nopayload() {
        let message = RawMessage::new(&[0x2, 0x3, 0x4, 0x5]).unwrap();
        assert_eq!(message.payload.len, 0);
        assert_eq!(message.node_id, 0x2);
        assert_eq!(message.data_type, 0x3);
        assert_eq!(message.service_code, 0x4);
        assert_eq!(message.message_code, 0x5);
    }

    #[test]
    fn test_new_withpayload() {
        let message = RawMessage::new(&[0x2, 0x3, 0x4, 0x5, 0x6, 0x7]).unwrap();
        assert_eq!(message.payload.len, 2);
        assert_eq!(message.payload.data[0], 0x6);
        assert_eq!(message.payload.data[1], 0x7);

        assert_eq!(message.node_id, 0x2);
        assert_eq!(message.data_type, 0x3);
        assert_eq!(message.service_code, 0x4);
        assert_eq!(message.message_code, 0x5);
    }

    #[test]
    fn test_empty() {
        let message = RawMessage::empty();
        assert_eq!(message.payload.len, 0);
        assert_eq!(message.node_id, 0x0);
        assert_eq!(message.data_type, 0x0);
        assert_eq!(message.service_code, 0x0);
        assert_eq!(message.message_code, 0x0);
    }

    #[test]
    fn test_from_arr4() {
        let message = RawMessage::from([0x1, 0x2, 0x3, 0x4]);
        assert_eq!(message.payload.len, 0);
        assert_eq!(message.node_id, 0x1);
        assert_eq!(message.data_type, 0x2);
        assert_eq!(message.service_code, 0x3);
        assert_eq!(message.message_code, 0x4);
    }

    #[test]
    fn test_from_arr5() {
        let message = RawMessage::from([0x1, 0x2, 0x3, 0x4, 0x5]);
        assert_eq!(message.payload.len, 1);
        assert_eq!(message.payload.data[0], 0x5);
        assert_eq!(message.node_id, 0x1);
        assert_eq!(message.data_type, 0x2);
        assert_eq!(message.service_code, 0x3);
        assert_eq!(message.message_code, 0x4);
    }

    #[test]
    fn test_from_arr6() {
        let message = RawMessage::from([0x1, 0x2, 0x3, 0x4, 0x5, 0x6]);
        assert_eq!(message.payload.len, 2);
        assert_eq!(message.payload.data[0], 0x5);
        assert_eq!(message.payload.data[1], 0x6);
        assert_eq!(message.node_id, 0x1);
        assert_eq!(message.data_type, 0x2);
        assert_eq!(message.service_code, 0x3);
        assert_eq!(message.message_code, 0x4);
    }

    #[test]
    fn test_from_arr7() {
        let message = RawMessage::from([0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]);
        assert_eq!(message.payload.len, 3);
        assert_eq!(message.payload.data[0], 0x5);
        assert_eq!(message.payload.data[1], 0x6);
        assert_eq!(message.payload.data[2], 0x7);

        assert_eq!(message.node_id, 0x1);
        assert_eq!(message.data_type, 0x2);
        assert_eq!(message.service_code, 0x3);
        assert_eq!(message.message_code, 0x4);
    }

    #[test]
    fn test_from_arr8() {
        let message = RawMessage::from([0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8]);
        assert_eq!(message.payload.len, 4);
        assert_eq!(message.payload.data[0], 0x5);
        assert_eq!(message.payload.data[1], 0x6);
        assert_eq!(message.payload.data[2], 0x7);
        assert_eq!(message.payload.data[3], 0x8);

        assert_eq!(message.node_id, 0x1);
        assert_eq!(message.data_type, 0x2);
        assert_eq!(message.service_code, 0x3);
        assert_eq!(message.message_code, 0x4);
    }
}

#[cfg(test)]
mod payload {
    use crate::{message::Payload, types::DataType};

    #[test]
    fn test_new() {
        let p = Payload::new(&[0x1, 0x2, 0x3, 0x4]).unwrap();
        assert_eq!(p.len, 0x4);
        assert_eq!(p.data[0], 0x1);
        assert_eq!(p.data[1], 0x2);
        assert_eq!(p.data[2], 0x3);
        assert_eq!(p.data[3], 0x4);
    }

    #[test]
    fn test_new_empty() {
        let p = Payload::new(&[]).unwrap();
        assert_eq!(p.len, 0x0);
        assert_eq!(p.data[0], 0x0);
        assert_eq!(p.data[1], 0x0);
        assert_eq!(p.data[2], 0x0);
        assert_eq!(p.data[3], 0x0);
    }

    #[test]
    fn test_from_datatype() {
        let p = Payload::from(&DataType::ULONG(0xDEAD_DEAD));

        assert_eq!(p.len, 4);
        assert_eq!(p.data, DataType::ULONG(0xDEAD_DEAD).to_be_bytes());
    }

    #[test]
    fn test_from_arr0() {
        let p = Payload::from([]);

        assert_eq!(p.len, 0);
        assert_eq!(p.data, [0, 0, 0, 0]);
    }

    #[test]
    fn test_from_arr1() {
        let p = Payload::from([1]);

        assert_eq!(p.len, 1);
        assert_eq!(p.data, [1, 0, 0, 0]);
    }

    #[test]
    fn test_from_arr2() {
        let p = Payload::from([1, 2]);

        assert_eq!(p.len, 2);
        assert_eq!(p.data, [1, 2, 0, 0]);
    }

    #[test]
    fn test_from_arr3() {
        let p = Payload::from([1, 2, 3]);

        assert_eq!(p.len, 3);
        assert_eq!(p.data, [1, 2, 3, 0]);
    }

    #[test]
    fn test_from_arr4() {
        let p = Payload::from([1, 2, 3, 4]);

        assert_eq!(p.len, 4);
        assert_eq!(p.data, [1, 2, 3, 4]);
    }
}
