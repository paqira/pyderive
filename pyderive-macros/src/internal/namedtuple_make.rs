use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let fields = data.iter().filter(|d| d.get).collect::<Vec<_>>();

    let field_length = fields.len();

    let assignments = fields
        .iter()
        .filter(|d| d.get)
        .enumerate()
        .map(|(idx, d)| {
            let ident = &d.field.ident;

            quote! {
                let #ident = match iter.next() {
                    Some(r) => r?.extract()?,
                    None => {
                        let msg = format!("Expected {} arguments, got {}", #field_length, #idx);

                        return Err(::pyo3::exceptions::PyTypeError::new_err(msg));
                    }
                };
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
            #[classmethod]
            pub fn _make(
                _: &pyo3::prelude::Bound<'_, pyo3::types::PyType>,
                iterable: &pyo3::prelude::Bound<'_, pyo3::prelude::PyAny>
            ) -> pyo3::prelude::PyResult<Self> {
                let mut iter = iterable.try_iter()?;

                #(#assignments)*

                let mut length = #field_length;
                while iter.next().is_some() {
                    length += 1;
                }

                if #field_length < length {
                    let msg = format!("Expected {} arguments, got {}", #field_length, length);
                    return Err(::pyo3::exceptions::PyTypeError::new_err(msg));
                }

                Ok(Self{ #(#args),* })
            }
        }
    };

    Ok(expanded.into())
}
