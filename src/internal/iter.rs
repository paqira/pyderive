use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Data, DeriveInput, Fields};

use crate::common::{is_string, ClassAttrOption, FieldAttrOption};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let class_opt = ClassAttrOption::try_from_attrs(&input.attrs)?;
    let struct_name = &input.ident;
    let iter_name = format_ident!("__{}IterableWrapper", struct_name);

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            ref e => return Err(syn::Error::new(e.span(), "unnamed field is not supported")),
        },
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "#[derive(__iter__)] supports struct, not enum and union",
            ))
        }
    };

    let tokens = fields
        .named
        .iter()
        .map(|f| {
            let i = f.ident.as_ref().unwrap();
            let ty = &f.ty;
            let opt = FieldAttrOption::parse_field_attr(&f.attrs)?;
            Ok((i, ty, opt))
        })
        // propagates error
        .filter(|r| {
            r.as_ref()
                .map_or(true, |(.., opt)| opt.is_gettable(&class_opt))
        })
        .map(|r| {
            r.map(|(i, ty, ..)| {
                if is_string(&ty) {
                    quote! { (&slf.#i).to_object(py) }
                } else {
                    quote! { slf.#i.to_object(py) }
                }
            })
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;
    let args = quote! { #(#tokens),* };

    let expanded = quote! {
        #[pyclass]
        struct #iter_name {
            inner: std::vec::IntoIter<pyo3::PyObject>,
        }

        #[pymethods]
        impl #iter_name {
            pub fn __iter__(slf: pyo3::PyRef<'_, Self>) -> pyo3::PyRef<'_, Self> {
                slf
            }

            pub fn __next__(mut slf: pyo3::PyRefMut<'_, Self>) -> std::option::Option<pyo3::PyObject> {
                slf.inner.next()
            }
        }

        #[pymethods]
        impl #struct_name {
            pub fn __iter__(slf: pyo3::PyRef<'_, Self>) -> pyo3::PyResult<Py<#iter_name>> {
                let py = slf.py();
                let iter = #iter_name {
                    inner: vec![#args].into_iter(),
                };
                pyo3::Py::new(py, iter)
            }
        }
    };

    Ok(expanded.into())
}
