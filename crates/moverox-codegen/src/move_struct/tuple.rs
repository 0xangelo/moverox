use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use unsynn::ToTokens as _;

use crate::move_type;

pub(super) trait TupleStructExt {
    /// The contents `( ... )`, including the parenthesis, for the generated Rust struct.
    fn to_rust_contents<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream;

    /// The `pub fn new` implementation for the generated Rust struct.
    fn impl_new<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> (TokenStream, TokenStream);
}

impl TupleStructExt for move_syn::TupleStruct {
    fn to_rust_contents<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream {
        let parenthesized_fields = crate::positional_fields::to_rust(
            &self.fields,
            phantoms,
            address_map,
            true, // bool_if_empty
            true, // visibility
        );

        quote! {
            #parenthesized_fields;
        }
    }

    fn impl_new<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> (TokenStream, TokenStream) {
        let move_fields = self.fields();

        let args: &mut dyn Iterator<Item = TokenStream> = if self.is_empty() {
            &mut std::iter::empty()
        } else {
            &mut move_fields.clone().enumerate().map(|(i, d)| {
                let ty = move_type::to_rust_with_substitutions(&d.ty, address_map);
                let ident = Ident::new(&format!("_{i}"), Span::call_site());
                quote! (#ident: #ty)
            })
        };

        let assign_args: &mut dyn Iterator<Item = TokenStream> = if self.is_empty() {
            &mut std::iter::once(quote!(false))
        } else {
            &mut move_fields
                .enumerate()
                .map(|(i, _)| Ident::new(&format!("_{i}"), Span::call_site()).to_token_stream())
        };

        let phantom_data = phantoms.map(|_| quote!(::std::marker::PhantomData));

        let args = quote!(#(#args),*);
        let assignments = quote! {
            (
                #(#assign_args,)*
                #( #phantom_data ),*
            )
        };
        (args, assignments)
    }
}
