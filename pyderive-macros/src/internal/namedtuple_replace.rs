use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::{is_py, FieldData};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let fields = data.iter().filter(|d| d.get).collect::<Vec<_>>();

    let args_nokwargs = fields
        .iter()
        .map(|d| {
            let ident = &d.field.ident.clone();

            if is_py(&d.field.ty) {
                quote! { #ident: self.#ident.clone_ref(py) }
            } else {
                quote! { #ident: self.#ident.clone() }
            }
        })
        .collect::<Vec<_>>();

    let names = fields
        .iter()
        .map(|d| {
            let name = d.field.ident.clone().unwrap().to_string();
            quote! { #name.to_string() }
        })
        .collect::<Vec<_>>();

    let assignments = fields
        .iter()
        .map(|d| {
            let ident = &d.field.ident.clone().unwrap();
            let ident_str = ident.to_string();

            if is_py(&d.field.ty) {
                quote! {
                   let #ident =  match ::pyo3::prelude::PyAnyMethods::get_item(kwargs_any, #ident_str) {
                       Ok(r) => r.extract()?,
                       Err(_) => self.#ident.clone_ref(py)
                   };
                }
            } else {
                quote! {
                   let #ident =  match ::pyo3::prelude::PyAnyMethods::get_item(kwargs_any, #ident_str) {
                       Ok(r) => r.extract()?,
                       Err(_) => self.#ident.clone()
                   };
                }
            }
        })
        .collect::<Vec<_>>();

    let args = fields
        .iter()
        .map(|d| {
            let ident = &d.field.ident;
            quote! { #ident }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            #[pyo3(signature = (**kwargs))]
            pub fn _replace<'py>(
                &self,
                py: ::pyo3::prelude::Python<'py>,
                kwargs: Option<&::pyo3::prelude::Bound<'py, ::pyo3::types::PyDict>>,
            ) -> ::pyo3::prelude::PyResult<Self> {
                match kwargs {
                    None => {
                        return {
                            Ok(Self { #(#args_nokwargs),* })
                        }
                    }
                    Some(kwargs) => {
                        let mut unknown_keys = Vec::new();

                        for key in kwargs.keys() {
                            let str_key = key.extract::<String>()?;
                            if ![ #(#names),* ].contains(&str_key) {
                                unknown_keys.push(str_key);
                            }
                        }

                        if !unknown_keys.is_empty() {
                            let py_unknown_keys = unknown_keys.into_pyobject(py)?;
                            let py_unknown_keys_repr = py_unknown_keys.repr()?;
                            let names = py_unknown_keys_repr.to_str()?;
                            let msg = format!("Got unexpected field names: {}", names);
                            return Err(::pyo3::exceptions::PyTypeError::new_err(msg));
                        }

                        let kwargs_any = kwargs.as_any();

                        #(#assignments)*

                        Ok(Self{ #(#args),* })
                    }
                }
            }
        }
    };

    Ok(expanded.into())
}
