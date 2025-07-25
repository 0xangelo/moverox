use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{DeriveInput, GenericParam, Generics, Path, Token, TypeParamBound, parse_quote};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(move_))]
struct MoveAttributes {
    #[deluxe(rename = crate)]
    thecrate: Option<Path>,
    address: Option<String>,
    module: Option<Ident>,
    #[deluxe(default = false)]
    nameless: bool,
}

impl MoveAttributes {
    fn thecrate(&self) -> Path {
        self.thecrate
            .as_ref()
            .cloned()
            .unwrap_or_else(|| parse_quote!(::moverox_traits))
    }
}

pub fn impl_move_struct(item: TokenStream) -> deluxe::Result<TokenStream> {
    // parse
    let mut ast: DeriveInput = syn::parse2(item)?;
    ensure_nonempty_struct(&ast)?;
    let attrs: MoveAttributes = deluxe::extract_attributes(&mut ast)?;

    let thecrate = attrs.thecrate();
    ast.generics = add_type_bound(ast.generics, parse_quote!(#thecrate::MoveType));
    validate_datatype_generics(&ast.generics)?;

    let type_tag = TypeTagStruct::new(&ast, &attrs);
    let type_tag_decl = type_tag.struct_declaration();
    let type_tag_impl_const_address = type_tag.impl_const_address();
    let type_tag_impl_const_module = type_tag.impl_const_module();
    let type_tag_impl_const_name = type_tag.impl_const_name();
    let type_tag_impl_into_struct_tag = type_tag.impl_into_struct_tag();
    let type_tag_impl_try_from_struct_tag = type_tag.impl_try_from_struct_tag();
    let type_tag_impl_basics = type_tag.impl_basics();
    let type_tag_impl_display = type_tag.impl_display();

    let struct_impl_move_type = move_struct_impl_move_type(&ast, type_tag);

    Ok(quote! {
        #type_tag_decl
        #type_tag_impl_const_address
        #type_tag_impl_const_module
        #type_tag_impl_const_name
        #type_tag_impl_into_struct_tag
        #type_tag_impl_try_from_struct_tag
        #type_tag_impl_basics
        #type_tag_impl_display

        #struct_impl_move_type
    })
}

fn ensure_nonempty_struct(ast: &DeriveInput) -> deluxe::Result<()> {
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
fn validate_datatype_generics(generics: &Generics) -> deluxe::Result<()> {
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

/// Main `impl` block for the struct and `MoveDatatype` impl for it
fn move_struct_impl_move_type(ast: &DeriveInput, type_tag: TypeTagStruct) -> TokenStream {
    let TypeTagStruct {
        ident: type_tag_ident,
        thecrate,
        ..
    } = &type_tag;

    let type_tag_type = {
        let type_generics = type_arguments_in_associated_type(&ast.generics);
        quote!(#type_tag_ident < #type_generics >)
    };

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

    let type_tag_field_names: Vec<_> = type_tag
        .fields()
        .into_iter()
        .filter_map(|f| f.ident)
        .collect();

    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics #thecrate::MoveType for #ident #type_generics #where_clause {
            type TypeTag = #type_tag_type;
        }

        impl #impl_generics #thecrate::MoveDatatype for #ident #type_generics #where_clause {
            type StructTag = #type_tag_type;
        }

        impl #impl_generics #ident #type_generics #where_clause {
            pub fn type_(#(#type_tag_fn_args),*) -> #type_tag_type {
                #type_tag_ident {
                    #(#type_tag_field_names),*
                }
            }
        }
    }
}

struct TypeTagStruct {
    /// Identifier of the type tag struct in Rust.
    ident: Ident,
    // These have values if they are statically known
    /// The address of the Move package defining the type.
    address: Option<String>,
    /// The name of the Move module defining the type.
    module: Option<String>,
    /// The name of the type in Move.
    name: Option<String>,

    /// The type tag struct generics in Rust.
    ///
    /// Should mirror the data type's generics, but with the `: moverox_traits::MoveTypeTag` bound on
    /// each.
    generics: Generics,

    /// Path to the `moverox_traits` crate
    thecrate: Path,
}

