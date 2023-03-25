use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

const PAYLOAD_ATTR: &str = "payload";

pub fn expand(ast: &syn::DeriveInput) -> TokenStream {
    let payload: Vec<_> = match get_payload_types(ast) {
        Ok(types) => types.iter().map(ToTokens::into_token_stream).collect(),
        Err(e) => return e.to_compile_error(),
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {

        impl #impl_generics #name #ty_generics #where_clause {
            pub async fn build(aid: ::kaantor::ActorId) -> ::kaantor::Node<#name #ty_generics> {
                use actix::prelude::*;
                let node = #name::from(aid);
                let addr = node.start();

                let node = ::kaantor::Node::new(aid, addr);

                #(let _ = node.register_proxy::<#payload>().await;)*

                node
            }
        }
    }
}

fn get_payload_types(ast: &syn::DeriveInput) -> syn::Result<Vec<syn::Type>> {
    let mut types = vec![];

    for attr in &ast.attrs {
        if attr.path().is_ident(PAYLOAD_ATTR) {
            match parse_payload(attr) {
                Ok(ts) => types.extend(ts),
                err => return err,
            }
        }
    }

    Ok(types)
}

fn parse_payload(attr: &syn::Attribute) -> syn::Result<Vec<syn::Type>> {
    let mut types = vec![];

    let _ = attr.parse_nested_meta(|meta| match meta.path.get_ident() {
        Some(ident) => {
            let res = syn::parse_str::<syn::Type>(&ident.to_string());
            match res {
                Ok(ty) => {
                    types.push(ty);
                    Ok(())
                }
                Err(e) => Err(meta.error(format!("Unrecognized type ({e})"))),
            }
        }
        None => Err(meta.error("Unrecognized meta (we are expecting an Ident")),
    });

    Ok(types)
}
