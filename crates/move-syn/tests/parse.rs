use move_syn::{File, sanitize_for_tokenizer};
use testresult::TestResult;
use unsynn::{IParse as _, ToTokens as _};

macro_rules! test_file {
    ($($package:literal {
       $( $path:literal ),* $(,)?
    })*) => {$($(
        let content = include_str!(concat!(
            "../../../move/",
            $package,
            "/sources/",
            $path,
            ".move"
        ));
        let _: File = sanitize_for_tokenizer(&content)
            .into_token_iter()
            .parse_all()
            .map_err(|err| format!("In Move file '{}': {err}", $path))?;
    )*)*};
}

#[test]
fn parse() -> TestResult {
    test_file! {
        "deepbook" {
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
        }
        "sui-framework" {
            "accumulator",
            "address",
            "authenticator_state",
            "bag",
            "balance",
            "bcs",
            "borrow",
            "clock",
            "coin",
            "config",
            "deny_list",
            "display",
            "dynamic_field",
            "dynamic_object_field",
            "event",
            "hex",
            "linked_table",
            "math",
            "object",
            "object_bag",
            "object_table",
            "package",
            "party",
            "pay",
            "priority_queue",
            "prover",
            "random",
            "sui",
            "table",
            "table_vec",
            "token",
            "transfer",
            "tx_context",
            "types",
            "url",
            "vec_map",
            "vec_set",
            "versioned",
        }
        "move-stdlib" {
            "address",
            "ascii",
            "bcs",
            "bit_vector",
            "bool",
            "debug",
            "fixed_point32",
            "hash",
            "macros",
            "option",
            "string",
            "type_name",
            "u128",
            "u64",
            "u8",
            "unit_test",
            "uq32_32",
            "uq64_64",
            "vector",
        }
        "legacy" {
            "main",
        }
        "enums" {
            "main",
            "other",
        }
    };

    Ok(())
}
