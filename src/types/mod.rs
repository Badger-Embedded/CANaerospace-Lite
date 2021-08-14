use crate::message::RawMessage;


pub type MessageCode = u8;
pub type ServiceCode = u8;

#[derive(Clone, Debug)]
pub enum ServiceCodeEnum {
    IDS = 0x0,
    NSS = 0x1,
    DDS = 0x2,
    DUS = 0x3,
    SCS = 0x4,
    TIS = 0x5,
    FPS = 0x6,
    STS = 0x7,
    FSS = 0x8,
    TCS = 0x9,
    BSS = 0xA,
    NIS = 0xB,
    MIS = 0xC,
    MCS = 0xD,
    CSS = 0xE,
    DSS = 0xF
}



#[derive(Clone, Debug)]
pub enum MessageType {
    /// Emergency Event Data \[0,127\]
    /// Transmitted asynchronously whenever a situation requiring immediate action occurs.
    EED(u16),
    /// High Priority Node Service Data [128,199]
    /// Transmitted asynchronously or cyclic with defined transmission intervals for operational commands (36 channels)
    NSH(u16),
    /// High Priority User-Defined Data [200,299]
    /// Message/data format and transmission intervals entirely user-defined
    UDH(u16),
    /// Normal Operation Data [300,1799]
    /// Transmitted asynchronously or cyclic with defined transmission intervals for operational and status data.
    NOD(u16),
    /// Low Priority User-Defined Data [1800,1899]
    /// Message/data format and transmission intervals entirely user-defined
    UDL(u16),
    /// Debug Service Data [1900,1999]
    /// Transmitted asynchronously or cyclic for debug communication & software download actions.
    DSD(u16),
    /// Low Priority Node Service Data [2000,2031]
    /// Transmitted asynchronously or cyclic for test & maintenance actions (16 channels).
    NSL(u16),
    INVALID
}

impl MessageType {
    pub fn id(&self) -> u16 {
        match *self {
            MessageType::EED(id) => {
                if id > 0x7F {
                    panic!("CANaerospace: Invalid Message ID({}) for EED Message.", id);
                }
                id
            },
            MessageType::NSH(id) => {
                if id < 0x80 || id > 0xC7 {
                    panic!("CANaerospace: Invalid Message ID({}) for NSH Message.", id);
                }
                id
            },
            MessageType::UDH(id) => {
                if id < 0xC8 || id > 0x12B {
                    panic!("CANaerospace: Invalid Message ID({}) for UDH Message.", id);
                }
                id
            },
            MessageType::NOD(id) => {
                if id < 0x12C || id > 0x707 {
                    panic!("CANaerospace: Invalid Message ID({}) for NOD Message.", id);
                }
                id
            },
            MessageType::UDL(id) => {
                if id < 0x708 || id > 0x76B {
                    panic!("CANaerospace: Invalid Message ID({}) for UDL Message.", id);
                }
                id
            },
            MessageType::DSD(id) => {
                if id < 0x76C || id > 0x7CF {
                    panic!("CANaerospace: Invalid Message ID({}) for DSD Message.", id);
                }
                id
            },
            MessageType::NSL(id) => {
                if id < 0x7D0 || id > 0x7EF {
                    panic!("CANaerospace: Invalid Message ID({}) for NSL Message.", id);
                }
                id
            },
            MessageType::INVALID => u16::MAX,
        }        
    }
}

impl From<u16> for MessageType {
    fn from(raw_id: u16) -> Self {
        match raw_id {
            0..=127     => MessageType::EED(raw_id),
            128..=199   => MessageType::NSH(raw_id),
            200..=299   => MessageType::UDH(raw_id),
            300..=1799  => MessageType::NOD(raw_id),
            1800..=1899 => MessageType::UDL(raw_id),
            1900..=1999 => MessageType::DSD(raw_id),
            2000..=2031 => MessageType::NSL(raw_id),
            _ => MessageType::INVALID
        }
    }
}

