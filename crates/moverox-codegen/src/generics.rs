use std::collections::HashSet;

use quote::quote;
use unsynn::{Ident, ToTokens as _, TokenStream};

use crate::Result;

pub(super) trait GenericsExt {
    fn to_rust(&self) -> TokenStream;

    /// Generics declaration for the enum/struct definition.
    fn type_generics(
        &self,
        thecrate: &TokenStream,
        otw_types: HashSet<Ident>,
    ) -> Result<TokenStream>;

    fn phantoms(&self) -> impl Iterator<Item = &Ident>;
}

impl GenericsExt for move_syn::Generics {
    fn to_rust(&self) -> TokenStream {
        let idents = self.generics().map(|g| &g.ident);
        quote! {
            <#(#idents),*>
        }
    }

    fn type_generics(
        &self,
        thecrate: &TokenStream,
        mut otw_types: HashSet<Ident>,
    ) -> Result<TokenStream> {
        let idents: Vec<_> = self
            .generics()
            .map(|g| &g.ident)
            .map(|ident| {
                if otw_types.remove(ident) {
                    quote!(#ident = #thecrate::Otw)
                } else {
                    ident.to_token_stream()
                }
            })
            .collect();

        if !otw_types.is_empty() {
            let excess = otw_types
                .into_iter()
                .map(|ident| ident.to_string())
                .reduce(|a, b| a + ", " + &b)
                .unwrap_or_default();
            return Err(format!("Not a type parameter of this datatype: {excess}").into());
        }

        Ok(quote! {
            <#(#idents),*>
        })
    }

    fn phantoms(&self) -> impl Iterator<Item = &Ident> {
        self.generics()
            .filter(|d| d.phantom.is_some())
            .map(|d| &d.ident)
    }
}
