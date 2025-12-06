use move_syn::File;
use testresult::TestResult;
use unsynn::{IParse as _, ToTokens as _};

macro_rules! test_file {
    (
       $( $path:literal ),* $(,)?
    ) => {$(
        let content = include_str!(concat!(
            "../../../move/deepbook/sources/",
            $path,
            ".move"
        ));
        let ast: File = content
            .to_token_iter()
            .parse_all()
            .map_err(|err| format!("In Move file '{}': {err}", $path))?;
        insta::assert_snapshot!($path, ast.tokens_to_string());
    )*};
}

#[test]
fn parse() -> TestResult {
    test_file! {
        "registry",
        "pool",
        "order_query",
        "balance_manager",
        "book/book",
        "book/fill",
        "book/order",
        "book/order_info",
        "helper/big_vector",
        "helper/constants",
        "helper/math",
        "helper/utils",
        "state/account",
        "state/balances",
        "state/governance",
        "state/history",
        "state/state",
        "state/trade_params",
        "vault/deep_price",
        "vault/vault",
    };

    Ok(())
}