#[derive(Clone, Debug)]
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
    SHORT2(i16,i16),
    USHORT2(u16,u16),
    BSHORT2(u16,u16),
    CHAR4(i8,i8,i8,i8),
    UCHAR4(u8,u8,u8,u8),
    BCHAR4(u8,u8,u8,u8),
    CHAR2(i8,i8),
    UCHAR2(u8,u8),
    BCHAR2(u8,u8),
    MEMID(u32),
    CHKSUM(u32),
    ACHAR(u8),
    ACHAR2(u8,u8),
    ACHAR4(u8,u8,u8,u8),
    CHAR3(i8,i8,i8),
    UCHAR3(u8,u8,u8),
    BCHAR3(u8,u8,u8),
    ACHAR3(u8,u8,u8),
    DOUBLEH(u32),
    DOUBLEL(u32),
    RESVD(u32),
    UDEF{value:u32, type_id:u8}
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

    pub fn to_be_bytes(&self) -> [u8; 4] {
        match *self {
            DataType::NODATA => [0, 0, 0, 0],
            DataType::ERROR(d) => d.to_be_bytes(),
            DataType::FLOAT(d) => d.to_be_bytes(),
            DataType::LONG(d) => d.to_be_bytes(),
            DataType::ULONG(d) => d.to_be_bytes(),
            DataType::BLONG(d) => d.to_be_bytes(),
            DataType::SHORT(d) => {
                let d_bytes = d.to_be_bytes();
                [d_bytes[0], d_bytes[1], 0, 0]
            },
            DataType::USHORT(d) | DataType::BSHORT(d) => {
                let d_bytes = d.to_be_bytes();
                [d_bytes[0], d_bytes[1], 0, 0]
            },
            DataType::CHAR(d) => [d as u8, 0, 0, 0],
            DataType::UCHAR(d) => [d, 0, 0, 0],
            DataType::BCHAR(d) => [d, 0, 0, 0],
            DataType::SHORT2(a, b) => {
                let a_bytes = a.to_be_bytes();
                let b_bytes = b.to_be_bytes();
                [a_bytes[0], a_bytes[1], b_bytes[0], b_bytes[1]]
            },
            DataType::USHORT2(a, b) => {
                let a_bytes = a.to_be_bytes();
                let b_bytes = b.to_be_bytes();
                [a_bytes[0], a_bytes[1], b_bytes[0], b_bytes[1]]
            },
            DataType::BSHORT2(a, b) => {
                let a_bytes = a.to_be_bytes();
                let b_bytes = b.to_be_bytes();
                [a_bytes[0], a_bytes[1], b_bytes[0], b_bytes[1]]
            },
            DataType::CHAR4(a, b, c, d) => [a as u8,b as u8,c as u8,d as u8],
            DataType::UCHAR4(a, b, c, d) => [a as u8,b as u8,c as u8,d as u8],
            DataType::BCHAR4(a, b, c, d) => [a as u8,b as u8,c as u8,d as u8],
            DataType::CHAR2(a, b) => [a as u8,b as u8, 0, 0],
            DataType::UCHAR2(a, b) => [a as u8,b as u8, 0, 0],
            DataType::BCHAR2(a, b) => [a as u8,b as u8, 0, 0],
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
            DataType::UDEF { value, type_id: _ } => value.to_be_bytes()
        }
    }

}

fn as_u32(arr: &[u8]) -> u32 {
    ((arr[0] as u32) << 24) + ((arr[1] as u32) << 16) + ((arr[2] as u32) << 8) + ((arr[3] as u32) << 0)
}

fn as_u16(arr: &[u8]) -> u16 {
    ((arr[0] as u16) << 8) + ((arr[1] as u16) << 0)
}

impl From<(u8, &[u8])> for DataType {
    fn from(data: (u8, &[u8])) -> Self {
        let (t, arr) = data;
        match t {
            0x0     => DataType::NODATA,
            0x1     => DataType::ERROR(as_u32(arr)),
            0x2     => DataType::FLOAT(as_u32(arr) as f32),
            0x3     => DataType::LONG(as_u32(arr) as i32),
            0x4     => DataType::ULONG(as_u32(arr)),
            0x5     => DataType::BLONG(as_u32(arr)),
            0x6     => DataType::SHORT(as_u16(arr) as i16),
            0x7     => DataType::USHORT(as_u16(arr)),
            0x8     => DataType::BSHORT(as_u16(arr)),
            0x9     => DataType::CHAR(arr[0] as i8),
            0xA     => DataType::UCHAR(arr[0] as u8),
            0xB     => DataType::BCHAR(arr[0] as u8),
            0xC     => DataType::SHORT2(as_u16(arr) as i16, as_u16(&arr[2..]) as i16),
            0xD     => DataType::USHORT2(as_u16(arr), as_u16(arr)),
            0xE     => DataType::BSHORT2(as_u16(arr), as_u16(&arr[2..])),
            0xF     => DataType::CHAR4(arr[0] as i8, arr[1] as i8, arr[2] as i8, arr[3] as i8),
            0x10    => DataType::UCHAR4(arr[0], arr[1], arr[2], arr[3]),
            0x11    => DataType::BCHAR4(arr[0], arr[1], arr[2], arr[3]),
            0x12    => DataType::CHAR2(arr[0] as i8, arr[1] as i8),
            0x13    => DataType::UCHAR2(arr[0], arr[1]),
            0x14    => DataType::BCHAR2(arr[0], arr[1]),
            0x15    => DataType::MEMID(as_u32(arr)),
            0x16    => DataType::CHKSUM(as_u32(arr)),
            0x17    => DataType::ACHAR(arr[0]),
            0x18    => DataType::ACHAR2(arr[0], arr[1]),
            0x19    => DataType::ACHAR4(arr[0], arr[1], arr[2], arr[3]),
            0x1A    => DataType::CHAR3(arr[0] as i8, arr[1] as i8, arr[2] as i8),
            0x1B    => DataType::UCHAR3(arr[0], arr[1], arr[2]),
            0x1C    => DataType::BCHAR3(arr[0], arr[1], arr[2]),
            0x1D    => DataType::ACHAR3(arr[0], arr[1], arr[2]),
            0x1E    => DataType::DOUBLEH(as_u32(arr)),
            0x1F    => DataType::DOUBLEL(as_u32(arr)),
            0x20    => DataType::RESVD(as_u32(arr)),
            _       => DataType::UDEF { value: as_u32(arr), type_id: t }
        }
    }
}

impl From<&RawMessage> for DataType {
    fn from(message: &RawMessage) -> Self {
        DataType::from((message.data_type, &message.payload.data[..]))
    }
}
