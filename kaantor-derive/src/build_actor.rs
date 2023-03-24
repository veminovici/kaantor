use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};

pub const PAYLOAD_ATTR: &str = "payload";

pub fn expand(ast: &syn::DeriveInput) -> TokenStream {
    let items = {
        match get_attribute_type_multiple(ast, PAYLOAD_ATTR) {
            Ok(ty) => match ty.len() {
                0 => {
                    return syn::Error::new(
                        Span::call_site(),
                        format!(
                            "#[{}(type)] takes 1 parameters, given {}",
                            PAYLOAD_ATTR,
                            ty.len()
                        ),
                    )
                    .to_compile_error()
                }
                _ => ty,
            },
            Err(err) => return err.to_compile_error(),
        }
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let payload: Vec<_> = items
        .iter()
        .map(|i| i.as_ref().map(ToTokens::into_token_stream).unwrap())
        .collect();

    quote! {

        impl #impl_generics #name #ty_generics #where_clause {
            async fn build(aid: ::kaantor::ActorId) -> ::kaantor::Node<#name #ty_generics> {
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

fn get_attribute_type_multiple(
    ast: &syn::DeriveInput,
    name: &str,
) -> syn::Result<Vec<Option<syn::Type>>> {
    let attr = ast
        .attrs
        .iter()
        .find_map(|a| {
            let a = a.parse_meta();
            match a {
                Ok(meta) => {
                    if meta.path().is_ident(name) {
                        Some(meta)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .ok_or_else(|| {
            syn::Error::new(Span::call_site(), format!("Expect an attribute `{}`", name))
        })?;

    if let syn::Meta::List(ref list) = attr {
        let xs: Vec<_> = list
            .nested
            .iter()
            .map(|m| {
                let res = meta_item_to_ty(m).ok();
                res
            })
            .collect();
        Ok(xs)
    } else {
        Err(syn::Error::new_spanned(
            attr,
            format!("The correct syntax is #[{}(type, type, ...)]", name),
        ))
    }
}

fn meta_item_to_ty(meta_item: &syn::NestedMeta) -> syn::Result<syn::Type> {
    match meta_item {
        syn::NestedMeta::Meta(syn::Meta::Path(ref path)) => match path.get_ident() {
            Some(ident) => syn::parse_str::<syn::Type>(&ident.to_string())
                .map_err(|_| syn::Error::new_spanned(ident, "Expect type")),
            None => Err(syn::Error::new_spanned(path, "Expect type")),
        },
        meta => Err(syn::Error::new_spanned(meta, "Expect type")),
    }
}
