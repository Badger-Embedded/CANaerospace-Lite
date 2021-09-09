#[cfg(test)]
mod servicecodeenum {
    use crate::types::ServiceCodeEnum;

    #[test]
    fn test_from_u8() {
        let mut code;
        code = ServiceCodeEnum::from(0x0);
        assert_eq!(code, ServiceCodeEnum::IDS);
        code = ServiceCodeEnum::from(0x1);
        assert_eq!(code, ServiceCodeEnum::NSS);
        code = ServiceCodeEnum::from(0x2);
        assert_eq!(code, ServiceCodeEnum::DDS);
        code = ServiceCodeEnum::from(0x3);
        assert_eq!(code, ServiceCodeEnum::DUS);
        code = ServiceCodeEnum::from(0x4);
        assert_eq!(code, ServiceCodeEnum::SCS);
        code = ServiceCodeEnum::from(0x5);
        assert_eq!(code, ServiceCodeEnum::TIS);
        code = ServiceCodeEnum::from(0x6);
        assert_eq!(code, ServiceCodeEnum::FPS);
        code = ServiceCodeEnum::from(0x7);
        assert_eq!(code, ServiceCodeEnum::STS);
        code = ServiceCodeEnum::from(0x8);
        assert_eq!(code, ServiceCodeEnum::FSS);
        code = ServiceCodeEnum::from(0x9);
        assert_eq!(code, ServiceCodeEnum::TCS);
        code = ServiceCodeEnum::from(0xA);
        assert_eq!(code, ServiceCodeEnum::BSS);
        code = ServiceCodeEnum::from(0xB);
        assert_eq!(code, ServiceCodeEnum::NIS);
        code = ServiceCodeEnum::from(0xC);
        assert_eq!(code, ServiceCodeEnum::MIS);
        code = ServiceCodeEnum::from(0xD);
        assert_eq!(code, ServiceCodeEnum::MCS);
        code = ServiceCodeEnum::from(0xE);
        assert_eq!(code, ServiceCodeEnum::CSS);
        code = ServiceCodeEnum::from(0xF);
        assert_eq!(code, ServiceCodeEnum::DSS);
        // code = ServiceCodeEnum::from(0xFF);
        // assert_ne!(code, ServiceCodeEnum::UNKNOWN)
    }
}
#[cfg(test)]
mod messagetype {

    use crate::types::MessageType;
    #[test]
    fn test_id() {
        for i in 0..=127 {
            let m = MessageType::EED(i as u16);
            assert_eq!(m.id(), i);
        }
        for i in 128..=199 {
            let m = MessageType::NSH(i as u16);
            assert_eq!(m.id(), i);
        }
        for i in 200..=299 {
            let m = MessageType::UDH(i as u16);
            assert_eq!(m.id(), i);
        }
        for i in 300..=1799 {
            let m = MessageType::NOD(i as u16);
            assert_eq!(m.id(), i);
        }
        for i in 1800..=1899 {
            let m = MessageType::UDL(i as u16);
            assert_eq!(m.id(), i);
        }
        for i in 1900..=1999 {
            let m = MessageType::DSD(i as u16);
            assert_eq!(m.id(), i);
        }
        for i in 2000..=2031 {
            let m = MessageType::NSL(i as u16);
            assert_eq!(m.id(), i);
        }
    }

    #[test]
    #[should_panic]
    fn test_id_eed_invalid() {
        let _ = MessageType::EED(128).id();
    }

