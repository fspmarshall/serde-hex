//! Test of `SerHex` functionality with `serde-json`.
extern crate stremio_serde_hex;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use stremio_serde_hex::{CompactPfx, SerHex, StrictPfx};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Foo {
    #[serde(with = "SerHex::<StrictPfx>")]
    bar: [u8; 32],
    #[serde(with = "SerHex::<CompactPfx>")]
    bin: u64,
}

#[test]
fn serialize() {
    let foo = Foo {
        bar: [0; 32],
        bin: 0xff,
    };
    let ser = serde_json::to_string(&foo).unwrap();
    let exp = r#"{"bar":"0x0000000000000000000000000000000000000000000000000000000000000000","bin":"0xff"}"#;
    assert_eq!(ser, exp);
}

#[test]
fn deserialize() {
    let ser = r#"{"bar":"0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","bin":"0x1234"}"#;
    let foo = serde_json::from_str::<Foo>(ser).unwrap();
    let exp = Foo {
        bar: [0xaa; 32],
        bin: 0x1234,
    };
    assert_eq!(foo, exp);
}

#[test]
fn deserialize_owned() {
    let ser = serde_json::json!({
        "bar": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "bin": "0x1234"
    });
    let foo = serde_json::from_value::<Foo>(ser).unwrap();
    let exp = Foo {
        bar: [0xaa; 32],
        bin: 0x1234,
    };
    assert_eq!(foo, exp);
}
