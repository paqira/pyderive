use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::common::{is_py, is_string, FieldData};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let mut kw_only = false;
    let assingments = data
        .iter()
        .filter(|d| d.dataclass_field.unwrap_or(true))
        .map(|d| {
            let ident = &d.field.ident;
            let field = if is_string(&d.field.ty) {
                quote! { (&sself.#ident) }
            } else {
                quote! { sself.#ident }
            };
            let pyname = &d.pyname;
            let init = &d.init.unwrap_or(true);
            let repr = &d.repr.unwrap_or(true);
            let default = &d
                .default
                .as_ref()
                .map_or(quote!(MISSING.to_object(py)), |expr| {
                    if is_py(&d.field.ty) {
                        // may never reach
                        quote!( #expr )
                    } else {
                        quote!( #expr.into_py(py) )
                    }
                });
            // once kw_only, always kw_only
            if let Some(true) = &d.kw_only {
                kw_only = true;
            }
            let field_type = if d.init.unwrap_or(true) {
                format_ident!("{}", "_FIELD")
            } else {
                format_ident!("{}", "_FIELD_CLASSVAR")
            };

            quote! {
                #[allow(unused_mut)]
                let mut field = Field.call1((
                    // default
                    #default,
                    // default_factory
                    MISSING,
                    // init
                    pyo3::types::PyBool::new(py, #init),
                    // repr
                    pyo3::types::PyBool::new(py, #repr),
                    // hash
                    py.None(),
                    // compare
                    py.None(),
                    // metadata
                    py.None(),
                    // kw_only
                    pyo3::types::PyBool::new(py, #kw_only),
                ))?;

                field.setattr(pyo3::intern!(py, "name"), pyo3::intern!(py, #pyname))?;
                field.setattr(pyo3::intern!(py, "type"), #field.to_object(py).as_ref(py).get_type())?;
                field.setattr(pyo3::intern!(py, "_field_type"), #field_type)?;
                field.call_method1(pyo3::intern!(py, "__set_name__"), (sself, pyo3::intern!(py, #pyname)))?;

                fields.set_item(pyo3::intern!(py, #pyname), field)?;
            }
        });

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            #[getter]
            fn __dataclass_fields__<'p>(slf: pyo3::PyRef<'p, Self>) -> pyo3::PyResult<&'p pyo3::types::PyDict> {
                let py = slf.py();
                let sself = std::borrow::Borrow::borrow(&slf);

                #[allow(unused_mut)]
                let mut fields = pyo3::types::PyDict::new(py);

                let dataclasses = pyo3::types::PyModule::import(py, "dataclasses")?;

                #[allow(non_snake_case)]
                let Field = dataclasses.getattr("Field")?;
                #[allow(non_snake_case)]
                let MISSING = dataclasses.getattr("MISSING")?;
                #[allow(non_snake_case)]
                let _FIELD = dataclasses.getattr("_FIELD")?;
                #[allow(non_snake_case)]
                let _FIELD_CLASSVAR = dataclasses.getattr("_FIELD_CLASSVAR")?;

                #(#assingments)*

                Ok(fields)
            }
        }
    };

    Ok(expanded.into())
}
