use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

// #[pyderive]                          -> __new__(field):     ...
// #[pyderive(default=xxx)]             -> __new__(field=xxx): ...
// #[pyderive(init=true)]               -> __new__(field):     ...
// #[pyderive(init=false)]              -> __new__():          field=default()
// #[pyderive(init=true, default=xxx)]  -> __new__(field=xxx): ...
// #[pyderive(init=false, default=xxx)] -> __new__():          field=xxx

// For init=true
fn signiture(d: &FieldData) -> proc_macro2::TokenStream {
    let pyident = &d.pyident;
    match &d.default {
        Some(expr) => quote! { #pyident=#expr },
        None => quote! { #pyident },
    }
}

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    // #[pyo3(signature=..)]
    let mut signature = Vec::new();

    signature.extend(
        data.iter()
            .take_while(|d| !d.kw_only.unwrap_or(false))
            .filter(|d| d.init.unwrap_or(true))
            .map(signiture),
    );

    let rest_args = data
        .iter()
        .skip_while(|d| !d.kw_only.unwrap_or(false))
        .map(signiture)
        .collect::<Vec<_>>();

    if !rest_args.is_empty() {
        signature.push(quote! { * });
        signature.extend(rest_args);
    }

    // constructor arguments
    let init_args = data
        .iter()
        .filter(|d| d.init.unwrap_or(true))
        .map(|d| {
            let ty = d.field.ty.to_owned();
            let pyident = d.pyident.to_owned();

            quote! { #pyident: #ty }
        })
        .collect::<Vec<_>>();

    // Self arguments
    let self_args = data
        .iter()
        .map(|d| {
            let ty = d.field.ty.to_owned();
            let ident = d.field.ident.as_ref().unwrap();
            let pyident = d.pyident.to_owned();

            match &d.init {
                Some(false) => match &d.default {
                    Some(expr) => quote! { #ident: #expr },
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
            #[pyo3(signature = ( #( #signature ),* ))]
            #[allow(non_snake_case)]
            pub fn __pyderive_new(
                #(#init_args),*
            ) -> Self {
                Self { #(#self_args),* }
            }
        }
    };

    Ok(expanded.into())
}