    #[test]
    #[should_panic]
    fn test_id_nsh_invalid_less() {
        let _ = MessageType::NSH(127).id();
    }
    #[test]
    #[should_panic]
    fn test_id_nsh_invalid_greater() {
        let _ = MessageType::NSH(200).id();
    }
    #[test]
    #[should_panic]
    fn test_id_udh_invalid_less() {
        let _ = MessageType::UDH(199).id();
    }
    #[test]
    #[should_panic]
    fn test_id_udh_invalid_greater() {
        let _ = MessageType::UDH(300).id();
    }
    #[test]
    #[should_panic]
    fn test_id_nod_invalid_less() {
        let _ = MessageType::NOD(299).id();
    }
    #[test]
    #[should_panic]
    fn test_id_nod_invalid_greater() {
        let _ = MessageType::NOD(1800).id();
    }
    #[test]
    #[should_panic]
    fn test_id_udl_invalid_less() {
        let _ = MessageType::UDL(1799).id();
    }
    #[test]
    #[should_panic]
    fn test_id_udl_invalid_greater() {
        let _ = MessageType::UDL(1900).id();
    }
    #[test]
    #[should_panic]
    fn test_id_dsd_invalid_less() {
        let _ = MessageType::DSD(1899).id();
    }
    #[test]
    #[should_panic]
    fn test_id_dsd_invalid_greater() {
        let _ = MessageType::DSD(2000).id();
    }
    #[test]
    #[should_panic]
    fn test_id_nsl_invalid_less() {
        let _ = MessageType::NSL(1999).id();
    }
    #[test]
    #[should_panic]
    fn test_id_nsl_invalid_greater() {
        let _ = MessageType::NSL(2032).id();
    }

    #[test]
    fn test_from_u16() {
        for i in 0..=127 {
            let m = MessageType::from(i as u16);
            assert_eq!(m, MessageType::EED(i as u16));
        }
        for i in 128..=199 {
            let m = MessageType::from(i as u16);
            assert_eq!(m, MessageType::NSH(i as u16));
        }
        for i in 200..=299 {
            let m = MessageType::from(i as u16);
            assert_eq!(m, MessageType::UDH(i as u16));
        }
        for i in 300..=1799 {
            let m = MessageType::from(i as u16);
            assert_eq!(m, MessageType::NOD(i as u16));
        }
        for i in 1800..=1899 {
            let m = MessageType::from(i as u16);
            assert_eq!(m, MessageType::UDL(i as u16));
        }
        for i in 1900..=1999 {
            let m = MessageType::from(i as u16);
            assert_eq!(m, MessageType::DSD(i as u16));
        }
        for i in 2000..=2031 {
            let m = MessageType::from(i as u16);
            assert_eq!(m, MessageType::NSL(i as u16));
        }
    }

    #[test]
    fn test_from_u16_invalid() {
        let m = MessageType::from(2032);
        assert_eq!(m, MessageType::INVALID);
    }
}
#[cfg(test)]
mod datatypes {
    use crate::types::DataType;

