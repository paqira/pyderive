use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

// #[pyderive]                          -> __new__(field):     ...
// #[pyderive(default=xxx)]             -> __new__(field=xxx): ...
// #[pyderive(new=true)]               -> __new__(field):     ...
// #[pyderive(new=false)]              -> __new__():          field=default()
// #[pyderive(new=true, default=xxx)]  -> __new__(field=xxx): ...
// #[pyderive(new=false, default=xxx)] -> __new__():          field=xxx

// For new=true
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
            .take_while(|d| !d.kw_only())
            .filter(|d| d.new())
            .map(signiture),
    );

    let rest_args = data
        .iter()
        .skip_while(|d| !d.kw_only())
        .filter(|d| d.new())
        .map(signiture)
        .collect::<Vec<_>>();

    if !rest_args.is_empty() {
        signature.push(quote! { * });
        signature.extend(rest_args);
    }

    // constructor arguments
    let new_args = data
        .iter()
        .filter(|d| d.new())
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

            if d.new() {
                quote! { #ident: #pyident }
            } else {
                match &d.default {
                    Some(expr) => quote! { #ident: #expr },
                    None => quote! { #ident: #ty::default() },
                }
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            #[new]
            #[pyo3(signature = ( #( #signature ),* ))]
            #[allow(non_snake_case)]
            pub fn __pyderive_internal_py_new(
                #(#new_args),*
            ) -> Self {
                Self { #(#self_args),* }
            }
        }
    };

    Ok(expanded.into())
}
