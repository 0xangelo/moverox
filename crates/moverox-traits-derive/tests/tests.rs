#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/sources/braced_struct.rs");
    t.pass("tests/sources/tuple_struct.rs");
    t.pass("tests/sources/generic_tuple_struct.rs");
    t.pass("tests/sources/generic_braced_struct.rs");
    t.pass("tests/sources/otw.rs");
    t.pass("tests/sources/no_type_bounds.rs");
    t.pass("tests/sources/enums.rs");
    t.compile_fail("tests/sources/empty_braced_struct.rs");
    t.compile_fail("tests/sources/empty_enum.rs");
    t.compile_fail("tests/sources/empty_tuple_struct.rs");
}
