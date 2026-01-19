use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let fields = data.iter().filter(|d| d.get).collect::<Vec<_>>();

    let assignments = fields
        .iter()
        .filter(|d| match (&d.new(), &d.default) {
            (true, None) => false,
            (true, Some(_)) => true,
            (false, None) => true,
            (false, Some(_)) => true,
        })
        .map(|d| {
            let ident = &d.field.ident.clone().unwrap().to_string();
            match (&d.new(), &d.default) {
                (true, None) => unreachable!(),
                (true, Some(default)) => quote! { dict.set_item(#ident, #default)?; },
                (false, None) => {
                    let ty = d.field.ty.to_owned();
                    quote! { dict.set_item(#ident, #ty::default())?; }
                }
                (false, Some(default)) => quote! { dict.set_item(#ident, #default)?; },
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            #[classattr]
            pub fn _field_defaults<'py>(
                py: pyo3::prelude::Python<'py>
            ) -> pyo3::prelude::PyResult<pyo3::prelude::Bound<'py, pyo3::types::PyDict>> {
                let dict = PyDict::new(py);

                #(#assignments)*

                Ok(dict)
    }
        }
    };

    Ok(expanded.into())
}
