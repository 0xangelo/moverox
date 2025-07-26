use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::named_fields;

pub(super) trait BracedStructExt {
    /// The contents `{ ... }`, including the braces, for the generated Rust struct.
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

impl BracedStructExt for move_syn::BracedStruct {
    fn to_rust_contents<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream {
        named_fields::to_rust(
            &self.fields,
            phantoms,
            address_map,
            true, // visibility
        )
    }

    fn impl_new<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> (TokenStream, TokenStream) {
        let mut move_fields = named_fields::to_rust_fields(&self.fields, address_map).peekable();
        let has_fields = move_fields.peek().is_some();

        let args: &mut dyn Iterator<Item = TokenStream> = if has_fields {
            &mut move_fields
                .clone()
                .map(|named_fields::Rust { ident, ty, .. }| quote!(#ident: #ty))
        } else {
            &mut std::iter::empty()
        };

        let assign_args: &mut dyn Iterator<Item = TokenStream> = if !has_fields {
            &mut std::iter::once(quote!(dummy_field: false))
        } else {
            &mut move_fields
                .clone()
                .map(|named_fields::Rust { ident, .. }| quote!(#ident))
        };

        let phantom_data = phantoms.map(|ty| {
            let field = Ident::new(&format!("_{ty}"), ty.span());
            quote! {
                #field: ::std::marker::PhantomData
            }
        });

        let args = quote!(#(#args),*);
        let assignments = quote! {
            {
                #(#assign_args,)*
                #( #phantom_data ),*
            }
        };
        (args, assignments)
    }
}