impl TypeTagStruct {
    fn new(ast: &DeriveInput, attrs: &MoveAttributes) -> Self {
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

    fn struct_declaration(&self) -> TokenStream {
        let Self {
            ident,
            generics,
            thecrate,
            ..
        } = self;
        let fields = self.fields();
        let serde_with_crate = quote!(#thecrate::external::serde_with).to_string();
        quote! {
            #[derive(
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                #thecrate::external::serde_with::DeserializeFromStr,
                #thecrate::external::serde_with::SerializeDisplay,
            )]
            #[serde_with(crate = #serde_with_crate)]
            pub struct #ident #generics {
                #fields
            }
        }
    }

    fn impl_const_address(&self) -> TokenStream {
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

    fn impl_const_module(&self) -> TokenStream {
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

    fn impl_const_name(&self) -> TokenStream {
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
    fn impl_display(&self) -> TokenStream {
        let Self {
            ident, thecrate, ..
        } = self;
        let struct_tag_type = quote!(#thecrate::external::StructTag);
        let generics = add_type_bound(self.generics.clone(), parse_quote!(::std::clone::Clone));
        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
        quote! {
            impl #impl_generics ::std::fmt::Display for #ident #type_generics
                #where_clause
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let stag: #struct_tag_type = self.clone().into();
                    write!(f, "{}", stag)
                }
            }
        }
    }

    fn impl_basics(&self) -> TokenStream {
        let Self {
            ident, thecrate, ..
        } = self;
        let result_type = result_type();
        let external = self.external();
        let type_tag_type = quote!(#external::TypeTag);
        let struct_tag_type = quote!(#external::StructTag);
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            impl #impl_generics ::std::convert::From<#ident #type_generics> for #type_tag_type
            #where_clause
            {
                fn from(value: #ident #type_generics) -> Self {
                    Self::Struct(::std::boxed::Box::new(value.into()))
                }
            }

            impl #impl_generics TryFrom<#type_tag_type> for #ident #type_generics
            #where_clause
            {
                type Error = #thecrate::TypeTagError;

                fn try_from(value: #type_tag_type) -> #result_type<Self, Self::Error> {
                    match value {
                        #type_tag_type::Struct(stag) => #result_type::Ok((*stag).try_into()?),
                        other => #result_type::Err(#thecrate::TypeTagError::Variant {
                            expected: "Struct(_)".to_owned(),
                            got: other,
                        }),
                    }
                }
            }

            impl #impl_generics ::std::str::FromStr for #ident #type_generics
            #where_clause
            {
                type Err = #thecrate::ParseStructTagError;

                fn from_str(s: &str) -> #result_type<Self, Self::Err> {
                    let stag = s
                        .parse::<#struct_tag_type>()
                        .map_err(|e| #thecrate::ParseStructTagError::FromStr(e.into()))?;
                    #result_type::Ok(stag.try_into()?)
                }
            }
        }
    }

    fn impl_into_struct_tag(&self) -> TokenStream {
        let Self {
            ident, thecrate, ..
        } = self;
        let external = self.external();
        let struct_tag_type = quote!(#external::StructTag);
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        let attr_idents: Vec<_> = self.fields().into_iter().filter_map(|f| f.ident).collect();

        let struct_tag_var_attrs = std::iter::empty()
            .chain(self.address.is_none().then_some(quote!(address)))
            .chain(self.module.is_none().then_some(quote!(module)))
            .chain(self.name.is_none().then_some(quote!(name)));

        let type_param_idents = self.type_fields().into_iter().filter_map(|f| f.ident);

        let struct_tag_const_declarations = std::iter::empty()
            .chain(self.address.is_some().then(|| {
                quote! {
                    address: <#ident #type_generics as #thecrate::ConstAddress>::ADDRESS
                }
            }))
            .chain(self.module.is_some().then(|| {
                quote! {
                    module: <#ident #type_generics as #thecrate::ConstModule>::MODULE.to_owned()
                }
            }))
            .chain(self.name.is_some().then(|| {
                quote! {
                    name: <#ident #type_generics as #thecrate::ConstName>::NAME.to_owned()
                }
            }))
            .chain(std::iter::once(
                quote!(type_params: vec![#(#type_param_idents.into()),*]),
            ));

        quote! {
            impl #impl_generics ::std::convert::From<#ident #type_generics> for #struct_tag_type
            #where_clause
            {
                fn from(value: #ident #type_generics) -> Self {
                    let #ident {
                        #(#attr_idents),*
                    } = value;
                    Self {
                        #(#struct_tag_var_attrs,)*
                        #(#struct_tag_const_declarations),*
                    }
                }
            }
        }
    }

    fn impl_try_from_struct_tag(&self) -> TokenStream {
        let Self {
            ident, thecrate, ..
        } = self;
        let result_type = result_type();
        let external = self.external();
        let struct_tag_type = quote!(#external::StructTag);
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        let address_check = if self.address.is_some() {
            quote! {
                let expected = <Self as #thecrate::ConstAddress>::ADDRESS;
                if address != expected {
                    return #result_type::Err(E::Address { expected, got: address });
                }
            }
        } else {
            TokenStream::new()
        };

        let module_check = if self.module.is_some() {
            quote! {
                let expected = <Self as #thecrate::ConstModule>::MODULE;
                let actual = ::std::borrow::Borrow::<#external::IdentStr>::borrow(&module);
                if expected != actual {
                    return #result_type::Err(E::Module {
                        expected: expected.to_owned(),
                        got: module
                    });
                }
            }
        } else {
            TokenStream::new()
        };

        let name_check = if self.name.is_some() {
            quote! {
                let expected = <Self as #thecrate::ConstName>::NAME;
                let actual = ::std::borrow::Borrow::<#external::IdentStr>::borrow(&name);
                if expected != actual {
                    return #result_type::Err(E::Name {
                        expected: expected.to_owned(),
                        got: name
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

        let field_idents: Vec<_> = self.fields().into_iter().filter_map(|f| f.ident).collect();
        let mut type_field_idents: Vec<_> = self.type_field_pairs().map(|pair| pair.0).collect();
        type_field_idents.reverse();

        quote! {
            impl #impl_generics TryFrom<#struct_tag_type> for #ident #type_generics
            #where_clause
            {
                type Error = #thecrate::StructTagError;

                fn try_from(value: #struct_tag_type) -> #result_type<Self, Self::Error> {
                    use #thecrate::StructTagError as E;
                    let #struct_tag_type {
                        address,
                        module,
                        name,
                        mut type_params,
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
                    #(
                        let #type_field_idents = type_params
                            .pop()
                            .expect("Checked type_params.len() above")
                            .try_into()
                            .map_err(#thecrate::TypeParamsError::from)?;
                    )*

                    #result_type::Ok(Self {
                        #(#field_idents),*
                    })
                }
            }
        }
    }

    fn type_fields(&self) -> Punctuated<syn::Field, Token![,]> {
        self.fields()
            .into_iter()
            .skip(
                self.address.is_none() as usize
                    + self.module.is_none() as usize
                    + self.name.is_none() as usize,
            )
            .collect()
    }

    fn non_type_fields(&self) -> Punctuated<syn::Field, Token![,]> {
        self.fields()
            .into_iter()
            .take(
                self.address.is_none() as usize
                    + self.module.is_none() as usize
                    + self.name.is_none() as usize,
            )
            .collect()
    }

    fn fields(&self) -> Punctuated<syn::Field, Token![,]> {
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

/// Move datatypes must have the `moverox_traits::MoveType` bound in all of its type parameters.
fn expected_trait_bound(bound: &syn::TraitBound) -> bool {
    matches!(bound.modifier, syn::TraitBoundModifier::None)
        && bound.lifetimes.is_none()
        && bound.path.segments.last().is_some_and(|ps| {
            ps.ident == "MoveType" && matches!(ps.arguments, syn::PathArguments::None)
        })
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

/// Unequivocal result type path
fn result_type() -> TokenStream {
    quote!(::std::result::Result)
}

#[test]
fn parse_quote_trait_bound() {
    let mut bounds = Punctuated::<TypeParamBound, Token![+]>::new();
    bounds.push(parse_quote!(crate::MoveTypeTag));
}

#[test]
fn const_address_value() {
    let v = "0x2";
    let _: syn::Expr = parse_quote!(::moverox_traits::external::const_address(#v.as_bytes()));
}
