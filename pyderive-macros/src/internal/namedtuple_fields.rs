use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let fields = data.iter().filter(|d| d.get).collect::<Vec<_>>();

    let args = fields
        .iter()
        .map(|d| {
            let ident = &d.field.ident.clone().unwrap().to_string();

            quote! { #ident }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            #[classattr]
            pub fn _fields<'py>(
                py: pyo3::prelude::Python<'py>
            ) -> pyo3::prelude::PyResult<pyo3::prelude::Bound<'py, pyo3::types::PyTuple>> {
                pyo3::types::PyTuple::new(py, [ #(#args),* ])
            }
        }
    };

    Ok(expanded.into())
}
