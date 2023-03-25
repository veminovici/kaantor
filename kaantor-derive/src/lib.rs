#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

mod build_node;
mod into_actorid;

#[proc_macro_derive(BuildNode, attributes(payload))]
pub fn buildnode_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    build_node::expand(&ast).into()
}

#[proc_macro_derive(IntoActorId)]
pub fn intoactorid_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    into_actorid::expand(&ast).into()
}
