pub mod standard {
    use crate::{
        message::CANAerospaceMessage,
        types::{DataType, MessageType},
    };

    pub struct EngineStatusBLONG<const N: usize, const S: usize>;
    impl<const N: usize, const S: usize> EngineStatusBLONG<N, S> {
        /// Creates new engine status message. If given N or S values are not defined in standard then function panics.
        ///
        /// N stands for engine number
        /// S stands for status number
        ///```
        /// # use can_aerospace_lite::ids::standard::EngineStatusBLONG;
        /// # use can_aerospace_lite::types::DataType;
        /// let mut message = EngineStatusBLONG::<1, 1>::new(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x26C);
        /// message = EngineStatusBLONG::<2, 1>::new(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x26D);
        /// message = EngineStatusBLONG::<3, 1>::new(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x26E);
        /// message = EngineStatusBLONG::<4, 1>::new(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x26F);
        /// message = EngineStatusBLONG::<1, 2>::new(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x270);
        /// message = EngineStatusBLONG::<2, 2>::new(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x271);
        /// message = EngineStatusBLONG::<3, 2>::new(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x272);
        /// message = EngineStatusBLONG::<4, 2>::new(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x273);
        /// assert_eq!(message.data, DataType::BLONG(0xDEADBEEF));
        ///```
        pub fn new(data: u32) -> CANAerospaceMessage {
            if !(1..=4).contains(&N) || !(1..=2).contains(&S) {
                panic!("Unsupported engine status message creation!")
            }

            CANAerospaceMessage::new(
                MessageType::NOD(engine_status_id(N as u16, S as u16)),
                0x0, // will be assigned by the controller
                0x0,
                0x0, // will be assigned by the controller
                DataType::BLONG(data),
            )
        }
    }

    pub struct EngineStatusBSHORT<const N: usize, const S: usize>;
    impl<const N: usize, const S: usize> EngineStatusBSHORT<N, S> {
        /// Creates new engine status message. If given N or S values are not defined in standard then function panics.
        ///
        /// N stands for engine number
        /// S stands for status number
        ///```
        /// # use can_aerospace_lite::ids::standard::EngineStatusBSHORT;
        /// # use can_aerospace_lite::types::DataType;
        /// let mut message = EngineStatusBSHORT::<1, 1>::new(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x26C);
        /// message = EngineStatusBSHORT::<2, 1>::new(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x26D);
        /// message = EngineStatusBSHORT::<3, 1>::new(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x26E);
        /// message = EngineStatusBSHORT::<4, 1>::new(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x26F);
        /// message = EngineStatusBSHORT::<1, 2>::new(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x270);
        /// message = EngineStatusBSHORT::<2, 2>::new(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x271);
        /// message = EngineStatusBSHORT::<3, 2>::new(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x272);
        /// message = EngineStatusBSHORT::<4, 2>::new(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x273);
        /// assert_eq!(message.data, DataType::BSHORT(0xBEEF));
        ///```
        pub fn new(data: u16) -> CANAerospaceMessage {
            if !(1..=4).contains(&N) || !(1..=2).contains(&S) {
                panic!("Unsupported engine status message creation!")
            }

            CANAerospaceMessage::new(
                MessageType::NOD(engine_status_id(N as u16, S as u16)),
                0x0, // will be assigned by the controller
                0x0,
                0x0, // will be assigned by the controller
                DataType::BSHORT(data),
            )
        }
    }

    fn engine_status_id(engine_num: u16, status: u16) -> u16 {
        let n = (engine_num - 1) as u16;
        let s = (status - 1) as u16;
        0x26C + ((s * 4) + n)
    }

    pub struct DCSystemVoltageFLOAT<const N: u8>;
    impl<const N: u8> DCSystemVoltageFLOAT<N> {
        /// Creates new dc system voltage message. If given N is not defined in standard then function panics.
        ///
        /// N stands for system number
        ///```
        /// # use can_aerospace_lite::ids::standard::DCSystemVoltageFLOAT;
        /// # use can_aerospace_lite::types::DataType;
        /// let mut message = DCSystemVoltageFLOAT::<1>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x398);
        /// message = DCSystemVoltageFLOAT::<2>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x399);
        /// message = DCSystemVoltageFLOAT::<3>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x39A);
        /// message = DCSystemVoltageFLOAT::<4>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x39B);
        /// message = DCSystemVoltageFLOAT::<5>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x39C);
        /// message = DCSystemVoltageFLOAT::<6>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x39D);
        /// message = DCSystemVoltageFLOAT::<7>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x39E);
        /// message = DCSystemVoltageFLOAT::<8>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x39F);
        /// message = DCSystemVoltageFLOAT::<9>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A0);
        /// message = DCSystemVoltageFLOAT::<10>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A1);
        /// assert_eq!(message.data, DataType::FLOAT(19.96));
        ///```
        pub fn new(data: f32) -> CANAerospaceMessage {
            if !(1..=10).contains(&N) {
                panic!("Unsupported dc system voltage message creation!")
            }

            CANAerospaceMessage::new(
                MessageType::NOD(dc_system_voltage_id(N)),
                0x0, // will be assigned by the controller
                0x0,
                0x0, // will be assigned by the controller
                DataType::FLOAT(data),
            )
        }
    }

