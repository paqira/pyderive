use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::common::{is_py, FieldData};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let iter_name = format_ident!("PyderiveInternalIteratorFor{}", struct_name);

    let args = data
        .iter()
        .filter(|d| d.iter())
        .map(|d| {
            let ident = &d.field.ident;

            if is_py(&d.field.ty) {
                quote! { (&slf.#ident).clone_ref(py).into_any() }
            } else {
                quote! { (&slf.#ident).into_pyobject(py)?.into_any().unbind() }
            }
        })
        .collect::<Vec<_>>();
    let length = args.len();

    let expanded = quote! {
        #[pyclass]
        #[pyo3(name="pyderive_iterator")]
        #[automatically_derived]
        struct #iter_name {
            inner: ::std::sync::Mutex<::std::array::IntoIter<::pyo3::PyObject, #length>>,
        }

        #[pymethods]
        #[automatically_derived]
        impl #iter_name {
            pub fn __iter__(slf: ::pyo3::PyRef<'_, Self>) -> ::pyo3::PyRef<'_, Self> {
                slf
            }
            pub fn __next__(mut slf: ::pyo3::PyRefMut<'_, Self>) -> ::std::option::Option<::pyo3::PyObject> {
                slf.inner.lock().unwrap().next()
            }
        }

        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            pub fn __iter__(slf: ::pyo3::PyRef<'_, Self>) -> ::pyo3::PyResult<::pyo3::Py<#iter_name>> {
                let py = slf.py();
                let iter = #iter_name {
                    inner: ::std::sync::Mutex::from(
                        [ #(#args),* ].into_iter()
                    ),
                };

                ::pyo3::Py::new(py, iter)
            }
        }
    };

    Ok(expanded.into())
}