    #[test]
    fn test_type_id() {
        let dt_nodata = DataType::NODATA;
        assert_eq!(dt_nodata.type_id(), 0x0);

        let dt_err = DataType::ERROR(0);
        assert_eq!(dt_err.type_id(), 0x1);

        let dt_fl = DataType::FLOAT(0.0);
        assert_eq!(dt_fl.type_id(), 0x2);

        let dt_l = DataType::LONG(0);
        assert_eq!(dt_l.type_id(), 0x3);

        let dt_ul = DataType::ULONG(0);
        assert_eq!(dt_ul.type_id(), 0x4);

        let dt_bl = DataType::BLONG(0);
        assert_eq!(dt_bl.type_id(), 0x5);

        let dt_sh = DataType::SHORT(0);
        assert_eq!(dt_sh.type_id(), 0x6);

        let dt_ush = DataType::USHORT(0);
        assert_eq!(dt_ush.type_id(), 0x7);

        let dt_bsh = DataType::BSHORT(0);
        assert_eq!(dt_bsh.type_id(), 0x8);

        let dt_ch = DataType::CHAR(0);
        assert_eq!(dt_ch.type_id(), 0x9);

        let dt_uch = DataType::UCHAR(0);
        assert_eq!(dt_uch.type_id(), 0xA);

        let dt_bch = DataType::BCHAR(0);
        assert_eq!(dt_bch.type_id(), 0xB);

        let dt_sh2 = DataType::SHORT2(0, 0);
        assert_eq!(dt_sh2.type_id(), 0xC);

        let dt_ush2 = DataType::USHORT2(0, 0);
        assert_eq!(dt_ush2.type_id(), 0xD);

        let dt_bsh2 = DataType::BSHORT2(0, 0);
        assert_eq!(dt_bsh2.type_id(), 0xE);

        let dt_ch4 = DataType::CHAR4(0, 0, 0, 0);
        assert_eq!(dt_ch4.type_id(), 0xF);

        let dt_uch4 = DataType::UCHAR4(0, 0, 0, 0);
        assert_eq!(dt_uch4.type_id(), 0x10);

        let dt_bch4 = DataType::BCHAR4(0, 0, 0, 0);
        assert_eq!(dt_bch4.type_id(), 0x11);

        let dt_ch2 = DataType::CHAR2(0, 0);
        assert_eq!(dt_ch2.type_id(), 0x12);

        let dt_uch2 = DataType::UCHAR2(0, 0);
        assert_eq!(dt_uch2.type_id(), 0x13);

        let dt_bch2 = DataType::BCHAR2(0, 0);
        assert_eq!(dt_bch2.type_id(), 0x14);

        let dt_mmd = DataType::MEMID(0);
        assert_eq!(dt_mmd.type_id(), 0x15);

        let dt_chksum = DataType::CHKSUM(0);
        assert_eq!(dt_chksum.type_id(), 0x16);

        let dt_ach = DataType::ACHAR(0);
        assert_eq!(dt_ach.type_id(), 0x17);

        let dt_ach2 = DataType::ACHAR2(0, 0);
        assert_eq!(dt_ach2.type_id(), 0x18);

        let dt_ach4 = DataType::ACHAR4(0, 0, 0, 0);
        assert_eq!(dt_ach4.type_id(), 0x19);

        let dt_ch3 = DataType::CHAR3(0, 0, 0);
        assert_eq!(dt_ch3.type_id(), 0x1A);

        let dt_uch3 = DataType::UCHAR3(0, 0, 0);
        assert_eq!(dt_uch3.type_id(), 0x1B);

        let dt_bch3 = DataType::BCHAR3(0, 0, 0);
        assert_eq!(dt_bch3.type_id(), 0x1C);

        let dt_ach3 = DataType::ACHAR3(0, 0, 0);
        assert_eq!(dt_ach3.type_id(), 0x1D);

        let dt_dbh = DataType::DOUBLEH(0);
        assert_eq!(dt_dbh.type_id(), 0x1E);

        let dt_dbl = DataType::DOUBLEL(0);
        assert_eq!(dt_dbl.type_id(), 0x1F);

        let dt_resvd = DataType::RESVD(0);
        assert_eq!(dt_resvd.type_id(), 0x20);

        let dt_udef = DataType::UDEF {
            value: 0,
            type_id: 0x23,
        };
        assert_eq!(dt_udef.type_id(), 0x23);
    }

    #[test]
    fn test_is_empty_true() {
        let dt_nodata = DataType::NODATA;
        assert_eq!(dt_nodata.is_empty(), true);
    }

    #[test]
    fn test_is_empty_false() {
        let dt_err = DataType::ERROR(0);
        assert_eq!(dt_err.is_empty(), false);
    }

