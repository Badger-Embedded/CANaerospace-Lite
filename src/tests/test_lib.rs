#[cfg(test)]
pub mod canaerospacelite {

    use crate::{CANAerospaceLite, IDS_CONF_STANDARD, IDS_MSG_HEADER_STANDARD, driver::CANAerospaceDriver, message::{CANAerospaceFrame, CANAerospaceMessage, Payload, RawMessage}, types::{DataType, HardwareRevision, IDSConfiguration, MessageType, ServiceCode, ServiceCodeEnum, SoftwareRevision}};

    static mut DRIVER_SEND_FRAME_CALL: u8 = 0;
    static mut DRIVER_RECV_FRAME_CALL: u8 = 0;
    static mut DRIVER_FRAME_HOLDER: Option<CANAerospaceFrame> = None;

    struct CANDriverMock;
    impl CANAerospaceDriver for CANDriverMock {
        fn send_frame(&mut self, frame: CANAerospaceFrame) {
            unsafe {
                DRIVER_SEND_FRAME_CALL += 1;
                DRIVER_FRAME_HOLDER = Some(frame);
            }
        }

        fn recv_frame(&mut self) -> Option<CANAerospaceFrame> {
            unsafe {
                DRIVER_RECV_FRAME_CALL += 1;
                if let Some(frame) = &DRIVER_FRAME_HOLDER {
                    Some(frame.clone())
                } else {
                    None
                }
            }
        }
    }

    #[test]
    fn test_new() {
        let canas = CANAerospaceLite::new(10, CANDriverMock{});
        assert_eq!(10, canas.node_id);
        assert_eq!(0, canas.nod_count);
        assert_eq!(canas.identification.hw_rev.0, 0);
        assert_eq!(canas.identification.sw_rev.0, 0);
        assert_eq!(canas.identification.configuration.0, 0);
        assert_eq!(canas.identification.header, 0);
        assert_eq!(canas.rx_queue.len(), 0);
    }

