use quote::quote;
use unsynn::{Ident, TokenStream};

pub(super) trait GenericsExt {
    fn to_rust(&self) -> TokenStream;

    fn phantoms(&self) -> impl Iterator<Item = &Ident>;
}

impl GenericsExt for move_syn::Generics {
    fn to_rust(&self) -> TokenStream {
        let idents = self.generics().map(|g| &g.ident);
        quote! {
            <#(#idents),*>
        }
    }

    fn phantoms(&self) -> impl Iterator<Item = &Ident> {
        self.generics()
            .filter(|d| d.phantom.is_some())
            .map(|d| &d.ident)
    }
}
