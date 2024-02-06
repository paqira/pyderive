use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let assigns = data
        .iter()
        // all annotated or get/set
        .filter(|d| d.annotation.is_some())
        .map(|d| {
            let key = &d.pyname;
            let value = format!("'{}'", d.annotation.as_ref().unwrap());

            quote! {
                annotations
                    .set_item(::pyo3::intern!(py, #key), ::pyo3::intern!(py, #value))
                    .expect("fail to init __annotations__ dict");
            }
        })
        .collect::<Vec<_>>();

    let expanded = if assigns.is_empty() {
        quote! {}
    } else {
        quote! {
            #[pymethods]
                impl #struct_name {
                    #[classattr]
                    fn __annotations__(py: ::pyo3::Python<'_>) -> &::pyo3::types::PyDict {
                        let annotations = ::pyo3::types::PyDict::new(py);
                        #(#assigns)*
                        annotations
                    }
                }
        }
    };

    Ok(expanded.into())
}
