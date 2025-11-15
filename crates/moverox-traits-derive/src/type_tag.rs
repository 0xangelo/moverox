use convert_case::{Case, Casing as _};
use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, GenericParam, Generics, Ident, Path, Token, parse_quote};

use crate::attributes::MoveAttributes;

pub(crate) struct TypeTagStruct {
    /// Identifier of the type tag struct in Rust.
    pub(crate) ident: Ident,
    // These have values if they are statically known
    /// The address of the Move package defining the type.
    pub(crate) address: Option<String>,
    /// The name of the Move module defining the type.
    pub(crate) module: Option<String>,
    /// The name of the type in Move.
    pub(crate) name: Option<String>,

    /// The type tag struct generics in Rust.
    ///
    /// Should mirror the data type's generics, but with the `: moverox_traits::MoveTypeTag` bound on
    /// each.
    pub(crate) generics: Generics,

    /// Path to the `moverox_traits` crate
    pub(crate) thecrate: Path,
}

impl TypeTagStruct {
    pub(crate) fn new(ast: &DeriveInput, attrs: &MoveAttributes) -> Self {
        Self {
            ident: type_tag_ident(ast),
            address: attrs.address.clone(),
            module: attrs.module.as_ref().map(ToString::to_string),
            name: if attrs.nameless {
                None
            } else {
                Some(ast.ident.to_string())
            },
            generics: datatype_generics_to_typetag_generics(ast.generics.clone(), attrs.thecrate()),
            thecrate: attrs.thecrate(),
        }
    }

