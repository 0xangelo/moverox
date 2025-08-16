#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Derive macros for `moverox-traits`.

use proc_macro2::TokenStream;
use quote::quote;

use crate::datatype::Datatype;

mod attributes;
mod datatype;
mod type_tag;

/// Derives `moverox_traits` trait implementations for an oxidized Move datatype.
///
/// Creates the `_TypeTag` struct related to the type being annotated, with conversion traits
/// between the former and the generic `StructTag` type, with errors like 'expected module to be x'
/// or 'expected struct name to be y', if we know those things at compile time (see the
/// [Attributes](#attributes) section for configurations around those checks).
///
/// # Attributes
///
/// - `#[move_(crate = ...)]`: sets the path of the `moverox_traits` crate, which can be useful if
///   using this inside other macros.
/// - `#[move_(address = "...")]`: sets a static package address for the generated type tag.
///   Deserialization of the latter will fail if the package addresses do not match.
/// - `#[move_(module = "...")]`: sets a static module name for the generated type tag.
///   Deserialization of the latter will fail if the module names do not match.
/// - `#[move_(nameless)]`: make the datatype name dynamic for the generated type tag. Upon the
///   deserializing the latter, any Move datatype name will be accepted. Otherwise, deserialization
///   will fail if the incoming datatype name is not equal to the Rust type's name.
///
/// # Type tag derivation
///
/// For a struct `Name<T: MoveType>`, the macro will create a `NameTypeTag` struct with fields:
/// - `address: Address`, unless the `#[move_(address = "...")]` attribute is present
/// - `module: Identifier`, unless the `#[move_(module = "...")]` attribute is present
/// - `name: Identifier` only if the `#[move_(nameless)]` attribute is present
/// - `type_t: <T as MoveType>::TypeTag`
///
/// The macro will also create custom `Into<StructTag>`, `Into<TypeTag>`, `TryFrom<&StructTag>`,
/// `TryFrom<StructTag>`, `TryFrom<&TypeTag>`, `TryFrom<TypeTag>`, `Display` and `FromStr` impls for
/// `NameTypeTag`.
///
/// # Derived traits
///
/// For the annotated type:
/// - `moverox_traits::MoveDatatype`
///
/// For the generated `_TypeTag` struct:
/// - `moverox_traits::ConstAddress` if `#[move_(address = "...")]` is specified
/// - `moverox_traits::ConstModule` if `#[move_(module = "...")]` is specified
/// - `moverox_traits::ConstName` if `#[move_(nameless)]` was **not** specified
#[proc_macro_derive(MoveDatatype, attributes(move_))]
pub fn move_datatype_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_move_datatype(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn impl_move_datatype(item: TokenStream) -> syn::Result<TokenStream> {
    let datatype = Datatype::parse(item)?;

    let type_tag_decl = datatype.type_tag.struct_declaration();
    let type_tag_impl_move_datatype_tag = datatype.type_tag.impl_move_datatype_tag();
    let type_tag_deserialize = datatype.type_tag.impl_deserialize();
    let type_tag_serialize = datatype.type_tag.impl_serialize();
    let type_tag_impl_const_address = datatype.type_tag.impl_const_address();
    let type_tag_impl_const_module = datatype.type_tag.impl_const_module();
    let type_tag_impl_const_name = datatype.type_tag.impl_const_name();
    let type_tag_impl_from_str = datatype.type_tag.impl_from_str();
    let type_tag_impl_display = datatype.type_tag.impl_display();

    let impl_move_datatype = datatype.impl_move_datatype();
    let impl_type_tag_constructor = datatype.impl_type_tag_constructor();
    let impl_const_struct_tag = datatype.impl_const_struct_tag().unwrap_or_default();

    Ok(quote! {
        #type_tag_decl
        #type_tag_impl_move_datatype_tag
        #type_tag_deserialize
        #type_tag_serialize
        #type_tag_impl_const_address
        #type_tag_impl_const_module
        #type_tag_impl_const_name
        #type_tag_impl_from_str
        #type_tag_impl_display

        #impl_move_datatype
        #impl_type_tag_constructor
        #impl_const_struct_tag
    })
}
