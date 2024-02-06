use std::iter;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::{is_py, FieldData};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    // args of foramt!(..)
    let args = data
        .iter()
        .filter(|d| d.repr.unwrap_or(d.get || d.set))
        .map(|d| {
            let ident = d.field.ident.to_owned().unwrap();
            let name = d.pyname.to_owned();

            if is_py(&d.field.ty) {
                quote! { #name, this.#ident.as_ref(py).repr()? }
            } else {
                quote! { #name, this.#ident.to_object(py).as_ref(py).repr()? }
            }
        })
        .collect::<Vec<_>>();

    // fmt of fotmat!(..)
    let fmt = iter::repeat("{}={}")
        .take(args.len())
        .collect::<Vec<_>>()
        .join(", ");
    let fmt = "{}(".to_string() + &fmt + ")";

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            pub fn __repr__(slf: &::pyo3::PyCell<Self>) -> ::pyo3::PyResult<::std::string::String> {
                let py = slf.py();
                let name = slf.get_type().name()?;
                let this = slf.borrow();
                let s = format!(#fmt, name, #(#args),*);
                ::pyo3::PyResult::Ok(s)
            }
        }
    };

    Ok(expanded.into())
}
