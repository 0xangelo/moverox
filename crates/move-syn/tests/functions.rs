use std::path::Path;

use itertools::Itertools as _;
use move_syn::{File, Function, ItemKind, Module, Visibility};
use testresult::TestResult;
use unsynn::ToTokens;

mod sui_framework {
    use super::*;

    test_files!("sui-framework" {
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
    });
}

macro_rules! test_files {
    ($package:literal { $($file: ident),* $(,)? }) => {$(
        #[test]
        fn $file() -> TestResult {
            use unsynn::{IParse as _, ToTokens as _};
            let path = move_dir()?
                .join($package)
                .join("sources")
                .join(concat!(stringify!($file), ".move"));
            let content = std::fs::read_to_string(path)?;
            let ast: File = move_syn::sanitize_for_tokenizer(&content)
                .as_str()
                .to_token_iter()
                .parse_all()?;
            let signatures = fun_signatures(ast);
            if !signatures.is_empty() {
                insta::assert_snapshot!(signatures);
            }
            Ok(())
        }
    )*};
}

pub(crate) use test_files;

fn fun_signatures(ast: File) -> String {
    let modules = ast
        .into_modules()
        .map(|mut module| {
            module
                .with_implicit_sui_imports()
                .fully_qualify_fun_signature_types();
            module
        })
        .collect_vec();
    modules
        .iter()
        .flat_map(Module::items)
        .filter(|item| matches!(item.visibility(), Visibility::Public))
        .filter_map(|item| match &item.kind {
            ItemKind::Function(fun) => Some(fun),
            _ => None,
        })
        .map(signature)
        .join("\n")
}

fn move_dir() -> TestResult<std::path::PathBuf> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("../")?
        .parent()
        .ok_or("../")?
        .join("move"))
}

fn signature(fun: &Function) -> String {
    fn to_string(t: &impl ToTokens) -> String {
        t.tokens_to_string()
    }

    let maybe_entry = if fun.is_entry() { "entry " } else { "" };
    let ident = fun.ident();
    let generics = fun.generics().map(to_string).unwrap_or_default();

    let args = fun
        .arguments()
        .map(|arg| {
            let type_ = to_string(arg.type_());
            format!("{}: {type_}", arg.ident())
        })
        .reduce(|a, b| a + ", " + &b)
        .unwrap_or_default();

    let mut returns = fun.returns();
    let ret = if returns.len() > 1 {
        let types = returns
            .map(to_string)
            .reduce(|a, b| a + ", " + &b)
            .unwrap_or_default();
        format!(": ({types})")
    } else if returns.len() == 1 {
        format!(": {}", returns.next().map(to_string).expect("len == 1"))
    } else {
        String::new()
    };
    format!("{maybe_entry}fun {ident}{generics}({args}){ret}")
        .replace(" ,", ",")
        .replace(",)", ")")
        .replace(" <", "<")
        .replace("< ", "<")
        .replace(" >", ">")
        .replace(" :", ":")
        .replace(":: ", "::")
        .replace("& ", "&")
}
