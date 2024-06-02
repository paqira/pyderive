use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            pub fn __lt__(&self, other: &Self) -> ::pyo3::PyResult<::std::primitive::bool> {
                use ::std::cmp::Ordering;
                match self.partial_cmp(other) {
                    ::std::option::Option::Some(Ordering::Less) => ::pyo3::PyResult::Ok(true),
                    _ => ::pyo3::PyResult::Ok(false),
                }
            }

            pub fn __le__(&self, other: &Self) -> ::pyo3::PyResult<::std::primitive::bool> {
                use ::std::cmp::Ordering;
                match self.partial_cmp(other) {
                    ::std::option::Option::Some(Ordering::Less | Ordering::Equal) => ::pyo3::PyResult::Ok(true),
                    _ => ::pyo3::PyResult::Ok(false),
                }
            }

            pub fn __gt__(&self, other: &Self) -> ::pyo3::PyResult<::std::primitive::bool> {
                use ::std::cmp::Ordering;
                match self.partial_cmp(other) {
                    ::std::option::Option::Some(Ordering::Greater) => ::pyo3::PyResult::Ok(true),
                    _ => ::pyo3::PyResult::Ok(false),
                }
            }

            pub fn __ge__(&self, other: &Self) -> ::pyo3::PyResult<::std::primitive::bool> {
                use ::std::cmp::Ordering;
                match self.partial_cmp(other) {
                    ::std::option::Option::Some(Ordering::Greater | Ordering::Equal) => ::pyo3::PyResult::Ok(true),
                    _ => ::pyo3::PyResult::Ok(false),
                }
            }
        }
    };

    Ok(expanded.into())
}
