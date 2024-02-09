use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let fields = data
        .iter()
        .filter(|d| d.dataclass_field())
        .collect::<Vec<_>>();

    let mut kw_only = false;
    let assingments = fields.iter().map(|d| {
        let pyname = &d.pyname;
        let init = &d.init();
        let repr = &d.repr();

        let (default, default_factory) = match &d.default {
            Some(default) => if d.default_factory() {
                let name = format!("__pyderive_internal_{}_{}_factory", struct_name, pyname);
                (quote!(MISSING), quote!(::pyo3::types::PyCFunction::new_closure(py, Some(#name), None, |_, _| #default)?))
            } else {
                (quote!(#default), quote!(MISSING))
            },
            None => (quote!(MISSING), quote!(MISSING)),
        };

        // once kw_only, always kw_only
        if d.kw_only() {
            kw_only = true;
        }

        // annotation or None
        let field_type = match d.annotation.as_ref() {
            Some(ty) => {
                let ty = format!("'{}'", ty);
                quote!( #ty )
            }
            None => quote!(py.None()),
        };

        // init=false -> ClassVar
        let dc_field_type = if *init {
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
                    #default_factory,
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
                    #default_factory,
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
            field.setattr(pystr_type, #field_type)?;
            field.setattr(pystr_field_type, #dc_field_type)?;

            // From dataclasses.Field (to support the PEP 487 __set_name__ protocol) at
            // https://github.com/python/cpython/blob/ee66c333493105e014678be118850e138e3c62a8/Lib/dataclasses.py#L341-L354
            field.call_method1(pystr_set_name, (&cls, field_name))?;

            fields.set_item(field_name, field)?;
        }
    });

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            // namely class getter,
            // it does not use field values
            #[getter]
            fn __dataclass_fields__<'p>(slf: ::pyo3::PyRef<'p, Self>) -> ::pyo3::PyResult<&'p ::pyo3::types::PyDict> {
                let py = slf.py();

                // To support __set_name__ protocol
                let cls = slf.into_py(py);
                let cls = cls.as_ref(py).get_type();

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