    pub(crate) fn struct_declaration(&self) -> TokenStream {
        let Self {
            ident, generics, ..
        } = self;
        let fields = self.fields();

        quote! {
            #[derive(
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
            )]
            pub struct #ident #generics {
                #fields
            }
        }
    }

    pub(crate) fn impl_deserialize(&self) -> TokenStream {
        let Self {
            ident,
            generics,
            thecrate,
            ..
        } = self;

        let result_type = result_type();
        let serde_crate = quote!(#thecrate::external::serde);

        // Add the `'de` lifetime only for the `impl` generics
        let mut ext_generics = generics.clone();
        ext_generics
            .params
            .push(GenericParam::Lifetime(parse_quote!('de)));
        let impl_generics = ext_generics.split_for_impl().0;

        let (_, type_generics, where_clause) = generics.split_for_impl();

        quote! {
            impl #impl_generics #serde_crate::Deserialize<'de> for #ident #type_generics #where_clause {
                fn deserialize<D>(deserializer: D) -> #result_type<Self, D::Error>
                where D: #serde_crate::Deserializer<'de>
                {
                    let stag = #thecrate::external::StructTag::deserialize(deserializer)?;
                    <Self as #thecrate::MoveDatatypeTag>::from_struct_tag(&stag).map_err(#serde_crate::de::Error::custom)
                }
            }
        }
    }

    pub(crate) fn impl_serialize(&self) -> TokenStream {
        let Self {
            ident,
            generics,
            thecrate,
            ..
        } = self;

        let result_type = result_type();
        let serde_crate = quote!(#thecrate::external::serde);

        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        quote! {
            impl #impl_generics #serde_crate::Serialize for #ident #type_generics #where_clause {
                fn serialize<S>(&self, serializer: S) -> #result_type<S::Ok, S::Error>
                where S: #serde_crate::Serializer
                {
                    #thecrate::MoveDatatypeTag::to_struct_tag(self).serialize(serializer)
                }
            }
        }
    }

    pub(crate) fn impl_const_address(&self) -> TokenStream {
        let Self {
            ident,
            address,
            generics,
            thecrate,
            ..
        } = self;
        if address.is_none() {
            return Default::default();
        }
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
        quote! {
            impl #impl_generics #thecrate::ConstAddress for #ident #type_generics #where_clause {
                const ADDRESS: #thecrate::external::Address =
                    #thecrate::external::const_address(#address.as_bytes());
            }
        }
    }

    pub(crate) fn impl_const_module(&self) -> TokenStream {
        let Self {
            ident,
            module,
            generics,
            thecrate,
            ..
        } = self;
        if module.is_none() {
            return Default::default();
        }
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
        quote! {
            impl #impl_generics #thecrate::ConstModule for #ident #type_generics #where_clause {
                const MODULE: &#thecrate::external::IdentStr =
                    #thecrate::external::const_ident(#module);
            }
        }
    }

    pub(crate) fn impl_const_name(&self) -> TokenStream {
        let Self {
            ident,
            name,
            generics,
            thecrate,
            ..
        } = self;
        if name.is_none() {
            return Default::default();
        }
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
        quote! {
            impl #impl_generics #thecrate::ConstName for #ident #type_generics #where_clause {
                const NAME: &#thecrate::external::IdentStr =
                    #thecrate::external::const_ident(#name);
            }
        }
    }

    /// `Display` implementation for the generated type tag struct. Requires it to be `Clone`
    pub(crate) fn impl_display(&self) -> TokenStream {
        let Self {
            ident,
            generics,
            thecrate,
            ..
        } = self;
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
        quote! {
            impl #impl_generics ::std::fmt::Display for #ident #type_generics
                #where_clause
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let stag = #thecrate::MoveDatatypeTag::to_struct_tag(self);
                    write!(f, "{}", stag)
                }
            }
        }
    }

    pub(crate) fn impl_from_str(&self) -> TokenStream {
        let Self {
            ident, thecrate, ..
        } = self;
        let result_type = result_type();
        let external = self.external();
        let struct_tag_type = quote!(#external::StructTag);
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            impl #impl_generics ::std::str::FromStr for #ident #type_generics
            #where_clause
            {
                type Err = #thecrate::ParseStructTagError;

                fn from_str(s: &str) -> #result_type<Self, Self::Err> {
                    let stag = s
                        .parse::<#struct_tag_type>()
                        .map_err(|e| #thecrate::ParseStructTagError::FromStr(e.into()))?;
                    #result_type::Ok(<Self as #thecrate::MoveDatatypeTag>::from_struct_tag(&stag)?)
                }
            }
        }
    }

    pub(crate) fn impl_move_datatype_tag(&self) -> TokenStream {
        let Self {
            ident,
            generics,
            thecrate,
            ..
        } = self;

        let required_getters_impl = self.impl_required_getters();
        let from_struct_tag_impl = self.impl_from_struct_tag();

        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        quote! {
            impl #impl_generics #thecrate::MoveDatatypeTag for #ident #type_generics
            #where_clause
            {
                #required_getters_impl
                #from_struct_tag_impl
            }
        }
    }

    pub(crate) fn impl_required_getters(&self) -> TokenStream {
        let Self { thecrate, .. } = self;

        let external = self.external();

        let type_param_idents = self.type_fields().into_iter().filter_map(|f| f.ident);

        let address_value = if self.address.is_some() {
            quote!(<Self as #thecrate::ConstAddress>::ADDRESS)
        } else {
            quote!(self.address)
        };

        let module_value = if self.module.is_some() {
            quote!(<Self as #thecrate::ConstModule>::MODULE)
        } else {
            quote! {
                use ::std::borrow::Borrow as _;
                self.module.borrow()
            }
        };

        let name_value = if self.name.is_some() {
            quote!(<Self as #thecrate::ConstName>::NAME)
        } else {
            quote! {
                use ::std::borrow::Borrow as _;
                self.name.borrow()
            }
        };

        quote! {
            fn address(&self) -> #external::Address {
                #address_value
            }

            fn module(&self) -> &#external::IdentStr {
                #module_value
            }

            fn name(&self) -> &#external::IdentStr {
                #name_value
            }

            fn type_params(&self) -> Box<dyn ::std::iter::ExactSizeIterator <Item = &dyn #thecrate::MoveTypeTag > + '_ > {
                Box::new([#(&self.#type_param_idents as _),*].into_iter())
            }
        }
    }

    pub(crate) fn impl_from_struct_tag(&self) -> TokenStream {
        let Self { thecrate, .. } = self;
        let result_type = result_type();
        let external = self.external();
        let struct_tag_type = quote!(#external::StructTag);

        let address_check = if self.address.is_some() {
            quote! {
                let expected = <Self as #thecrate::ConstAddress>::ADDRESS;
                if address != &expected {
                    return #result_type::Err(E::Address { expected, got: *address });
                }
            }
        } else {
            TokenStream::new()
        };

        let module_check = if self.module.is_some() {
            quote! {
                let expected = <Self as #thecrate::ConstModule>::MODULE;
                let actual = ::std::borrow::Borrow::<#external::IdentStr>::borrow(module);
                if expected != actual {
                    return #result_type::Err(E::Module {
                        expected: expected.to_owned(),
                        got: module.clone()
                    });
                }
            }
        } else {
            TokenStream::new()
        };

        let name_check = if self.name.is_some() {
            quote! {
                let expected = <Self as #thecrate::ConstName>::NAME;
                let actual = ::std::borrow::Borrow::<#external::IdentStr>::borrow(name);
                if expected != actual {
                    return #result_type::Err(E::Name {
                        expected: expected.to_owned(),
                        got: name.clone()
                    });
                }
            }
        } else {
            TokenStream::new()
        };

        let n_types_expected = {
            let n_types = self.generic_type_idents().count();
            quote!(#n_types)
        };

        let field_idents = self.non_type_fields().into_iter().filter_map(|f| f.ident);
        let type_field_idents: Vec<_> = self.type_field_pairs().map(|pair| pair.0).collect();

        quote! {
            fn from_struct_tag(value: &#struct_tag_type) -> #result_type<Self, #thecrate::StructTagError> {
                use #thecrate::StructTagError as E;
                let #struct_tag_type {
                    address,
                    module,
                    name,
                    type_params,
                } = value;

                #address_check
                #module_check
                #name_check

                // Extract type parameters
                let expected = #n_types_expected;
                if expected != type_params.len() {
                    return #result_type::Err(E::TypeParams(#thecrate::TypeParamsError::Number {
                        expected, got: type_params.len()
                    }));
                }
                let mut type_params_iter = type_params.iter();
                #(
                    let #type_field_idents = #thecrate::MoveTypeTag::from_type_tag(
                        type_params_iter
                            .next()
                            .expect("Checked type_params.len() above")
                    )
                    .map_err(#thecrate::TypeParamsError::from)?;
                )*

                #result_type::Ok(Self {
                    #(#field_idents: #field_idents.clone(),)*
                    #(#type_field_idents),*
                })
            }
        }
    }

    pub(crate) fn type_fields(&self) -> Punctuated<syn::Field, Token![,]> {
        self.fields()
            .into_iter()
            .skip(
                self.address.is_none() as usize
                    + self.module.is_none() as usize
                    + self.name.is_none() as usize,
            )
            .collect()
    }

    pub(crate) fn non_type_fields(&self) -> Punctuated<syn::Field, Token![,]> {
        self.fields()
            .into_iter()
            .take(
                self.address.is_none() as usize
                    + self.module.is_none() as usize
                    + self.name.is_none() as usize,
            )
            .collect()
    }

    pub(crate) fn fields(&self) -> Punctuated<syn::Field, Token![,]> {
        let thecrate = &self.thecrate;
        let mut punctuated = Punctuated::new();
        if self.address.is_none() {
            punctuated.push(parse_quote!(pub address: #thecrate::external::Address));
        }
        if self.module.is_none() {
            punctuated.push(parse_quote!(pub module: #thecrate::external::Identifier));
        }
        if self.name.is_none() {
            punctuated.push(parse_quote!(pub name: #thecrate::external::Identifier));
        }
        for (type_field, type_ident) in self.type_field_pairs() {
            punctuated.push(parse_quote!(pub #type_field: #type_ident));
        }
        punctuated
    }

    /// Pairs of ident and type for the struct's 'type' fields, e.g., `type_t: T`, `type_u: U`, etc.
    fn type_field_pairs(&self) -> impl Iterator<Item = (Ident, &Ident)> {
        self.generic_type_idents().map(|type_ident| {
            let to_snake = type_ident.to_string().to_case(Case::Snake);
            let type_field = Ident::new(&format!("type_{to_snake}"), type_ident.span());
            (type_field, type_ident)
        })
    }

    fn generic_type_idents(&self) -> impl Iterator<Item = &Ident> {
        self.generics.params.iter().filter_map(|p| {
            let GenericParam::Type(type_param) = p else {
                return None;
            };
            Some(&type_param.ident)
        })
    }

    /// Path to re-exports of the `moverox_traits` crate
    fn external(&self) -> Path {
        let mut path = self.thecrate.clone();
        path.segments.push(parse_quote!(external));
        path
    }
}

/// Transform the datatype's (struct/enum) generics into the generics for its type tag.
///
/// Simply put, the type tag generated for a datatype has the `: MoveTypeTag` bound in all of its
/// type arguments.
fn datatype_generics_to_typetag_generics(mut generics: Generics, thecrate: Path) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            let mut bounds = Punctuated::new();
            bounds.push(parse_quote!(#thecrate::MoveTypeTag));
            type_param.bounds = bounds;
        }
    }
    generics
}

/// The type tag struct's name is simply the datatype name + `TypeTag`.
fn type_tag_ident(ast: &DeriveInput) -> Ident {
    let ident = &ast.ident;
    Ident::new(&format!("{ident}TypeTag"), ident.span())
}

/// Unequivocal result type path
fn result_type() -> TokenStream {
    quote!(::std::result::Result)
}

#[test]
fn parse_quote_trait_bound() {
    let mut bounds = Punctuated::<syn::TypeParamBound, Token![+]>::new();
    bounds.push(parse_quote!(crate::MoveTypeTag));
}

#[test]
fn const_address_value() {
    let v = "0x2";
    let _: syn::Expr = parse_quote!(::moverox_traits::external::const_address(#v.as_bytes()));
}
