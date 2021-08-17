//! # CANAerospace - Types
//!
//! All required types to implement CANAerospace protocol is defined in this module.

use core::convert::TryInto;

use crate::message::RawMessage;

pub type MessageCode = u8;
pub type ServiceCode = u8;
pub type NodeId = u8;
pub type IDSHeaderConfiguration = u8;

#[derive(Clone, Copy, Debug)]
pub struct IDSConfiguration(pub u8);

#[derive(Clone, Copy, Debug)]
pub struct HardwareRevision(pub u8);

#[derive(Clone, Copy, Debug)]
pub struct SoftwareRevision(pub u8);

#[derive(Clone, Copy, Debug)]
pub struct IDSResponse {
    pub hw_rev: HardwareRevision,
    pub sw_rev: SoftwareRevision,
    pub configuration: IDSConfiguration,
    pub header: IDSHeaderConfiguration,
}

#[derive(Clone, Debug)]
pub enum ServiceCodeEnum {
    /// Identification Service (0x0)
    IDS = 0x0,
    /// Node Synchronisation Service (0x1)
    NSS = 0x1,
    /// Data Download Service (0x2)
    DDS = 0x2,
    /// Data Upload Service (0x3)
    DUS = 0x3,
    /// Simultation Control Service (0x4)
    SCS = 0x4,
    /// Transmission Interval Service (0x5)
    TIS = 0x5,
    /// FLASH Programming Service (0x6)
    FPS = 0x6,
    /// State Transmission Service (0x7)
    STS = 0x7,
    /// Filter Setting Service (0x8)
    FSS = 0x8,
    /// Test Control Service (0x9)
    TCS = 0x9,
    /// CAN Baudrate Setting Service (0xA)
    BSS = 0xA,
    /// NodeId Setting Service (0xB)
    NIS = 0xB,
    /// Module Information Service (0xC)
    MIS = 0xC,
    /// Module Configuration Service (0xD)
    MCS = 0xD,
    /// CAN ID Setting Service (0xE)
    CSS = 0xE,
    /// CAN ID Distribution Setting Service (0xF)
    DSS = 0xF,

    UNKNOWN = 0xFF,
}

