//! # CANAeropsace - ids
//!
//! Module contains standard messages defined by the CANaerospace standard.

pub mod standard {
    use crate::{
        message::CANAerospaceMessage,
        types::{DataType, MessageType, ServiceCodeEnum},
    };

    pub struct BodyLongitudinalAcceleration;
    impl BodyLongitudinalAcceleration {
        /// Creates new body longitudinal acceleration message.
        /// Units: g
        /// Notes:
        ///     forward: +
        ///     aft: -
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyLongitudinalAcceleration;
        /// # use can_aerospace_lite::types::DataType;
        /// let accl = BodyLongitudinalAcceleration::create(DataType::FLOAT(1.5));
        /// assert_eq!(accl.message_type.id(), 0x12C);
        /// assert_eq!(accl.data, DataType::FLOAT(1.5));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x12C),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyLateralAcceleration;
    impl BodyLateralAcceleration {
        /// Creates new body lateral acceleration message.
        /// Units: g
        /// Notes:
        ///     right: +
        ///     left: -
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyLateralAcceleration;
        /// # use can_aerospace_lite::types::DataType;
        /// let accl = BodyLateralAcceleration::create(DataType::FLOAT(0.1));
        /// assert_eq!(accl.message_type.id(), 0x12D);
        /// assert_eq!(accl.data, DataType::FLOAT(0.1));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x12D),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyNormalAcceleration;
    impl BodyNormalAcceleration {
        /// Creates new body normal acceleration message.
        /// Units: g
        /// Notes:
        ///     up: +
        ///     down: -
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyNormalAcceleration;
        /// # use can_aerospace_lite::types::DataType;
        /// let accl = BodyNormalAcceleration::create(DataType::FLOAT(0.5));
        /// assert_eq!(accl.message_type.id(), 0x12E);
        /// assert_eq!(accl.data, DataType::FLOAT(0.5));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x12E),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyPitchRate;
    impl BodyPitchRate {
        /// Creates new body pitch rate message.
        /// Units: deg/s
        /// Notes:
        ///     nose up: +
        ///     nose down: -
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyPitchRate;
        /// # use can_aerospace_lite::types::DataType;
        /// let pitch = BodyPitchRate::create(DataType::FLOAT(1.0));
        /// assert_eq!(pitch.message_type.id(), 0x12F);
        /// assert_eq!(pitch.data, DataType::FLOAT(1.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x12F),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyRollRate;
    impl BodyRollRate {
        /// Creates new body roll rate message.
        /// Units: deg/s
        /// Notes:
        ///     roll right: +
        ///     roll left: -
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyRollRate;
        /// # use can_aerospace_lite::types::DataType;
        /// let roll = BodyRollRate::create(DataType::FLOAT(1.0));
        /// assert_eq!(roll.message_type.id(), 0x130);
        /// assert_eq!(roll.data, DataType::FLOAT(1.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x130),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyYawRate;
    impl BodyYawRate {
        /// Creates new body yaw rate message.
        /// Units: deg/s
        /// Notes:
        ///     yaw right: +
        ///     yaw left: -
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyYawRate;
        /// # use can_aerospace_lite::types::DataType;
        /// let yaw = BodyYawRate::create(DataType::FLOAT(1.0));
        /// assert_eq!(yaw.message_type.id(), 0x131);
        /// assert_eq!(yaw.data, DataType::FLOAT(1.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x131),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyPitchAngle;
    impl BodyPitchAngle {
        /// Creates new body pitch angle message.
        /// Units: deg
        /// Notes:
        ///     nose up: +
        ///     nose down: -
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyPitchAngle;
        /// # use can_aerospace_lite::types::DataType;
        /// let pitch = BodyPitchAngle::create(DataType::FLOAT(0.1));
        /// assert_eq!(pitch.message_type.id(), 0x137);
        /// assert_eq!(pitch.data, DataType::FLOAT(0.1));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x137),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyRollAngle;
    impl BodyRollAngle {
        /// Creates new body roll angle message.
        /// Units: deg
        /// Notes:
        ///     roll right: +
        ///     roll left: -
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyRollAngle;
        /// # use can_aerospace_lite::types::DataType;
        /// let roll = BodyRollAngle::create(DataType::FLOAT(1.0));
        /// assert_eq!(roll.message_type.id(), 0x138);
        /// assert_eq!(roll.data, DataType::FLOAT(1.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x138),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodySideSlip;
    impl BodySideSlip {
        /// Creates new body side slip message.
        /// Units: deg
        /// Notes:
        ///     yaw right: +
        ///     yaw left: -
        ///```
        /// # use can_aerospace_lite::ids::standard::BodySideSlip;
        /// # use can_aerospace_lite::types::DataType;
        /// let yaw = BodySideSlip::create(DataType::FLOAT(0.2));
        /// assert_eq!(yaw.message_type.id(), 0x139);
        /// assert_eq!(yaw.data, DataType::FLOAT(0.2));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x139),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct AltitudeRate;
    impl AltitudeRate {
        /// Creates new altitude rate message.
        /// Units: m/s
        ///```
        /// # use can_aerospace_lite::ids::standard::AltitudeRate;
        /// # use can_aerospace_lite::types::DataType;
        /// let alt = AltitudeRate::create(DataType::FLOAT(10.2));
        /// assert_eq!(alt.message_type.id(), 0x13A);
        /// assert_eq!(alt.data, DataType::FLOAT(10.2));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x13A),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BaroCorrectedAltitude;
    impl BaroCorrectedAltitude {
        /// Creates new baro corrected altitude message.
        /// Units: m
        ///```
        /// # use can_aerospace_lite::ids::standard::BaroCorrectedAltitude;
        /// # use can_aerospace_lite::types::DataType;
        /// let alt = BaroCorrectedAltitude::create(DataType::FLOAT(50.0));
        /// assert_eq!(alt.message_type.id(), 0x140);
        /// assert_eq!(alt.data, DataType::FLOAT(50.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x140),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct HeadingAngle;
    impl HeadingAngle {
        /// Creates new heading angle message.
        /// Units: deg
        /// Notes: +/- 180(deg)
        ///```
        /// # use can_aerospace_lite::ids::standard::HeadingAngle;
        /// # use can_aerospace_lite::types::DataType;
        /// let heading = HeadingAngle::create(DataType::FLOAT(50.0));
        /// assert_eq!(heading.message_type.id(), 0x141);
        /// assert_eq!(heading.data, DataType::FLOAT(50.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x141),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct StandardAltitude;
    impl StandardAltitude {
        /// Creates new standard altitude message.
        /// Units: m
        ///```
        /// # use can_aerospace_lite::ids::standard::StandardAltitude;
        /// # use can_aerospace_lite::types::DataType;
        /// let alt = StandardAltitude::create(DataType::FLOAT(50.0));
        /// assert_eq!(alt.message_type.id(), 0x142);
        /// assert_eq!(alt.data, DataType::FLOAT(50.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x142),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct HeadingRate;
    impl HeadingRate {
        /// Creates new heading rate message.
        /// Units: deg/s
        ///```
        /// # use can_aerospace_lite::ids::standard::HeadingRate;
        /// # use can_aerospace_lite::types::DataType;
        /// let heading = HeadingRate::create(DataType::FLOAT(5.0));
        /// assert_eq!(heading.message_type.id(), 0x147);
        /// assert_eq!(heading.data, DataType::FLOAT(5.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x147),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct TrueAltitude;
    impl TrueAltitude {
        /// Creates new true altitude message.
        /// Units: m
        ///```
        /// # use can_aerospace_lite::ids::standard::TrueAltitude;
        /// # use can_aerospace_lite::types::DataType;
        /// let heading = TrueAltitude::create(DataType::FLOAT(5.0));
        /// assert_eq!(heading.message_type.id(), 0x14C);
        /// assert_eq!(heading.data, DataType::FLOAT(5.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x14C),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyNormalVelocity;
    impl BodyNormalVelocity {
        /// Creates new body normal velocity message.
        /// Units: m/s
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyNormalVelocity;
        /// # use can_aerospace_lite::types::DataType;
        /// let vel = BodyNormalVelocity::create(DataType::FLOAT(15.0));
        /// assert_eq!(vel.message_type.id(), 0x150);
        /// assert_eq!(vel.data, DataType::FLOAT(15.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x150),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyLongitudinalVelocity;
    impl BodyLongitudinalVelocity {
        /// Creates new body longitudinal velocity message.
        /// Units: m/s
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyLongitudinalVelocity;
        /// # use can_aerospace_lite::types::DataType;
        /// let vel = BodyLongitudinalVelocity::create(DataType::FLOAT(15.0));
        /// assert_eq!(vel.message_type.id(), 0x151);
        /// assert_eq!(vel.data, DataType::FLOAT(15.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x151),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct BodyLateralVelocity;
    impl BodyLateralVelocity {
        /// Creates new body lateral velocity message.
        /// Units: m/s
        ///```
        /// # use can_aerospace_lite::ids::standard::BodyLateralVelocity;
        /// # use can_aerospace_lite::types::DataType;
        /// let vel = BodyLateralVelocity::create(DataType::FLOAT(15.0));
        /// assert_eq!(vel.message_type.id(), 0x152);
        /// assert_eq!(vel.data, DataType::FLOAT(15.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x152),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct EngineStatusBLONG<const N: usize, const S: usize>;
    impl<const N: usize, const S: usize> EngineStatusBLONG<N, S> {
        /// Creates new engine status message. If given N or S values are not defined in standard then function panics.
        ///
        /// N stands for engine number
        /// S stands for status number
        ///```
        /// # use can_aerospace_lite::ids::standard::EngineStatusBLONG;
        /// # use can_aerospace_lite::types::DataType;
        /// let mut message = EngineStatusBLONG::<1, 1>::create(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x26C);
        /// message = EngineStatusBLONG::<2, 1>::create(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x26D);
        /// message = EngineStatusBLONG::<3, 1>::create(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x26E);
        /// message = EngineStatusBLONG::<4, 1>::create(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x26F);
        /// message = EngineStatusBLONG::<1, 2>::create(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x270);
        /// message = EngineStatusBLONG::<2, 2>::create(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x271);
        /// message = EngineStatusBLONG::<3, 2>::create(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x272);
        /// message = EngineStatusBLONG::<4, 2>::create(0xDEADBEEF);
        /// assert_eq!(message.message_type.id(), 0x273);
        /// assert_eq!(message.data, DataType::BLONG(0xDEADBEEF));
        ///```
        pub fn create(data: u32) -> CANAerospaceMessage {
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
        /// let mut message = EngineStatusBSHORT::<1, 1>::create(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x26C);
        /// message = EngineStatusBSHORT::<2, 1>::create(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x26D);
        /// message = EngineStatusBSHORT::<3, 1>::create(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x26E);
        /// message = EngineStatusBSHORT::<4, 1>::create(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x26F);
        /// message = EngineStatusBSHORT::<1, 2>::create(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x270);
        /// message = EngineStatusBSHORT::<2, 2>::create(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x271);
        /// message = EngineStatusBSHORT::<3, 2>::create(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x272);
        /// message = EngineStatusBSHORT::<4, 2>::create(0xBEEF);
        /// assert_eq!(message.message_type.id(), 0x273);
        /// assert_eq!(message.data, DataType::BSHORT(0xBEEF));
        ///```
        pub fn create(data: u16) -> CANAerospaceMessage {
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
        /// let mut message = DCSystemVoltageFLOAT::<1>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x398);
        /// message = DCSystemVoltageFLOAT::<2>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x399);
        /// message = DCSystemVoltageFLOAT::<3>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x39A);
        /// message = DCSystemVoltageFLOAT::<4>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x39B);
        /// message = DCSystemVoltageFLOAT::<5>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x39C);
        /// message = DCSystemVoltageFLOAT::<6>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x39D);
        /// message = DCSystemVoltageFLOAT::<7>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x39E);
        /// message = DCSystemVoltageFLOAT::<8>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x39F);
        /// message = DCSystemVoltageFLOAT::<9>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A0);
        /// message = DCSystemVoltageFLOAT::<10>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A1);
        /// assert_eq!(message.data, DataType::FLOAT(19.96));
        ///```
        pub fn create(data: f32) -> CANAerospaceMessage {
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
        /// let mut message = DCSystemVoltageSHORT2::<1>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x398);
        /// message = DCSystemVoltageSHORT2::<2>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x399);
        /// message = DCSystemVoltageSHORT2::<3>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x39A);
        /// message = DCSystemVoltageSHORT2::<4>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x39B);
        /// message = DCSystemVoltageSHORT2::<5>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x39C);
        /// message = DCSystemVoltageSHORT2::<6>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x39D);
        /// message = DCSystemVoltageSHORT2::<7>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x39E);
        /// message = DCSystemVoltageSHORT2::<8>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x39F);
        /// message = DCSystemVoltageSHORT2::<9>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A0);
        /// message = DCSystemVoltageSHORT2::<10>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A1);
        /// assert_eq!(message.data, DataType::SHORT2(19,96));
        ///```
        pub fn create(v1: i16, v2: i16) -> CANAerospaceMessage {
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
        /// let mut message = DCSystemCurrentFLOAT::<1>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A2);
        /// message = DCSystemCurrentFLOAT::<2>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A3);
        /// message = DCSystemCurrentFLOAT::<3>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A4);
        /// message = DCSystemCurrentFLOAT::<4>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A5);
        /// message = DCSystemCurrentFLOAT::<5>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A6);
        /// message = DCSystemCurrentFLOAT::<6>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A7);
        /// message = DCSystemCurrentFLOAT::<7>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A8);
        /// message = DCSystemCurrentFLOAT::<8>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3A9);
        /// message = DCSystemCurrentFLOAT::<9>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3AA);
        /// message = DCSystemCurrentFLOAT::<10>::create(19.96);
        /// assert_eq!(message.message_type.id(), 0x3AB);
        /// assert_eq!(message.data, DataType::FLOAT(19.96));
        ///```
        pub fn create(data: f32) -> CANAerospaceMessage {
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
        /// let mut message = DCSystemCurrentSHORT2::<1>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A2);
        /// message = DCSystemCurrentSHORT2::<2>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A3);
        /// message = DCSystemCurrentSHORT2::<3>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A4);
        /// message = DCSystemCurrentSHORT2::<4>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A5);
        /// message = DCSystemCurrentSHORT2::<5>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A6);
        /// message = DCSystemCurrentSHORT2::<6>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A7);
        /// message = DCSystemCurrentSHORT2::<7>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A8);
        /// message = DCSystemCurrentSHORT2::<8>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3A9);
        /// message = DCSystemCurrentSHORT2::<9>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3AA);
        /// message = DCSystemCurrentSHORT2::<10>::create(19,96);
        /// assert_eq!(message.message_type.id(), 0x3AB);
        /// assert_eq!(message.data, DataType::SHORT2(19,96));
        ///```
        pub fn create(v1: i16, v2: i16) -> CANAerospaceMessage {
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

    pub struct GPSAircraftLatitude;
    impl GPSAircraftLatitude {
        /// Creates new GPS aircraft latitude message.
        ///```
        /// # use can_aerospace_lite::ids::standard::GPSAircraftLatitude;
        /// # use can_aerospace_lite::types::DataType;
        /// let lat_h = GPSAircraftLatitude::create(DataType::DOUBLEH(51));
        /// let lat_l = GPSAircraftLatitude::create(DataType::DOUBLEL(4401459));
        /// assert_eq!(lat_h.message_type.id(), 0x40C);
        /// assert_eq!(lat_l.message_type.id(), 0x40C);
        /// assert_eq!(lat_h.data, DataType::DOUBLEH(51));
        /// assert_eq!(lat_l.data, DataType::DOUBLEL(4401459));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x40C),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct GPSAircraftLongitude;
    impl GPSAircraftLongitude {
        /// Creates new GPS aircraft longitude message.
        ///```
        /// # use can_aerospace_lite::ids::standard::GPSAircraftLongitude;
        /// # use can_aerospace_lite::types::DataType;
        /// let lon_h = GPSAircraftLongitude::create(DataType::DOUBLEH(5));
        /// let lon_l = GPSAircraftLongitude::create(DataType::DOUBLEL(4707237));
        /// assert_eq!(lon_h.message_type.id(), 0x40D);
        /// assert_eq!(lon_l.message_type.id(), 0x40D);
        /// assert_eq!(lon_h.data, DataType::DOUBLEH(5));
        /// assert_eq!(lon_l.data, DataType::DOUBLEL(4707237));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x40D),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct GPSAircraftHeightAboveEllips;
    impl GPSAircraftHeightAboveEllips {
        /// Creates new GPS aircraft height above ellipsis message.
        ///```
        /// # use can_aerospace_lite::ids::standard::GPSAircraftHeightAboveEllips;
        /// # use can_aerospace_lite::types::DataType;
        /// let height = GPSAircraftHeightAboveEllips::create(DataType::FLOAT(5.5));
        /// assert_eq!(height.message_type.id(), 0x40E);
        /// assert_eq!(height.data, DataType::FLOAT(5.5));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x40E),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct GPSGroundSpeed;
    impl GPSGroundSpeed {
        /// Creates new GPS aircraft ground speed message.
        ///```
        /// # use can_aerospace_lite::ids::standard::GPSGroundSpeed;
        /// # use can_aerospace_lite::types::DataType;
        /// let height = GPSGroundSpeed::create(DataType::FLOAT(5.5));
        /// assert_eq!(height.message_type.id(), 0x40F);
        /// assert_eq!(height.data, DataType::FLOAT(5.5));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x40F),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct INSAircraftLatitude;
    impl INSAircraftLatitude {
        /// Creates new INS aircraft latitude message.
        ///```
        /// # use can_aerospace_lite::ids::standard::INSAircraftLatitude;
        /// # use can_aerospace_lite::types::DataType;
        /// let lat_h = INSAircraftLatitude::create(DataType::DOUBLEH(51));
        /// let lat_l = INSAircraftLatitude::create(DataType::DOUBLEL(4401459));
        /// assert_eq!(lat_h.message_type.id(), 0x419);
        /// assert_eq!(lat_l.message_type.id(), 0x419);
        /// assert_eq!(lat_h.data, DataType::DOUBLEH(51));
        /// assert_eq!(lat_l.data, DataType::DOUBLEL(4401459));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x419),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct INSAircraftLongitude;
    impl INSAircraftLongitude {
        /// Creates new INS aircraft longitude message.
        ///```
        /// # use can_aerospace_lite::ids::standard::INSAircraftLongitude;
        /// # use can_aerospace_lite::types::DataType;
        /// let lon_h = INSAircraftLongitude::create(DataType::DOUBLEH(5));
        /// let lon_l = INSAircraftLongitude::create(DataType::DOUBLEL(4707237));
        /// assert_eq!(lon_h.message_type.id(), 0x41A);
        /// assert_eq!(lon_l.message_type.id(), 0x41A);
        /// assert_eq!(lon_h.data, DataType::DOUBLEH(5));
        /// assert_eq!(lon_l.data, DataType::DOUBLEL(4707237));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x41A),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct INSAircraftHeightAboveEllips;
    impl INSAircraftHeightAboveEllips {
        /// Creates new INS aircraft height above ellipsis message.
        ///```
        /// # use can_aerospace_lite::ids::standard::INSAircraftHeightAboveEllips;
        /// # use can_aerospace_lite::types::DataType;
        /// let height = INSAircraftHeightAboveEllips::create(DataType::FLOAT(5.5));
        /// assert_eq!(height.message_type.id(), 0x41B);
        /// assert_eq!(height.data, DataType::FLOAT(5.5));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x41B),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct INSGroundSpeed;
    impl INSGroundSpeed {
        /// Creates new INS aircraft ground speed message.
        ///```
        /// # use can_aerospace_lite::ids::standard::INSGroundSpeed;
        /// # use can_aerospace_lite::types::DataType;
        /// let height = INSGroundSpeed::create(DataType::FLOAT(5.5));
        /// assert_eq!(height.message_type.id(), 0x41C);
        /// assert_eq!(height.data, DataType::FLOAT(5.5));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x41C),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct MagneticHeading;
    impl MagneticHeading {
        /// Creates new magnetic heading message.
        ///```
        /// # use can_aerospace_lite::ids::standard::MagneticHeading;
        /// # use can_aerospace_lite::types::DataType;
        /// let height = MagneticHeading::create(DataType::FLOAT(105.5));
        /// assert_eq!(height.message_type.id(), 0x42D);
        /// assert_eq!(height.data, DataType::FLOAT(105.5));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x42D),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct DecisionHeight;
    impl DecisionHeight {
        /// Creates new decision height message.
        ///```
        /// # use can_aerospace_lite::ids::standard::DecisionHeight;
        /// # use can_aerospace_lite::types::DataType;
        /// let height = DecisionHeight::create(DataType::FLOAT(500.0));
        /// assert_eq!(height.message_type.id(), 0x44A);
        /// assert_eq!(height.data, DataType::FLOAT(500.0));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x44A),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }

    pub struct MiscUTC;
    impl MiscUTC {
        /// Creates new miscellaneous UTC message.
        ///```
        /// # use can_aerospace_lite::ids::standard::MiscUTC;
        /// # use can_aerospace_lite::types::DataType;
        /// let height = MiscUTC::create(DataType::CHAR4(13, 43, 22, 00));
        /// assert_eq!(height.message_type.id(), 0x4B0);
        /// assert_eq!(height.data, DataType::CHAR4(13, 43, 22, 00));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x4B0),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }
    pub struct MiscDate;
    impl MiscDate {
        /// Creates new miscellaneous date message.
        ///```
        /// # use can_aerospace_lite::ids::standard::MiscDate;
        /// # use can_aerospace_lite::types::DataType;
        /// let height = MiscDate::create(DataType::CHAR4(10, 08, 19, 96));
        /// assert_eq!(height.message_type.id(), 0x4B6);
        /// assert_eq!(height.data, DataType::CHAR4(10, 08, 19, 96));
        ///```
        pub fn create(data: DataType) -> CANAerospaceMessage {
            CANAerospaceMessage {
                message_type: MessageType::NOD(0x4B6),
                node_id: 0x0,
                service_code: ServiceCodeEnum::UNKNOWN,
                message_code: 0x0,
                data,
            }
        }
    }
}