    pub struct DCSystemVoltageSHORT2<const N: u8>;
    impl<const N: u8> DCSystemVoltageSHORT2<N> {
        /// Creates new dc system voltage message. If given N is not defined in standard then function panics.
        ///
        /// N stands for system number
        ///```
        /// # use can_aerospace_lite::ids::standard::DCSystemVoltageSHORT2;
        /// # use can_aerospace_lite::types::DataType;
        /// let mut message = DCSystemVoltageSHORT2::<1>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x398);
        /// message = DCSystemVoltageSHORT2::<2>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x399);
        /// message = DCSystemVoltageSHORT2::<3>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x39A);
        /// message = DCSystemVoltageSHORT2::<4>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x39B);
        /// message = DCSystemVoltageSHORT2::<5>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x39C);
        /// message = DCSystemVoltageSHORT2::<6>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x39D);
        /// message = DCSystemVoltageSHORT2::<7>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x39E);
        /// message = DCSystemVoltageSHORT2::<8>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x39F);
        /// message = DCSystemVoltageSHORT2::<9>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A0);
        /// message = DCSystemVoltageSHORT2::<10>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A1);
        /// assert_eq!(message.data, DataType::SHORT2(19,96));
        ///```
        pub fn new(v1: i16, v2: i16) -> CANAerospaceMessage {
            if !(1..=10).contains(&N) {
                panic!("Unsupported dc system voltage message creation!")
            }

            CANAerospaceMessage::new(
                MessageType::NOD(dc_system_voltage_id(N)),
                0x0, // will be assigned by the controller
                0x0,
                0x0, // will be assigned by the controller
                DataType::SHORT2(v1, v2),
            )
        }
    }

    fn dc_system_voltage_id(num: u8) -> u16 {
        let n = (num - 1) as u16;
        0x398 + n
    }

    pub struct DCSystemCurrentFLOAT<const N: u8>;
    impl<const N: u8> DCSystemCurrentFLOAT<N> {
        /// Creates new dc system current message. If given N is not defined in standard then function panics.
        ///
        /// N stands for system number
        ///```
        /// # use can_aerospace_lite::ids::standard::DCSystemCurrentFLOAT;
        /// # use can_aerospace_lite::types::DataType;
        /// let mut message = DCSystemCurrentFLOAT::<1>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A2);
        /// message = DCSystemCurrentFLOAT::<2>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A3);
        /// message = DCSystemCurrentFLOAT::<3>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A4);
        /// message = DCSystemCurrentFLOAT::<4>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A5);
        /// message = DCSystemCurrentFLOAT::<5>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A6);
        /// message = DCSystemCurrentFLOAT::<6>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A7);
        /// message = DCSystemCurrentFLOAT::<7>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A8);
        /// message = DCSystemCurrentFLOAT::<8>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A9);
        /// message = DCSystemCurrentFLOAT::<9>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3AA);
        /// message = DCSystemCurrentFLOAT::<10>::new(19.96);
        /// assert_eq!(message.message_type.id(), 0x3AB);
        /// assert_eq!(message.data, DataType::FLOAT(19.96));
        ///```
        pub fn new(data: f32) -> CANAerospaceMessage {
            if !(1..=10).contains(&N) {
                panic!("Unsupported dc system current message creation!")
            }

            CANAerospaceMessage::new(
                MessageType::NOD(dc_system_current_id(N)),
                0x0, // will be assigned by the controller
                0x0,
                0x0, // will be assigned by the controller
                DataType::FLOAT(data),
            )
        }
    }

    pub struct DCSystemCurrentSHORT2<const N: u8>;
    impl<const N: u8> DCSystemCurrentSHORT2<N> {
        /// Creates new dc system current message. If given N is not defined in standard then function panics.
        ///
        /// N stands for system number
        ///```
        /// # use can_aerospace_lite::ids::standard::DCSystemCurrentSHORT2;
        /// # use can_aerospace_lite::types::DataType;
        /// let mut message = DCSystemCurrentSHORT2::<1>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A2);
        /// message = DCSystemCurrentSHORT2::<2>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A3);
        /// message = DCSystemCurrentSHORT2::<3>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A4);
        /// message = DCSystemCurrentSHORT2::<4>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A5);
        /// message = DCSystemCurrentSHORT2::<5>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A6);
        /// message = DCSystemCurrentSHORT2::<6>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A7);
        /// message = DCSystemCurrentSHORT2::<7>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A8);
        /// message = DCSystemCurrentSHORT2::<8>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A9);
        /// message = DCSystemCurrentSHORT2::<9>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3AA);
        /// message = DCSystemCurrentSHORT2::<10>::new(19,96);
        /// assert_eq!(message.message_type.id(), 0x3AB);
        /// assert_eq!(message.data, DataType::SHORT2(19,96));
        ///```
        pub fn new(v1: i16, v2: i16) -> CANAerospaceMessage {
            if !(1..=10).contains(&N) {
                panic!("Unsupported dc system current message creation!")
            }

            CANAerospaceMessage::new(
                MessageType::NOD(dc_system_current_id(N)),
                0x0, // will be assigned by the controller
                0x0,
                0x0, // will be assigned by the controller
                DataType::SHORT2(v1, v2),
            )
        }
    }

    fn dc_system_current_id(num: u8) -> u16 {
        let n = (num - 1) as u16;
        0x3A2 + n
    }
}