    #[test]
    fn test_set_hw_revision() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        canas.set_hw_revision(HardwareRevision(0x01));
        assert_eq!(canas.identification.hw_rev.0, 0x01);
        canas.set_hw_revision(HardwareRevision(0x00));
        assert_eq!(canas.identification.hw_rev.0, 0x00);
        canas.set_hw_revision(HardwareRevision(0xFF));
        assert_eq!(canas.identification.hw_rev.0, 0xFF);
    }

    #[test]
    fn test_set_sw_revision() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        canas.set_sw_revision(SoftwareRevision(0x01));
        assert_eq!(canas.identification.sw_rev.0, 0x01);
        canas.set_sw_revision(SoftwareRevision(0x00));
        assert_eq!(canas.identification.sw_rev.0, 0x00);
        canas.set_sw_revision(SoftwareRevision(0xFF));
        assert_eq!(canas.identification.sw_rev.0, 0xFF);
    }

    #[test]
    fn test_set_ids_configuration() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        canas.set_ids_configuration(IDSConfiguration(0x00));
        assert_eq!(canas.identification.configuration.0, 0);
        assert_eq!(canas.identification.configuration.0, IDS_CONF_STANDARD.0);

        canas.set_ids_configuration(IDSConfiguration(0xFF));
        assert_eq!(canas.identification.configuration.0, 0xFF);
    }

    #[test]
    fn test_set_message_header_conf() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        canas.set_message_header_conf(0x00);
        assert_eq!(canas.identification.header, 0);
        assert_eq!(canas.identification.header, IDS_MSG_HEADER_STANDARD);

        canas.set_message_header_conf(0xFF);
        assert_eq!(canas.identification.header, 0xFF);
    }

    #[test]
    fn test_send_message_nod() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        let message = CANAerospaceMessage{
            message_type: MessageType::NOD(300),
            node_id: 10,
            service_code: ServiceCodeEnum::UNKNOWN,
            message_code: 0,
            data: DataType::USHORT2(0xDEAD, 0xBEEF),
        };
        unsafe {
            DRIVER_SEND_FRAME_CALL = 0;
            canas.send_message(message);

            assert_eq!(DRIVER_SEND_FRAME_CALL, 1);
            assert!(matches!(DRIVER_FRAME_HOLDER, Some(_)));
            if let Some(frame) = &DRIVER_FRAME_HOLDER {
                assert!(matches!(frame.message_type, MessageType::NOD(300)));
                assert_eq!(frame.message.node_id, 10);
                assert_eq!(frame.message.service_code, ServiceCodeEnum::UNKNOWN as u8);
                assert_eq!(frame.message.message_code, canas.nod_count);

                let data = DataType::USHORT2(0xDEAD, 0xBEEF).to_be_bytes();
                for (i, val) in data.iter().enumerate() {
                    assert_eq!(frame.message.payload.data[i], *val);
                }
            }
        }

    }

    // TODO: test_send_message for each message type

    #[test]
    fn test_read_message_empty() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        assert_eq!(canas.read_message().is_none(), true);
    }

    #[test]
    fn test_read_message_nod() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        unsafe {
            DRIVER_FRAME_HOLDER = Some(CANAerospaceFrame{
                message_type: MessageType::NOD(300),
                message: RawMessage {
                    node_id: 0,
                    data_type: DataType::ULONG(0).type_id(),
                    service_code: 0xFF,
                    message_code: 0,
                    payload: Payload::from(0xDEAD_BEEFu32.to_be_bytes()),
                },
            });
            DRIVER_RECV_FRAME_CALL = 0;
            canas.notify_receive_event();
            assert_eq!(DRIVER_RECV_FRAME_CALL, 1);
            let message = canas.read_message();
            assert!(matches!(message, Some(_)));
            if let Some(m) = message {
                assert!(matches!(m.message_type, MessageType::NOD(300)));
                assert_eq!(m.node_id, 0);
                assert!(matches!(m.service_code, ServiceCodeEnum::UNKNOWN));
                assert_eq!(m.message_code, 0);
                assert!(matches!(m.data, DataType::ULONG(0xDEAD_BEEF)));
            }
        }
    }

    // TODO: test_read_message for each message type

    #[test]
    fn test_notify_receive_event() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        unsafe {
            DRIVER_RECV_FRAME_CALL = 0;
            canas.notify_receive_event();
            canas.notify_receive_event();
            canas.notify_receive_event();
            assert_eq!(DRIVER_RECV_FRAME_CALL, 3);
        }
    }

    #[test]
    fn test_handle_service_request_discard() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        unsafe {
            let be_discarded = CANAerospaceFrame{
                message_type: MessageType::NSH(128),
                message: RawMessage {
                    node_id: 15,
                    data_type: DataType::NODATA.type_id(),
                    service_code: ServiceCodeEnum::IDS as u8,
                    message_code: 0,
                    payload: Payload::from([]),
                },
            };
            DRIVER_SEND_FRAME_CALL = 0;
            canas.handle_service_request(be_discarded);
            assert_eq!(DRIVER_SEND_FRAME_CALL, 0);
            assert_eq!(canas.rx_queue.len(), 0);
        }
    }

    #[test]
    fn test_handle_service_request_ids() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        canas.set_sw_revision(SoftwareRevision(0x01));
        unsafe {
            let ids = CANAerospaceFrame{
                message_type: MessageType::NSH(128),
                message: RawMessage {
                    node_id: 10,
                    data_type: DataType::NODATA.type_id(),
                    service_code: ServiceCodeEnum::IDS as u8,
                    message_code: 0,
                    payload: Payload::from([]),
                },
            };
            DRIVER_SEND_FRAME_CALL = 0;
            canas.handle_service_request(ids);
            assert_eq!(DRIVER_SEND_FRAME_CALL, 1);
            assert!(matches!(DRIVER_FRAME_HOLDER, Some(_)));
            if let Some(frame) = &DRIVER_FRAME_HOLDER {
                assert_eq!(frame.message_type.id(), 129);
                assert_eq!(frame.message.payload.data[0], canas.identification.hw_rev.0);
                assert_eq!(frame.message.payload.data[1], canas.identification.sw_rev.0);
                assert_eq!(frame.message.payload.data[2], canas.identification.configuration.0);
                assert_eq!(frame.message.payload.data[3], canas.identification.header);
            }
            assert_eq!(canas.rx_queue.len(), 0);
        }
    }

    #[test]
    fn test_handle_service_request_notids() {
        let mut canas = CANAerospaceLite::new(10, CANDriverMock{});
        canas.set_sw_revision(SoftwareRevision(0x01));
        unsafe {
            let not_ids = CANAerospaceFrame{
                message_type: MessageType::NSH(128),
                message: RawMessage {
                    node_id: 10,
                    data_type: DataType::NODATA.type_id(),
                    service_code: ServiceCodeEnum::NSS as u8,
                    message_code: 0,
                    payload: Payload::from([]),
                },
            };
            DRIVER_SEND_FRAME_CALL = 0;
            canas.handle_service_request(not_ids);
            assert_eq!(DRIVER_SEND_FRAME_CALL, 0);
            assert_eq!(canas.rx_queue.len(), 1);
        }
    }

}