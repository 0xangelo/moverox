use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::{DeriveInput, GenericParam, Generics, Path, Token, TypeParamBound, parse_quote};

use crate::attributes::MoveAttributes;
use crate::type_tag::TypeTagStruct;

pub(crate) struct Datatype {
    pub(crate) type_tag: TypeTagStruct,
    value: DeriveInput,
}

impl Datatype {
    pub(crate) fn parse(item: TokenStream) -> syn::Result<Self> {
        let mut ast: DeriveInput = syn::parse2(item)?;
        ensure_nonempty_struct(&ast)?;
        let attrs: MoveAttributes = deluxe::extract_attributes(&mut ast)?;
        validate_datatype_generics(&ast.generics)?;
        let type_tag = TypeTagStruct::new(&ast, &attrs);
        Ok(Self {
            type_tag,
            value: ast,
        })
    }

    pub(crate) fn impl_move_datatype(&self) -> TokenStream {
        let Self {
            type_tag,
            value: ast,
        } = self;
        let TypeTagStruct {
            ident: type_tag_ident,
            thecrate,
            ..
        } = type_tag;

        let type_tag_type = {
            let type_generics = type_arguments_in_associated_type(&ast.generics);
            quote!(#type_tag_ident < #type_generics >)
        };

        let ident = &ast.ident;
        let generics = add_type_bound(ast.generics.clone(), parse_quote!(#thecrate::MoveType));
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        quote! {
            impl #impl_generics #thecrate::MoveType for #ident #type_generics #where_clause {
                type TypeTag = #type_tag_type;
            }

            impl #impl_generics #thecrate::MoveDatatype for #ident #type_generics #where_clause {
                type StructTag = #type_tag_type;
            }
        }
    }

    pub(crate) fn impl_type_tag_constructor(&self) -> TokenStream {
        let Self {
            type_tag,
            value: ast,
        } = self;
        let TypeTagStruct {
            ident: type_tag_ident,
            thecrate,
            ..
        } = type_tag;

        // for use in function signatures
        let type_tag_fn_args: Vec<_> = type_tag
            .non_type_fields()
            .into_iter()
            .filter_map(|f| {
                let name = f.ident?;
                let ty = f.ty;
                Some(quote!(#name: #ty))
            })
            .chain(type_tag.type_fields().into_iter().filter_map(|f| {
                let name = f.ident?;
                let ty = f.ty;
                Some(quote!(#name: #ty::TypeTag))
            }))
            .collect();

        // to use in constructing the type tag struct
        let type_tag_field_names: Vec<_> = type_tag
            .fields()
            .into_iter()
            .filter_map(|f| f.ident)
            .collect();

        let ident = &ast.ident;
        let generics = add_type_bound(ast.generics.clone(), parse_quote!(#thecrate::MoveType));
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let type_tag_type = self.type_tag_type();

        quote! {
            impl #impl_generics #ident #type_generics #where_clause {
                /// Create this type's specialized type tag.
                pub const fn type_tag(#(#type_tag_fn_args),*) -> #type_tag_type {
                    #type_tag_ident {
                        #(#type_tag_field_names),*
                    }
                }
            }
        }
    }

    /// If the type tag's address, module and name are const, implement `ConstStructTag`
    /// conditional on all type parameters implementing `ConstTypeTag`.
    pub(crate) fn impl_const_struct_tag(&self) -> Option<TokenStream> {
        let Self {
            type_tag:
                TypeTagStruct {
                    ident: type_tag_ident,
                    address,
                    module,
                    name,
                    thecrate,
                    ..
                },
            value,
        } = self;
        if address.is_none() || module.is_none() || name.is_none() {
            return None;
        }

        let ident = &value.ident;
        let generics = add_type_bound(
            value.generics.clone(),
            parse_quote!(#thecrate::ConstTypeTag),
        );
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let type_tag_type = self.type_tag_type();

        let type_tag_constructor = self
            .type_tag
            .type_fields() // We know address, module and name are const
            .into_iter()
            .filter_map(|f| {
                let name = f.ident?;
                let ty = f.ty;
                Some(quote!(#name: <#ty as #thecrate::ConstTypeTag>::TYPE_TAG))
            });

        Some(quote! {
            impl #impl_generics #thecrate::ConstStructTag for #ident #type_generics #where_clause {
                const STRUCT_TAG: #type_tag_type = #type_tag_ident {
                    #(#type_tag_constructor),*
                };
            }
        })
    }

    /// This datatype's associated type tag as `_TypeTag<T::TypeTag, U::TypeTag, ...>`, where `T,
    /// U, ...` are this type's generic types.
    fn type_tag_type(&self) -> TokenStream {
        let type_tag_ident = &self.type_tag.ident;

        let type_generics = type_arguments_in_associated_type(&self.value.generics);
        quote!(#type_tag_ident < #type_generics >)
    }
}

fn ensure_nonempty_struct(ast: &DeriveInput) -> syn::Result<()> {
    match &ast.data {
        syn::Data::Struct(data) => {
            if data.fields.is_empty() {
                return Err(syn::Error::new(
                    data.fields.span(),
                    "Structs can't be empty. If a Move struct is empty, then in the Rust equivalent it \
                must have a single field of type `bool`. This is because the BCS of an empty Move \
                struct encodes a single boolean dummy field.",
                ));
            }
        }
        syn::Data::Enum(data) => {
            if data.variants.is_empty() {
                return Err(syn::Error::new(
                    data.variants.span(),
                    "A Move 'enum' must define at least one variant",
                ));
            }
        }
        _ => {
            return Err(syn::Error::new(
                ast.span(),
                "MoveDatatype only defined for structs",
            ));
        }
    };
    Ok(())
}

/// Check that the datatype (struct/enum) has valid generics.
fn validate_datatype_generics(generics: &Generics) -> syn::Result<()> {
    use syn::TypeParamBound;

    for param in &generics.params {
        match param {
            GenericParam::Type(type_param) => {
                if type_param.bounds.iter().all(|bound| {
                    matches!(
                        bound,
                        TypeParamBound::Trait(trait_bound) if expected_trait_bound(trait_bound)
                    )
                }) {
                    continue;
                }
                return Err(deluxe::Error::new_spanned(
                    type_param,
                    "Move datatypes can at most have the `moverox_traits::MoveType` bound on its \
                        type parameters",
                ));
            }
            _ => {
                return Err(deluxe::Error::new_spanned(
                    param,
                    "Only Type generics are supported",
                ));
            }
        }
    }
    Ok(())
}

/// Move datatypes must have the `moverox_traits::MoveType` bound in all of its type parameters.
fn expected_trait_bound(bound: &syn::TraitBound) -> bool {
    matches!(bound.modifier, syn::TraitBoundModifier::None)
        && bound.lifetimes.is_none()
        && bound.path.segments.last().is_some_and(|ps| {
            ps.ident == "MoveType" && matches!(ps.arguments, syn::PathArguments::None)
        })
}

// https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs#L36-L44
fn add_type_bound(mut generics: Generics, bound: TypeParamBound) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(bound.clone());
        }
    }
    generics
}

/// The `TypeTag` and `StructTag` associated types have type parameters like `<T::TypeTag, ...>`.
fn type_arguments_in_associated_type(generics: &Generics) -> Punctuated<Path, Token![,]> {
    use syn::GenericParam as G;
    let idents = generics.params.iter().filter_map(|p| {
        if let G::Type(t) = p {
            Some(&t.ident)
        } else {
            None
        }
    });
    parse_quote!(#(#idents::TypeTag),*)
}
