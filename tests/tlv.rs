#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding_derive;
#[macro_use]
extern crate strict_encoding_test;

mod common;
use std::collections::BTreeMap;

use common::{compile_test, Error, Result};
use internet2::tlv;
use strict_encoding::{strict_deserialize, StrictDecode};
use strict_encoding_test::test_encoding_roundtrip;

#[test]
#[should_panic]
fn tlv_no_strict() { compile_test("tlv-failures/no_strict"); }

#[test]
#[should_panic]
fn tlv_no_enums() { compile_test("tlv-failures/no_enums"); }

#[test]
#[should_panic]
fn tlv_no_unions() { compile_test("tlv-failures/no_unions"); }

#[test]
#[should_panic]
fn tlv_non_u16_type() { compile_test("tlv-failures/tlv_non_u16_type"); }

#[test]
#[should_panic]
fn tlv_undeclared() { compile_test("tlv-failures/tlv_undeclared"); }

#[test]
#[should_panic]
fn wrong_unknown_type() { compile_test("tlv-failures/wrong_unknown_type"); }

const TLV_U32: u32 = 0xDEADCAFE;
macro_rules! tlv_u32 {
    () => {
        [
            // Count of TLV elements:
            0x01, 0x00, //
            // Type field:
            0xEF, 0xBE, //
            // Length:
            0x04, 0x00, //
            // Value field:
            0xFE, 0xCA, 0xAD, 0xDE,
        ]
    };
}

#[test]
fn tlv_optional() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(NetworkEncode, NetworkDecode)]
    #[network_encoding(use_tlv)]
    struct Tlv {
        fixed: u8,

        #[network_encoding(tlv = 0xBEEF)]
        tlv: Option<u32>,
    }

    test_encoding_roundtrip(
        &Tlv {
            fixed: 0xDD,
            tlv: None,
        },
        &[0xDD, 0x00, 0x00],
    )?;
    test_encoding_roundtrip(
        &Tlv {
            fixed: 0xDD,
            tlv: Some(TLV_U32),
        },
        vec![0xDD]
            .iter()
            .chain(&tlv_u32!()[..])
            .cloned()
            .collect::<Vec<_>>(),
    )
    .map_err(Error::from)
}

#[test]
fn tlv_newtype() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(NetworkEncode, NetworkDecode)]
    #[network_encoding(use_tlv)]
    struct Tlv(#[network_encoding(tlv = 0xBEEF)] Option<u32>);

    test_encoding_roundtrip(&Tlv(None), &[0x00, 0x00])?;
    //    println!("{:02x?}", Tlv::strict_deserialize(tlv_u32!())?);
    test_encoding_roundtrip(&Tlv(Some(TLV_U32)), tlv_u32!())
        .map_err(Error::from)
}

#[test]
fn tlv_default() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(NetworkEncode, NetworkDecode)]
    #[network_encoding(use_tlv)]
    struct TlvDefault {
        fixed: u8,

        #[network_encoding(tlv = 0xBEEF)]
        tlv: Vec<u8>,
    }

    test_encoding_roundtrip(&TlvDefault::default(), &[0x00; 3])?;

    test_encoding_roundtrip(
        &TlvDefault {
            fixed: 0xDD,
            tlv: TLV_U32.to_le_bytes().to_vec(),
        },
        vec![
            0xdd, // =fixed
            0x01, 0x00, // # of TLVs
            0xef, 0xbe, // TLV type
            0x06, 0x00, // TLV length
            0x04, 0x00, 0xfe, 0xca, 0xad, 0xde, // Value: length + vec
        ],
    )
    .map_err(Error::from)
}

