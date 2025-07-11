use move_syn::{File, sanitize_for_tokenizer};
use testresult::TestResult;
use unsynn::{IParse as _, ToTokens as _};

macro_rules! test_file {
    ($($file: ident),* $(,)?) => {$(
        #[test]
        fn $file() -> TestResult {
            let content = include_str!(concat!(
                "../../../move/sui-framework/sources/",
                stringify!($file),
                ".move"
            ));
            let ast: File = sanitize_for_tokenizer(&content).into_token_iter().parse_all()?;
            insta::assert_snapshot!(ast.tokens_to_string());
            Ok(())
        }
    )*};
}

test_file!(
    accumulator,
    address,
    authenticator_state,
    bag,
    balance,
    bcs,
    borrow,
    clock,
    coin,
    config,
    deny_list,
    display,
    dynamic_field,
    dynamic_object_field,
    event,
    hex,
    linked_table,
    math,
    object,
    object_bag,
    object_table,
    package,
    party,
    pay,
    priority_queue,
    prover,
    random,
    sui,
    table,
    table_vec,
    token,
    transfer,
    tx_context,
    types,
    url,
    vec_map,
    vec_set,
    versioned,
);
