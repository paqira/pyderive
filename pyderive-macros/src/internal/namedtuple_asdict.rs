use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::{is_py, FieldData};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let fields = data.iter().filter(|d| d.get).collect::<Vec<_>>();

    let assignments = fields
        .iter()
        .map(|d| {
            let ident = &d.field.ident.clone().unwrap();
            let pyname = &d.pyname.to_string();

            if is_py(&d.field.ty) {
                quote! { dict.set_item(::pyo3::intern!(py, #pyname), self.#ident.clone_ref(py))?; }
            } else {
                quote! { dict.set_item(::pyo3::intern!(py, #pyname), (&self.#ident).into_pyobject(py)?)?; }
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            pub fn _asdict<'py>(
                &self,
                py: ::pyo3::prelude::Python<'py>
            ) -> ::pyo3::prelude::PyResult<::pyo3::prelude::Bound<'py, ::pyo3::types::PyDict>> {
                let dict = ::pyo3::types::PyDict::new(py);

                #(#assignments)*

                Ok(dict)
    }
        }
    };

    Ok(expanded.into())
}
