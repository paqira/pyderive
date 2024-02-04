use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            pub fn __lt__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
                use std::cmp::Ordering;
                match self.partial_cmp(other) {
                    Some(Ordering::Less) => Ok(true),
                    _ => Ok(false),
                }
            }

            pub fn __le__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
                use std::cmp::Ordering;
                match self.partial_cmp(other) {
                    Some(Ordering::Less | Ordering::Equal) => Ok(true),
                    _ => Ok(false),
                }
            }

            pub fn __gt__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
                use std::cmp::Ordering;
                match self.partial_cmp(other) {
                    Some(Ordering::Greater) => Ok(true),
                    _ => Ok(false),
                }
            }

            pub fn __ge__(&self, other: &Self) -> ::pyo3::PyResult<bool> {
                use std::cmp::Ordering;
                match self.partial_cmp(other) {
                    Some(Ordering::Greater | Ordering::Equal) => Ok(true),
                    _ => Ok(false),
                }
            }
        }
    };

    Ok(expanded.into())
}