    #[test]
    fn test_len() {
        let dt_nodata = DataType::NODATA;
        assert_eq!(dt_nodata.len(), 0);

        let dt_resvd = DataType::RESVD(0);
        assert_eq!(dt_resvd.len(), 0);

        let dt_err = DataType::ERROR(0);
        assert_eq!(dt_err.len(), 4);

        let dt_fl = DataType::FLOAT(0.0);
        assert_eq!(dt_fl.len(), 4);

        let dt_l = DataType::LONG(0);
        assert_eq!(dt_l.len(), 4);

        let dt_ul = DataType::ULONG(0);
        assert_eq!(dt_ul.len(), 4);

        let dt_bl = DataType::BLONG(0);
        assert_eq!(dt_bl.len(), 4);

        let dt_sh2 = DataType::SHORT2(0, 0);
        assert_eq!(dt_sh2.len(), 4);

        let dt_ush2 = DataType::USHORT2(0, 0);
        assert_eq!(dt_ush2.len(), 4);

        let dt_bsh2 = DataType::BSHORT2(0, 0);
        assert_eq!(dt_bsh2.len(), 4);

        let dt_ch4 = DataType::CHAR4(0, 0, 0, 0);
        assert_eq!(dt_ch4.len(), 4);

        let dt_uch4 = DataType::UCHAR4(0, 0, 0, 0);
        assert_eq!(dt_uch4.len(), 4);

        let dt_bch4 = DataType::BCHAR4(0, 0, 0, 0);
        assert_eq!(dt_bch4.len(), 4);

        let dt_mmd = DataType::MEMID(0);
        assert_eq!(dt_mmd.len(), 4);

        let dt_chksum = DataType::CHKSUM(0);
        assert_eq!(dt_chksum.len(), 4);

        let dt_ach4 = DataType::ACHAR4(0, 0, 0, 0);
        assert_eq!(dt_ach4.len(), 4);

        let dt_dbh = DataType::DOUBLEH(0);
        assert_eq!(dt_dbh.len(), 4);

        let dt_dbl = DataType::DOUBLEL(0);
        assert_eq!(dt_dbl.len(), 4);

        let dt_sh = DataType::SHORT(0);
        assert_eq!(dt_sh.len(), 2);

        let dt_ush = DataType::USHORT(0);
        assert_eq!(dt_ush.len(), 2);

        let dt_bsh = DataType::BSHORT(0);
        assert_eq!(dt_bsh.len(), 2);

        let dt_ch2 = DataType::CHAR2(0, 0);
        assert_eq!(dt_ch2.len(), 2);

        let dt_uch2 = DataType::UCHAR2(0, 0);
        assert_eq!(dt_uch2.len(), 2);

        let dt_bch2 = DataType::BCHAR2(0, 0);
        assert_eq!(dt_bch2.len(), 2);

        let dt_ach2 = DataType::ACHAR2(0, 0);
        assert_eq!(dt_ach2.len(), 2);

        let dt_ch = DataType::CHAR(0);
        assert_eq!(dt_ch.len(), 1);

        let dt_uch = DataType::UCHAR(0);
        assert_eq!(dt_uch.len(), 1);

        let dt_bch = DataType::BCHAR(0);
        assert_eq!(dt_bch.len(), 1);

        let dt_ach = DataType::ACHAR(0);
        assert_eq!(dt_ach.len(), 1);

        let dt_ch3 = DataType::CHAR3(0, 0, 0);
        assert_eq!(dt_ch3.len(), 3);

        let dt_uch3 = DataType::UCHAR3(0, 0, 0);
        assert_eq!(dt_uch3.len(), 3);

        let dt_bch3 = DataType::BCHAR3(0, 0, 0);
        assert_eq!(dt_bch3.len(), 3);

        let dt_ach3 = DataType::ACHAR3(0, 0, 0);
        assert_eq!(dt_ach3.len(), 3);

        let dt_udef = DataType::UDEF {
            value: 0,
            type_id: 0x23,
        };
        assert_eq!(dt_udef.len(), 4);
    }

