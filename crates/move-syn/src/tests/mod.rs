use crate::*;

mod imports;

#[test]
fn empty_struct() {
    ensure_roundtrip_move_struct("struct OTW {}");
    ensure_roundtrip_move_struct("struct OTW()");
}

#[test]
fn public_empty_struct() {
    ensure_roundtrip_move_struct("public struct OTW {}");
    ensure_roundtrip_move_struct("public struct OTW()");
}

#[test]
#[should_panic]
fn tuple_struct_trailing_semicolon() {
    ensure_roundtrip_move_struct("public struct OTW();");
}

#[test]
fn empty_struct_with_ability() {
    ensure_roundtrip_move_struct("struct OTW has key {}");
    ensure_roundtrip_move_struct("struct OTW() has key;");
}

#[test]
#[should_panic]
fn tuple_struct_with_ability_missing_semicolon() {
    ensure_roundtrip_move_struct("struct OTW() has key");
}

#[test]
fn empty_struct_with_abilities() {
    ensure_roundtrip_move_struct("struct OTW has key, store {}");
}

#[test]
fn missing_ability() {
    let _ = "struct OTW has {}"
        .to_token_iter()
        .parse::<Struct>()
        .expect_err("Missing keyword after 'has'");
}

#[test]
fn trailing_comma_after_ability() {
    let _ = "struct OTW has drop, {}"
        .to_token_iter()
        .parse::<Struct>()
        .expect_err("Trailing comma after 'drop'");
}

#[test]
fn struct_with_attr() {
    ensure_roundtrip_move_struct(
        "
            #[attr]
            struct Name {} ",
    );
}

#[test]
fn struct_with_field() {
    ensure_roundtrip_move_struct(
        "
            public struct Admin has key {
                id: UID
            } ",
    );
}

#[test]
fn struct_with_fields() {
    ensure_roundtrip_move_struct(
        "
            public struct Admin has key {
                id: UID,
                sender: address,
                object: ID
            } ",
    );
}

#[test]
fn struct_with_annotated_fields() {
    ensure_roundtrip_move_struct(
        "
        /// A general 'object admin'.
        public struct Admin has key {
            id: UID,
            /// Transaction sender with irrevokable privileged access.
            sender: address,
            /// Object being admistrated. Never changes after construction.
            object: ID
        } ",
    );
}

#[test]
fn empty_tuple_struct() {
    ensure_roundtrip_move_struct("public struct Wut()");
}

#[test]
fn empty_tuple_struct_with_ability() {
    ensure_roundtrip_move_struct("public struct Wut() has drop;");
}

#[test]
fn tuple_struct_with_fields_and_ability() {
    ensure_roundtrip_move_struct("public struct Wut(u64, address) has drop;");
}

#[test]
fn empty_phantom_generic_tuple_struct() {
    ensure_roundtrip_move_struct("public struct Wut<phantom T>()");
}

#[test]
fn tuple_struct_with_generic_field_type_and_ability() {
    ensure_roundtrip_move_struct("public struct Wut<T>(T) has drop;");
}

pub fn ensure_roundtrip_move_struct(decl: &str) {
    let ast: Item = decl.to_token_iter().parse_all().unwrap();
    assert!(matches!(
        ast,
        Item {
            kind: ItemKind::Struct(_),
            ..
        }
    ));
    assert_eq!(ast.tokens_to_string(), decl.tokens_to_string());
}

#[test]
fn move_module() {
    ensure_roundtrip_move_module("module package::module_name {}");
    ensure_roundtrip_move_module(
        "
            module package::module_name {
                struct MODULE_NAME()

                /// A general 'object admin'.
                public struct Admin has key {
                    id: UID,
                    /// Transaction sender with irrevokable privileged access.
                    sender: address,
                    /// Object being admistrated. Never changes after construction.
                    object: ID
                }
            }",
    );
}

fn ensure_roundtrip_move_module(decl: &str) {
    let ast: Module = decl.to_token_iter().parse_all().unwrap();
    assert_eq!(ast.tokens_to_string(), decl.tokens_to_string());
}