impl From<u8> for ServiceCodeEnum {
    fn from(code: u8) -> Self {
        match code {
            code if code == ServiceCodeEnum::IDS as u8 => ServiceCodeEnum::IDS,
            code if code == ServiceCodeEnum::NSS as u8 => ServiceCodeEnum::NSS,
            code if code == ServiceCodeEnum::DDS as u8 => ServiceCodeEnum::DDS,
            code if code == ServiceCodeEnum::DUS as u8 => ServiceCodeEnum::DUS,
            code if code == ServiceCodeEnum::SCS as u8 => ServiceCodeEnum::SCS,
            code if code == ServiceCodeEnum::TIS as u8 => ServiceCodeEnum::TIS,
            code if code == ServiceCodeEnum::FPS as u8 => ServiceCodeEnum::FPS,
            code if code == ServiceCodeEnum::STS as u8 => ServiceCodeEnum::STS,
            code if code == ServiceCodeEnum::FSS as u8 => ServiceCodeEnum::FSS,
            code if code == ServiceCodeEnum::TCS as u8 => ServiceCodeEnum::TCS,
            code if code == ServiceCodeEnum::BSS as u8 => ServiceCodeEnum::BSS,
            code if code == ServiceCodeEnum::NIS as u8 => ServiceCodeEnum::NIS,
            code if code == ServiceCodeEnum::MIS as u8 => ServiceCodeEnum::MIS,
            code if code == ServiceCodeEnum::MCS as u8 => ServiceCodeEnum::MCS,
            code if code == ServiceCodeEnum::CSS as u8 => ServiceCodeEnum::CSS,
            code if code == ServiceCodeEnum::DSS as u8 => ServiceCodeEnum::DSS,
            _ => ServiceCodeEnum::UNKNOWN,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MessageType {
    /// Emergency Event Data \[0,127\]
    /// Transmitted asynchronously whenever a situation requiring immediate action occurs.
    EED(u16),
    /// High Priority Node Service Data \[128,199\]
    /// Transmitted asynchronously or cyclic with defined transmission intervals for operational commands (36 channels)
    NSH(u16),
    /// High Priority User-Defined Data \[200,299\]
    /// Message/data format and transmission intervals entirely user-defined
    UDH(u16),
    /// Normal Operation Data \[300,1799\]
    /// Transmitted asynchronously or cyclic with defined transmission intervals for operational and status data.
    NOD(u16),
    /// Low Priority User-Defined Data \[1800,1899\]
    /// Message/data format and transmission intervals entirely user-defined
    UDL(u16),
    /// Debug Service Data \[1900,1999\]
    /// Transmitted asynchronously or cyclic for debug communication & software download actions.
    DSD(u16),
    /// Low Priority Node Service Data \[2000,2031\]
    /// Transmitted asynchronously or cyclic for test & maintenance actions (16 channels).
    NSL(u16),
    INVALID,
}

impl MessageType {
    pub fn id(&self) -> u16 {
        match *self {
            MessageType::EED(id) => {
                if id > 0x7F {
                    panic!("CANaerospace: Invalid Message ID({}) for EED Message.", id);
                }
                id
            }
            MessageType::NSH(id) => {
                if !(0x80..=0xC7).contains(&id) {
                    panic!("CANaerospace: Invalid Message ID({}) for NSH Message.", id);
                }
                id
            }
            MessageType::UDH(id) => {
                if !(0xC8..=0x12B).contains(&id) {
                    panic!("CANaerospace: Invalid Message ID({}) for UDH Message.", id);
                }
                id
            }
            MessageType::NOD(id) => {
                if !(0x12C..=0x707).contains(&id) {
                    panic!("CANaerospace: Invalid Message ID({}) for NOD Message.", id);
                }
                id
            }
            MessageType::UDL(id) => {
                if !(0x708..=0x76B).contains(&id) {
                    panic!("CANaerospace: Invalid Message ID({}) for UDL Message.", id);
                }
                id
            }
            MessageType::DSD(id) => {
                if !(0x76C..=0x7CF).contains(&id) {
                    panic!("CANaerospace: Invalid Message ID({}) for DSD Message.", id);
                }
                id
            }
            MessageType::NSL(id) => {
                if !(0x7D0..=0x7EF).contains(&id) {
                    panic!("CANaerospace: Invalid Message ID({}) for NSL Message.", id);
                }
                id
            }
            MessageType::INVALID => u16::MAX,
        }
    }
}

impl From<u16> for MessageType {
    fn from(raw_id: u16) -> Self {
        match raw_id {
            0..=127 => MessageType::EED(raw_id),
            128..=199 => MessageType::NSH(raw_id),
            200..=299 => MessageType::UDH(raw_id),
            300..=1799 => MessageType::NOD(raw_id),
            1800..=1899 => MessageType::UDL(raw_id),
            1900..=1999 => MessageType::DSD(raw_id),
            2000..=2031 => MessageType::NSL(raw_id),
            _ => MessageType::INVALID,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataType {
    NODATA,
    ERROR(u32),
    FLOAT(f32),
    LONG(i32),
    ULONG(u32),
    BLONG(u32),
    SHORT(i16),
    USHORT(u16),
    BSHORT(u16),
    CHAR(i8),
    UCHAR(u8),
    BCHAR(u8),
    SHORT2(i16, i16),
    USHORT2(u16, u16),
    BSHORT2(u16, u16),
    CHAR4(i8, i8, i8, i8),
    UCHAR4(u8, u8, u8, u8),
    BCHAR4(u8, u8, u8, u8),
    CHAR2(i8, i8),
    UCHAR2(u8, u8),
    BCHAR2(u8, u8),
    MEMID(u32),
    CHKSUM(u32),
    ACHAR(u8),
    ACHAR2(u8, u8),
    ACHAR4(u8, u8, u8, u8),
    CHAR3(i8, i8, i8),
    UCHAR3(u8, u8, u8),
    BCHAR3(u8, u8, u8),
    ACHAR3(u8, u8, u8),
    DOUBLEH(u32),
    DOUBLEL(u32),
    RESVD(u32),
    UDEF { value: u32, type_id: u8 },
}

impl DataType {
    pub fn type_id(&self) -> u8 {
        match *self {
            DataType::NODATA => 0x0,
            DataType::ERROR(_) => 0x1,
            DataType::FLOAT(_) => 0x2,
            DataType::LONG(_) => 0x3,
            DataType::ULONG(_) => 0x4,
            DataType::BLONG(_) => 0x5,
            DataType::SHORT(_) => 0x6,
            DataType::USHORT(_) => 0x7,
            DataType::BSHORT(_) => 0x8,
            DataType::CHAR(_) => 0x9,
            DataType::UCHAR(_) => 0xA,
            DataType::BCHAR(_) => 0xB,
            DataType::SHORT2(_, _) => 0xC,
            DataType::USHORT2(_, _) => 0xD,
            DataType::BSHORT2(_, _) => 0xE,
            DataType::CHAR4(_, _, _, _) => 0xF,
            DataType::UCHAR4(_, _, _, _) => 0x10,
            DataType::BCHAR4(_, _, _, _) => 0x11,
            DataType::CHAR2(_, _) => 0x12,
            DataType::UCHAR2(_, _) => 0x13,
            DataType::BCHAR2(_, _) => 0x14,
            DataType::MEMID(_) => 0x15,
            DataType::CHKSUM(_) => 0x16,
            DataType::ACHAR(_) => 0x17,
            DataType::ACHAR2(_, _) => 0x18,
            DataType::ACHAR4(_, _, _, _) => 0x19,
            DataType::CHAR3(_, _, _) => 0x1A,
            DataType::UCHAR3(_, _, _) => 0x1B,
            DataType::BCHAR3(_, _, _) => 0x1C,
            DataType::ACHAR3(_, _, _) => 0x1D,
            DataType::DOUBLEH(_) => 0x1E,
            DataType::DOUBLEL(_) => 0x1F,
            DataType::RESVD(_) => 0x20,
            DataType::UDEF { value: _, type_id } => type_id,
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == DataType::NODATA
    }

    pub fn len(&self) -> u8 {
        match *self {
            DataType::NODATA | DataType::RESVD(_) => 0,

            DataType::ERROR(_)
            | DataType::FLOAT(_)
            | DataType::LONG(_)
            | DataType::ULONG(_)
            | DataType::BLONG(_)
            | DataType::SHORT2(_, _)
            | DataType::USHORT2(_, _)
            | DataType::BSHORT2(_, _)
            | DataType::CHAR4(_, _, _, _)
            | DataType::UCHAR4(_, _, _, _)
            | DataType::BCHAR4(_, _, _, _)
            | DataType::MEMID(_)
            | DataType::CHKSUM(_)
            | DataType::ACHAR4(_, _, _, _)
            | DataType::DOUBLEH(_)
            | DataType::DOUBLEL(_) => 4,

            DataType::SHORT(_)
            | DataType::USHORT(_)
            | DataType::BSHORT(_)
            | DataType::CHAR2(_, _)
            | DataType::UCHAR2(_, _)
            | DataType::BCHAR2(_, _)
            | DataType::ACHAR2(_, _) => 2,

            DataType::CHAR(_) | DataType::UCHAR(_) | DataType::BCHAR(_) | DataType::ACHAR(_) => 1,

            DataType::CHAR3(_, _, _)
            | DataType::UCHAR3(_, _, _)
            | DataType::BCHAR3(_, _, _)
            | DataType::ACHAR3(_, _, _) => 3,

            DataType::UDEF {
                value: _,
                type_id: _,
            } => 4,
        }
    }

    pub fn to_be_bytes(self) -> [u8; 4] {
        match self {
            DataType::NODATA => [0, 0, 0, 0],
            DataType::ERROR(d) => d.to_be_bytes(),
            DataType::FLOAT(d) => d.to_be_bytes(),
            DataType::LONG(d) => d.to_be_bytes(),
            DataType::ULONG(d) => d.to_be_bytes(),
            DataType::BLONG(d) => d.to_be_bytes(),
            DataType::SHORT(d) => {
                let d_bytes = d.to_be_bytes();
                [d_bytes[0], d_bytes[1], 0, 0]
            }
            DataType::USHORT(d) | DataType::BSHORT(d) => {
                let d_bytes = d.to_be_bytes();
                [d_bytes[0], d_bytes[1], 0, 0]
            }
            DataType::CHAR(d) => [d as u8, 0, 0, 0],
            DataType::UCHAR(d) => [d, 0, 0, 0],
            DataType::BCHAR(d) => [d, 0, 0, 0],
            DataType::SHORT2(a, b) => {
                let a_bytes = a.to_be_bytes();
                let b_bytes = b.to_be_bytes();
                [a_bytes[0], a_bytes[1], b_bytes[0], b_bytes[1]]
            }
            DataType::USHORT2(a, b) => {
                let a_bytes = a.to_be_bytes();
                let b_bytes = b.to_be_bytes();
                [a_bytes[0], a_bytes[1], b_bytes[0], b_bytes[1]]
            }
            DataType::BSHORT2(a, b) => {
                let a_bytes = a.to_be_bytes();
                let b_bytes = b.to_be_bytes();
                [a_bytes[0], a_bytes[1], b_bytes[0], b_bytes[1]]
            }
            DataType::CHAR4(a, b, c, d) => [a as u8, b as u8, c as u8, d as u8],
            DataType::UCHAR4(a, b, c, d) => [a as u8, b as u8, c as u8, d as u8],
            DataType::BCHAR4(a, b, c, d) => [a as u8, b as u8, c as u8, d as u8],
            DataType::CHAR2(a, b) => [a as u8, b as u8, 0, 0],
            DataType::UCHAR2(a, b) => [a as u8, b as u8, 0, 0],
            DataType::BCHAR2(a, b) => [a as u8, b as u8, 0, 0],
            DataType::MEMID(d) => d.to_be_bytes(),
            DataType::CHKSUM(d) => d.to_be_bytes(),
            DataType::ACHAR(a) => [a, 0, 0, 0],
            DataType::ACHAR2(a, b) => [a, b, 0, 0],
            DataType::ACHAR4(a, b, c, d) => [a, b, c, d],
            DataType::CHAR3(a, b, c) => [a as u8, b as u8, c as u8, 0],
            DataType::UCHAR3(a, b, c) => [a, b, c, 0],
            DataType::BCHAR3(a, b, c) => [a, b, c, 0],
            DataType::ACHAR3(a, b, c) => [a, b, c, 0],
            DataType::DOUBLEH(d) => d.to_be_bytes(),
            DataType::DOUBLEL(d) => d.to_be_bytes(),
            DataType::RESVD(d) => d.to_be_bytes(),
            DataType::UDEF { value, type_id: _ } => value.to_be_bytes(),
        }
    }
}

fn as_u32(arr: &[u8]) -> u32 {
    u32::from_be_bytes(arr[..4].try_into().unwrap())
}

fn as_u16(arr: &[u8]) -> u16 {
    u16::from_be_bytes(arr[..2].try_into().unwrap())
}

impl From<(u8, &[u8])> for DataType {
    fn from(data: (u8, &[u8])) -> Self {
        let (t, arr) = data;
        match t {
            0x0 => DataType::NODATA,
            0x1 => DataType::ERROR(as_u32(arr)),
            0x2 => DataType::FLOAT(as_u32(arr) as f32),
            0x3 => DataType::LONG(as_u32(arr) as i32),
            0x4 => DataType::ULONG(as_u32(arr)),
            0x5 => DataType::BLONG(as_u32(arr)),
            0x6 => DataType::SHORT(as_u16(arr) as i16),
            0x7 => DataType::USHORT(as_u16(arr)),
            0x8 => DataType::BSHORT(as_u16(arr)),
            0x9 => DataType::CHAR(arr[0] as i8),
            0xA => DataType::UCHAR(arr[0] as u8),
            0xB => DataType::BCHAR(arr[0] as u8),
            0xC => DataType::SHORT2(as_u16(arr) as i16, as_u16(&arr[2..]) as i16),
            0xD => DataType::USHORT2(as_u16(arr), as_u16(arr)),
            0xE => DataType::BSHORT2(as_u16(arr), as_u16(&arr[2..])),
            0xF => DataType::CHAR4(arr[0] as i8, arr[1] as i8, arr[2] as i8, arr[3] as i8),
            0x10 => DataType::UCHAR4(arr[0], arr[1], arr[2], arr[3]),
            0x11 => DataType::BCHAR4(arr[0], arr[1], arr[2], arr[3]),
            0x12 => DataType::CHAR2(arr[0] as i8, arr[1] as i8),
            0x13 => DataType::UCHAR2(arr[0], arr[1]),
            0x14 => DataType::BCHAR2(arr[0], arr[1]),
            0x15 => DataType::MEMID(as_u32(arr)),
            0x16 => DataType::CHKSUM(as_u32(arr)),
            0x17 => DataType::ACHAR(arr[0]),
            0x18 => DataType::ACHAR2(arr[0], arr[1]),
            0x19 => DataType::ACHAR4(arr[0], arr[1], arr[2], arr[3]),
            0x1A => DataType::CHAR3(arr[0] as i8, arr[1] as i8, arr[2] as i8),
            0x1B => DataType::UCHAR3(arr[0], arr[1], arr[2]),
            0x1C => DataType::BCHAR3(arr[0], arr[1], arr[2]),
            0x1D => DataType::ACHAR3(arr[0], arr[1], arr[2]),
            0x1E => DataType::DOUBLEH(as_u32(arr)),
            0x1F => DataType::DOUBLEL(as_u32(arr)),
            0x20 => DataType::RESVD(as_u32(arr)),
            _ => DataType::UDEF {
                value: as_u32(arr),
                type_id: t,
            },
        }
    }
}

impl From<&RawMessage> for DataType {
    fn from(message: &RawMessage) -> Self {
        DataType::from((message.data_type, &message.payload.data[..]))
    }
}
