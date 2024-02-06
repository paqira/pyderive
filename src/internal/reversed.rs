use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = input.ident.clone();
    let data = FieldData::try_from_input(&input)?;

    let iter_name = format_ident!("Pyderive{}ReversedIterableWrapper", struct_name);

    let args = data
        .iter()
        .filter(|d| d.iter.unwrap_or(d.get))
        .map(|d| {
            let ident = &d.field.ident;

            quote! { this.#ident.to_object(py) }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[pyclass]
        struct #iter_name {
            inner: ::std::iter::Rev<::std::vec::IntoIter<pyo3::PyObject>>,
        }

        #[pymethods]
        impl #iter_name {
            pub fn __iter__(slf: ::pyo3::PyRef<'_, Self>) -> ::pyo3::PyRef<'_, Self> {
                slf
            }

            pub fn __next__(mut slf: ::pyo3::PyRefMut<'_, Self>) -> ::std::option::Option<::pyo3::PyObject> {
                slf.inner.next()
            }
        }

        #[pymethods]
        impl #struct_name {
            pub fn __reversed__(slf: ::pyo3::PyRef<'_, Self>) -> ::pyo3::PyResult<::pyo3::Py<#iter_name>> {
                let py = slf.py();
                let this = ::std::borrow::Borrow::borrow(&slf);
                let iter = #iter_name {
                    inner: ::std::vec![ #(#args),* ].into_iter().rev(),
                };
                ::pyo3::Py::new(py, iter)
            }
        }
    };

    Ok(expanded.into())
}
