use std::iter;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{
    attr::StructOption,
    common::{is_py, FieldData},
};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = input.ident.clone();
    let struct_option = StructOption::try_from(&input.attrs)?;
    let field_data = FieldData::try_from_data(input, &struct_option)?;

    // args of foramt!(..)
    let args = field_data
        .iter()
        .filter(|d| d.get() || d.set())
        .map(|d| {
            let ident = d.ident();
            let name = d.pyname();

            if is_py(&d.ty()) {
                quote! { #name, py_ref.#ident.as_ref(py).repr()? }
            } else {
                quote! { #name, py_ref.#ident.to_object(py).as_ref(py).repr()? }
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
            pub fn __repr__(slf: &pyo3::PyCell<Self>) -> pyo3::PyResult<String> {
                let py = slf.py();
                let name: &str = slf.get_type().name()?;
                let py_ref = slf.borrow();
                let s = format!(#fmt, name, #(#args),*);
                pyo3::PyResult::Ok(s)
            }
        }
    };

    Ok(expanded.into())
}
