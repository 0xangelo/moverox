use moverox_traits::{MoveDatatype, ParseStructTagError, StructTagError, TypeParamsError};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    ::moverox_traits::MoveDatatype,
)]
#[move_(address = "0x2")]
#[move_(module = dynamic_field)]
struct Field<Name, Value> {
    id: UID,
    name: Name,
    value: Value,
}

// Dummy UID replacing the original type
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[expect(clippy::upper_case_acronyms)]
struct UID(moverox_types::Address);

#[test]
fn field_type_tag_deser() {
    fn from_str(
        s: &str,
    ) -> Result<<Field<Vec<u8>, u64> as MoveDatatype>::StructTag, ParseStructTagError> {
        s.parse()
    }

    assert!(matches!(
        from_str("0xabc::dynamic_field::Field<vector<u8>, u64>"),
        Err(ParseStructTagError::StructTag(
            StructTagError::Address { .. }
        ))
    ));

    assert!(matches!(
        from_str("0x2::field::Field<vector<u8>, u64>"),
        Err(ParseStructTagError::StructTag(
            StructTagError::Module { .. }
        ))
    ));

    assert!(matches!(
        from_str("0x2::dynamic_field::DynamicField<vector<u8>, u64>"),
        Err(ParseStructTagError::StructTag(StructTagError::Name { .. }))
    ));

    assert!(matches!(
        from_str("0x2::dynamic_field::Field<vector<u8>, u64, u64>"),
        Err(ParseStructTagError::StructTag(StructTagError::TypeParams(
            TypeParamsError::Number { .. }
        )))
    ));

    assert!(from_str("0x2::dynamic_field::Field<vector<u8>, u64>").is_ok());

    insta::assert_snapshot!(from_str("0xabc::dynamic_field::Field<vector<u8>, u64>").unwrap_err(), @"Converting from StructTag: Wrong address: expected 0x0000000000000000000000000000000000000000000000000000000000000002, got 0x0000000000000000000000000000000000000000000000000000000000000abc");

    insta::assert_snapshot!(from_str("0x2::field::Field<vector<u8>, u64>").unwrap_err(), @"Converting from StructTag: Wrong module: expected dynamic_field, got field");

    insta::assert_snapshot!(from_str("0x2::dynamic_field::DynamicField<vector<u8>, u64>").unwrap_err(), @"Converting from StructTag: Wrong name: expected Field, got DynamicField");

    insta::assert_snapshot!(from_str("0x2::dynamic_field::Field<vector<u8>, u64, u64>").unwrap_err(), @"Converting from StructTag: Wrong type parameters: Wrong number of generics: expected 2, got 3");
}
