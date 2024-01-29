use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    // #[pyderive]                          -> __init__(field):     ...
    // #[pyderive(default=xxx)]             -> __init__(field=xxx): ...
    // #[pyderive(init=true)]               -> __init__(field):     ...
    // #[pyderive(init=false)]              -> __init__():          field=default()
    // #[pyderive(init=true, default=xxx)]  -> __init__(field=xxx): ...
    // #[pyderive(init=false, default=xxx)] -> __init__():          field=xxx
    let struct_name = input.ident.clone();
    let data = FieldData::try_from_input(&input)?;

    let mut iter = data.iter();
    let mut signature = Vec::new();
    signature.extend(
        iter.by_ref()
            .take_while(|d| !d.kw_only.unwrap_or(false))
            .filter(|d| d.init.unwrap_or(true))
            .map(|d| {
                let pyident = d.pyident.to_owned();
                match &d.default {
                    Some(default) => quote! { #pyident=#default },
                    None => quote! { #pyident },
                }
            }),
    );
    signature.push(quote! { * });
    signature.extend(iter.clone().filter(|d| d.init.unwrap_or(true)).map(|d| {
        let pyident = d.pyident.to_owned();
        match &d.default {
            Some(default) => quote! { #pyident=#default },
            None => quote! { #pyident },
        }
    }));

    let init_args = data
        .iter()
        .filter(|d| d.init.unwrap_or(true))
        .map(|d| {
            let ty = d.field.ty.to_owned();
            let pyident = d.pyident.to_owned();

            quote! { #pyident: #ty }
        })
        .collect::<Vec<_>>();

    let self_args = data
        .iter()
        .map(|d| {
            let ty = d.field.ty.to_owned();
            let ident = d.field.ident.as_ref().unwrap();
            let pyident = d.pyident.to_owned();

            match &d.init {
                Some(false) => match &d.default {
                    Some(default) => quote! { #ident: #default },
                    None => quote! { #ident: #ty::default() },
                },
                _ => quote! { #ident: #pyident },
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            #[new]
            #[pyo3(
                signature = ( #( #signature ),* ),
                text_signiture = ( #( #signature ),* )
            )]
            pub fn __pyderive_new(
                #(#init_args),*
            ) -> Self {
                Self { #(#self_args),* }
            }
        }
    };

    Ok(expanded.into())
}
