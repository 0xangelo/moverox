use move_syn::File;
use testresult::TestResult;
use unsynn::{IParse as _, ToTokens as _};

macro_rules! test_file {
    ($($file: ident),* $(,)?) => {$(
        #[test]
        fn $file() -> TestResult {
            let content = include_str!(concat!(
                "../../../move/move-stdlib/sources/",
                stringify!($file),
                ".move"
            ));
            let ast: File = content.to_token_iter().parse_all()?;
            insta::assert_snapshot!(ast.tokens_to_string());
            Ok(())
        }
    )*};
}

test_file!(
    address,
    ascii,
    bcs,
    bit_vector,
    bool,
    debug,
    fixed_point32,
    hash,
    macros,
    option,
    string,
    type_name,
    u128,
    u64,
    u8,
    unit_test,
    uq32_32,
    uq64_64,
    vector,
);
