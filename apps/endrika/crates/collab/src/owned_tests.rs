//! `Admin::Owned` and its answer on the wire: both encodings round-trip, the new variants sit at
//! the end of their enums (so every message an older peer reads keeps its bytes), and an older
//! peer, which doesn't know them, fails to read them instead of misreading them.

use crate::message::{Admin, AdminReply, ClientMsg, OwnedFile, ServerMsg, decode, decode_json, encode, encode_json};

fn asked() -> ClientMsg {
    ClientMsg::Admin(Admin::Owned { files: vec![("f1".into(), "OwnerToken".into()), (String::new(), String::new())] })
}

fn answered() -> ServerMsg {
    let file = OwnedFile { name: "Homepage".into(), links: vec!["a".into(), "b".into()], expires: Some(1_900_000_000) };
    ServerMsg::Admin(Box::new(AdminReply::Owned(vec![Some(file), None])))
}

#[test]
fn both_encodings_round_trip() {
    let (c, s) = (asked(), answered());
    assert_eq!(decode::<ClientMsg>(&encode(&c).unwrap(), 1 << 20).unwrap(), c);
    assert_eq!(decode::<ServerMsg>(&encode(&s).unwrap(), 1 << 20).unwrap(), s);
    let json = encode_json(&c).unwrap();
    assert!(json.contains(r#""owned":{"files":[["f1","OwnerToken"],["",""]]}"#), "{json}");
    assert_eq!(decode_json::<ClientMsg>(&json, 1 << 20).unwrap(), c);
    let json = encode_json(&s).unwrap();
    assert!(json.contains(r#""owned":[{"name":"Homepage","links":["a","b"],"expires":1900000000},null]"#), "{json}");
    assert_eq!(decode_json::<ServerMsg>(&json, 1 << 20).unwrap(), s);
}

#[test]
fn the_new_variants_are_the_last_of_their_enums() {
    // Postcard writes a variant as its index: the last byte before the payload of an `Admin`
    // request is the variant's. `Code` was the last one before; `Owned` follows it.
    let index = |a: Admin| postcard::to_allocvec(&a).unwrap()[0];
    let code = index(Admin::Code { file: String::new(), token: String::new() });
    assert_eq!(index(Admin::Owned { files: vec![] }), code + 1);
    let index = |r: AdminReply| postcard::to_allocvec(&r).unwrap()[0];
    assert_eq!(index(AdminReply::Owned(vec![])), index(AdminReply::Code { code: String::new() }) + 1);
    // A peer from before them reads an unknown variant as an error, never as another message:
    // one index further is as unknown to this version as `Owned` is to an older one.
    let mut bytes = encode(&asked()).unwrap();
    let at = bytes.iter().position(|b| *b == code + 1).unwrap();
    bytes[at] = code + 2;
    assert!(decode::<ClientMsg>(&bytes, 1 << 20).is_err());
}

#[test]
fn hostile_answers_are_refused_by_size_not_trusted() {
    // A count that promises more than the message holds doesn't allocate or read past the end.
    let mut bytes = encode(&answered()).unwrap();
    bytes.truncate(bytes.len() - 3);
    assert!(decode::<ServerMsg>(&bytes, 1 << 20).is_err());
    assert!(decode::<ServerMsg>(&encode(&answered()).unwrap(), 4).is_err(), "over the size limit");
}
