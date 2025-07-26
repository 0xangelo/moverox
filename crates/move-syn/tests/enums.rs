use move_syn::File;
use testresult::TestResult;
use unsynn::{IParse as _, ToTokens as _};

macro_rules! include_move {
    ($path: literal) => {
        include_str!(concat!("../../../move/", $path))
    };
}

#[test]
fn main_() -> TestResult {
    let ast: File = include_move!("enums/sources/main.move")
        .to_token_iter()
        .parse_all()?;
    insta::assert_snapshot!(ast.tokens_to_string());
    Ok(())
}
