use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::common::{is_py, FieldData};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let mut kw_only = false;
    let assingments = data
        .iter()
        .filter(|d| d.dataclass_field.unwrap_or(true))
        .map(|d| {
            let ident = &d.field.ident;
            let field = quote! { slf.#ident };
            let pyname = &d.pyname;
            let init = &d.init.unwrap_or(true);
            let repr = &d.repr.unwrap_or(true);
            let default = &d.default.as_ref().map_or(quote!(MISSING), |expr| {
                if is_py(&d.field.ty) {
                    quote!( #expr )
                } else {
                    quote!( #expr.into_py(py) )
                }
            });
            // once kw_only, always kw_only
            if let Some(true) = &d.kw_only {
                kw_only = true;
            }
            // init=false -> ClassVar
            let field_type = if *init {
                format_ident!("{}", "_FIELD")
            } else {
                format_ident!("{}", "_FIELD_CLASSVAR")
            };

            quote! {
                let field_name = ::pyo3::intern!(py, #pyname);
                let field = if py.version_info() >= (3, 10) {
                    let args = (
                        // default
                        #default,
                        // default_factory
                        MISSING,
                        // init
                        ::pyo3::types::PyBool::new(py, #init),
                        // repr
                        ::pyo3::types::PyBool::new(py, #repr),
                        // hash
                        py.None(),
                        // compare
                        py.None(),
                        // metadata
                        py.None(),
                        // kw_only for python >= 3.10
                        ::pyo3::types::PyBool::new(py, #kw_only),
                    );
                    Field.call1(args)
                } else {
                    let args = (
                        // default
                        #default,
                        // default_factory
                        MISSING,
                        // init
                        ::pyo3::types::PyBool::new(py, #init),
                        // repr
                        ::pyo3::types::PyBool::new(py, #repr),
                        // hash
                        py.None(),
                        // compare
                        py.None(),
                        // metadata
                        py.None(),
                    );
                    Field.call1(args)
                }?;

                // Field does not have name, type and _field_type
                // in the constructor's arguments.
                // From dataclasses._get_field at
                // https://github.com/python/cpython/blob/ee66c333493105e014678be118850e138e3c62a8/Lib/dataclasses.py#L760-855
                field.setattr(pystr_name, field_name)?;
                field.setattr(pystr_type, #field.to_object(py).as_ref(py).get_type())?;
                field.setattr(pystr_field_type, #field_type)?;

                // From dataclasses.Field (to support the PEP 487 __set_name__ protocol) at
                // https://github.com/python/cpython/blob/ee66c333493105e014678be118850e138e3c62a8/Lib/dataclasses.py#L341-L354
                field.call_method1(pystr_set_name, (&slf, field_name))?;

                fields.set_item(field_name, field)?;
            }
        });

    // Is borrowing goot strat?
    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            #[getter]
            fn __dataclass_fields__<'p>(slf: ::pyo3::PyRef<'p, Self>) -> ::pyo3::PyResult<&'p ::pyo3::types::PyDict> {
                let py = slf.py();

                let dataclasses = ::pyo3::types::PyModule::import(py, "dataclasses")?;

                #[allow(non_snake_case)]
                let Field = dataclasses.getattr("Field")?;
                #[allow(non_snake_case)]
                let MISSING = dataclasses.getattr("MISSING")?;
                #[allow(non_snake_case)]
                let _FIELD = dataclasses.getattr("_FIELD")?;
                #[allow(non_snake_case)]
                let _FIELD_CLASSVAR = dataclasses.getattr("_FIELD_CLASSVAR")?;

                // cashe attr. names
                let pystr_name = ::pyo3::intern!(py, "name");
                let pystr_type = ::pyo3::intern!(py, "type");
                let pystr_field_type = ::pyo3::intern!(py, "_field_type");
                let pystr_set_name = ::pyo3::intern!(py, "__set_name__");

                let fields = ::pyo3::types::PyDict::new(py);

                #(#assingments)*

                Ok(fields)
            }
        }
    };

    Ok(expanded.into())
}
