#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/braced_struct.rs");
    t.pass("tests/tuple_struct.rs");
    t.pass("tests/generic_tuple_struct.rs");
    t.pass("tests/generic_braced_struct.rs");
    t.pass("tests/otw.rs");
    t.pass("tests/no_type_bounds.rs");
    t.compile_fail("tests/empty_braced_struct.rs");
    t.compile_fail("tests/empty_tuple_struct.rs");
}