    #[test]
    fn test_to_be_bytes() {
        let dt_nodata = DataType::NODATA;
        assert_eq!(dt_nodata.to_be_bytes(), [0, 0, 0, 0]);

        let dt_err = DataType::ERROR(55495);
        assert_eq!(dt_err.to_be_bytes(), [0, 0, 216, 199]);

        let dt_float = DataType::FLOAT(12.5);
        assert_eq!(dt_float.to_be_bytes(), [65, 72, 0, 0]);

        let dt_l = DataType::LONG(0x12345678);
        assert_eq!(dt_l.to_be_bytes(), [18, 52, 86, 120]);

        let dt_l2 = DataType::LONG(-125);
        assert_eq!(dt_l2.to_be_bytes(), [255, 255, 255, 131]);

        let dt_ul = DataType::ULONG(3158675557);
        assert_eq!(dt_ul.to_be_bytes(), [188, 69, 144, 101]);

        let dt_bl = DataType::BLONG(3158675558);
        assert_eq!(dt_bl.to_be_bytes(), [188, 69, 144, 102]);

        let dt_sh = DataType::SHORT(-8095);
        assert_eq!(dt_sh.to_be_bytes(), [224, 97, 0, 0]);

        let dt_ush = DataType::USHORT(8095);
        assert_eq!(dt_ush.to_be_bytes(), [31, 159, 0, 0]);

        let dt_ch = DataType::CHAR(-80);
        assert_eq!(dt_ch.to_be_bytes(), [0xB0, 0, 0, 0]);

        let dt_uch = DataType::UCHAR(80);
        assert_eq!(dt_uch.to_be_bytes(), [80, 0, 0, 0]);

        let dt_bch = DataType::BCHAR(255);
        assert_eq!(dt_bch.to_be_bytes(), [0xFF, 0, 0, 0]);

        let dt_sh2 = DataType::SHORT2(3520, 255);
        assert_eq!(dt_sh2.to_be_bytes(), [13, 192, 0, 255]);

        let dt_ush2 = DataType::USHORT2(3520, 255);
        assert_eq!(dt_ush2.to_be_bytes(), [13, 192, 0, 255]);

        let dt_bsh2 = DataType::BSHORT2(3520, 255);
        assert_eq!(dt_bsh2.to_be_bytes(), [13, 192, 0, 255]);

        let dt_ch4 = DataType::CHAR4(0x1E, 0x14, 0xF, 0x28);
        assert_eq!(dt_ch4.to_be_bytes(), [30, 20, 15, 40]);

        let dt_uch4 = DataType::UCHAR4(0xA, 0xB, 0xF, 0x28);
        assert_eq!(dt_uch4.to_be_bytes(), [10, 11, 15, 40]);

        let dt_bch4 = DataType::BCHAR4(0xA, 0xB, 0xF, 0x28);
        assert_eq!(dt_bch4.to_be_bytes(), [10, 11, 15, 40]);

        let dt_ch2 = DataType::CHAR2(14, 23);
        assert_eq!(dt_ch2.to_be_bytes(), [0xE, 0x17, 0, 0]);

        let dt_uch2 = DataType::UCHAR2(14, 23);
        assert_eq!(dt_uch2.to_be_bytes(), [0xE, 0x17, 0, 0]);

        let dt_bch2 = DataType::UCHAR2(14, 23);
        assert_eq!(dt_bch2.to_be_bytes(), [0xE, 0x17, 0, 0]);

        let dt_memid = DataType::MEMID(0x40587580);
        assert_eq!(dt_memid.to_be_bytes(), [64, 88, 117, 128]);

        let dt_chcksum = DataType::CHKSUM(0x41405152);
        assert_eq!(dt_chcksum.to_be_bytes(), [65, 64, 81, 82]);

        let dt_ach = DataType::ACHAR(0x30);
        assert_eq!(dt_ach.to_be_bytes(), [48, 0, 0, 0]);

        let dt_ach2 = DataType::ACHAR2(0x30, 0x50);
        assert_eq!(dt_ach2.to_be_bytes(), [48, 80, 0, 0]);

        let dt_ach4 = DataType::ACHAR4(0x30, 0x50, 0x80, 0x1);
        assert_eq!(dt_ach4.to_be_bytes(), [48, 80, 128, 1]);

        let dt_ch3 = DataType::CHAR3(0x30, 0x12, 0x3);
        assert_eq!(dt_ch3.to_be_bytes(), [48, 18, 3, 0]);

        let dt_uch3 = DataType::UCHAR3(0x30, 0x50, 0x40);
        assert_eq!(dt_uch3.to_be_bytes(), [48, 80, 64, 0]);

        let dt_bch3 = DataType::BCHAR3(0x30, 0x50, 0x80);
        assert_eq!(dt_bch3.to_be_bytes(), [48, 80, 128, 0]);

        let dt_ach3 = DataType::ACHAR3(0x30, 0x12, 0x3);
        assert_eq!(dt_ach3.to_be_bytes(), [48, 18, 3, 0]);

        let dt_dbh = DataType::DOUBLEH(0x31203);
        assert_eq!(dt_dbh.to_be_bytes(), [0, 3, 18, 3]);

        let dt_dbl = DataType::DOUBLEL(0xBCDE);
        assert_eq!(dt_dbl.to_be_bytes(), [0, 0, 188, 222]);

        let dt_resvd = DataType::RESVD(0xBCDE);
        assert_eq!(dt_resvd.to_be_bytes(), [0, 0, 188, 222]);
    }
}
