use quote::quote;
use unsynn::{Ident, TokenStream};

pub(super) trait GenericsExt {
    fn to_rust(&self) -> TokenStream;

    fn to_rust_with_bound(&self, bound: &TokenStream) -> TokenStream;

    fn phantoms(&self) -> impl Iterator<Item = &Ident>;
}

impl GenericsExt for move_syn::Generics {
    fn to_rust(&self) -> TokenStream {
        let idents = self.generics().map(|g| &g.ident);
        quote! {
            <#(#idents),*>
        }
    }

    fn to_rust_with_bound(&self, bound: &TokenStream) -> TokenStream {
        let idents = self.generics().map(|d| &d.ident);
        quote! {
            <#(#idents: #bound),*>
        }
    }

    fn phantoms(&self) -> impl Iterator<Item = &Ident> {
        self.generics()
            .filter(|d| d.phantom.is_some())
            .map(|d| &d.ident)
    }
}