#[test]
fn tlv_ordering() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(NetworkEncode, NetworkDecode)]
    #[network_encoding(use_tlv)]
    struct Tlv {
        #[network_encoding(tlv = 0xCAFE)]
        second: Option<u8>,

        #[network_encoding(tlv = 0xBAD)]
        first: Option<u8>,
    }

    let data = [
        // Count of TLV fields
        0x02, 0x00, //
        // First goes first
        0xAD, 0x0B, // type
        0x01, 0x00, // length
        0xA1, // value
        // Second goes second
        0xFE, 0xCA, // type
        0x01, 0x00, // length
        0xA2, // value
    ];
    let _: Tlv = strict_deserialize(&data).unwrap();

    test_encoding_roundtrip(&Tlv::default(), &[0x00; 2])?;
    test_encoding_roundtrip(
        &Tlv {
            second: Some(0xA2),
            first: Some(0xA1),
        },
        &data,
    )?;

    // Now lets switch ordering
    Tlv::strict_deserialize(&[
        // Count of TLV fields
        0x02, 0x00, //
        // Second goes first
        0xFE, 0xCA, // type
        0x01, 0x00, // length
        0xA1, // value
        // First goes second
        0xAD, 0x0B, // type
        0x01, 0x00, // length
        0xA1, // value
    ])
    .expect_err("");

    Ok(())
}

#[test]
fn pseudo_tlv() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(NetworkEncode, NetworkDecode)]
    #[network_encoding(use_tlv)]
    struct Tlv {
        vec: Vec<u8>,
        unknown_tlvs: BTreeMap<usize, Box<[u8]>>,
    }

    let data1 = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2 = [
        // vec:
        0x03, 0x00, 0xB1, 0xB2, 0xB3, // empty map:
        0x00, 0x00, // unknown_tlvs - auto added on top
        0x00, 0x00,
    ];

    // Checking that the data are entirely consumed
    let _: Tlv = strict_deserialize(&data1).unwrap();
    let _: Tlv = strict_deserialize(&data2).unwrap();

    test_encoding_roundtrip(&Tlv::default(), &data1)?;
    test_encoding_roundtrip(
        &Tlv {
            vec: vec![0xB1, 0xB2, 0xB3],
            unknown_tlvs: bmap! {},
        },
        &data2,
    )
    .map_err(Error::from)
}

#[test]
fn tlv_collection() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(NetworkEncode, NetworkDecode)]
    #[network_encoding(use_tlv)]
    struct Tlv {
        #[network_encoding(tlv = 0xCAFE)]
        map: BTreeMap<u8, String>,

        #[network_encoding(tlv = 0xBAD)]
        vec: Vec<u8>,
    }

    test_encoding_roundtrip(&Tlv::default(), &[0x00, 0x00])?;
    test_encoding_roundtrip(
        &Tlv {
            map: bmap! { 0xA1u8 => s!("First"), 0xA2u8 => s!("Second") },
            vec: vec![0xB1, 0xB2, 0xB3],
        },
        &[
            // Count of TLV fields
            0x02, 0x00, //
            // First goes first
            0xAD, 0x0B, // type
            0x05, 0x00, // length
            0x03, 0x00, 0xB1, 0xB2, 0xB3, // value
            // Second goes second
            0xFE, 0xCA, // type
            0x13, 0x00, // length
            0x02, 0x00, // value: # of map elements
            0xA1, 0x05, 0x00, b'F', b'i', b'r', b's', b't', // first entry
            0xA2, 0x06, 0x00, b'S', b'e', b'c', b'o', b'n', b'd',
        ],
    )
    .map_err(Error::from)
}

#[test]
fn tlv_stream() -> Result {
    #[derive(Clone, PartialEq, Eq, Debug, Default)]
    #[derive(NetworkEncode, NetworkDecode)]
    #[network_encoding(use_tlv)]
    struct TlvStream {
        field: Vec<u8>,

        #[network_encoding(tlv = 1)]
        tlv_int: Option<u16>,

        #[network_encoding(tlv = 2)]
        tlv_int2: Option<String>,

        #[network_encoding(unknown_tlvs)]
        rest_of_tlvs: tlv::Stream,
    }

    test_encoding_roundtrip(&TlvStream::default(), &[0x00; 4])
        .map_err(Error::from)
}

// TODO: Complete TLV encoding derivation test cases:
//       - Failing on unknown even fields
//       - Failed lengths etc
