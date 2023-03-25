use proc_macro2::TokenStream;
use quote::quote;

pub fn expand(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {

        impl #impl_generics From<::kaantor::ActorId> for #name #ty_generics #where_clause {
            fn from(aid: ::kaantor::ActorId) -> Self {
                Self { aid, ..Default::default() }
            }
        }
    }
}
