#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

mod build_actor;

#[proc_macro_derive(BuildActor, attributes(payload))]
pub fn message_derive_rtype(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    build_actor::expand(&ast).into()
}
