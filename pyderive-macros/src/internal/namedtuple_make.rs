use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let fields = data.iter().filter(|d| d.get).collect::<Vec<_>>();

    let defs = fields
        .iter()
        .filter(|d| d.get)
        .map(|d| {
            let ident = &d.field.ident;

            quote! { let #ident = iter.call_method0("__next__")?.extract()?; }
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
            #[classmethod]
            pub fn _make(
                _: &pyo3::prelude::Bound<'_, pyo3::prelude::PyAny>,
                iterable: &pyo3::prelude::Bound<'_, pyo3::prelude::PyAny>
            ) -> pyo3::prelude::PyResult<Self> {
                let iter: pyo3::prelude::Bound<'_, pyo3::prelude::PyAny> = iterable.call_method0("__iter__")?.extract()?;

                #(#defs)*

                Ok(Self{ #(#args),* })
            }
        }
    };

    Ok(expanded.into())
}
