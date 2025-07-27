use moverox::parse_move_datatype;
use moverox_types::{Address, StructTag};

#[derive(serde::Deserialize, serde::Serialize, moverox_traits::MoveDatatype)]
#[move_(module = manager)]
struct ManagerCap {
    id: UID,
    last_used: u64,
}

#[derive(serde::Deserialize, serde::Serialize, moverox_traits::MoveDatatype)]
#[move_(module = oracle)]
struct OracleCap {
    id: UID,
    last_used: u64,
}

// Dummy UID replacing the original type
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[expect(clippy::upper_case_acronyms)]
struct UID(Address);

#[test]
fn parse_manager_cap() {
    let tag: StructTag = "0xbeef::manager::ManagerCap".parse().unwrap();
    let value = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 215, 108, 134, 104, 0, 0, 0, 0,
    ];

    assert!(bcs::from_bytes::<OracleCap>(&value).is_ok());
    assert!(bcs::from_bytes::<ManagerCap>(&value).is_ok());

    assert!(parse_move_datatype::<OracleCap>(&tag, &value).is_err());
    assert!(parse_move_datatype::<ManagerCap>(&tag, &value).is_ok());
}
