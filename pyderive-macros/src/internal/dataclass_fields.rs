use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::spanned::Spanned;
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
    let assignments = fields.iter().map(|d| {
        let pyname = &d.pyname;
        let new = &d.new();
        let repr = &d.repr();

        let (default, default_factory) = match &d.default {
            Some(default) => {
                if d.default_factory() {
                    let name = format!("pyderive_internal_{}_{}_factory\0", struct_name, pyname);

                    // name must contains exactly one null char ('\0').
                    if 1 != name.chars().filter(|c| *c == '\0').count() {
                        return Err(syn::Error::new(input.span(), "invalid struct name"));
                    }

                    (
                        quote! { MISSING.as_unbound() },
                        quote! {
                            ::pyo3::types::PyCFunction::new_closure(
                                py,
                                ::std::option::Option::Some(
                                    // make &'static CStr
                                    // we check #name contains only one \0 above 
                                    unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(#name.as_bytes()) }
                                ),
                                ::std::option::Option::None,
                                |_, _| #default
                            )?
                        },
                    )
                } else {
                    (quote! { #default }, quote! { MISSING.as_unbound() })
                }
            }
            None => (
                quote! { MISSING.as_unbound() },
                quote! { MISSING.as_unbound() },
            ),
        };

        // once kw_only, always kw_only
        if d.kw_only() {
            kw_only = true;
        }

        // annotation or None
        let annotation = match d.annotation.as_ref() {
            Some(ty) => {
                let ty = format!("'{}'", ty);
                quote! { #ty }
            }
            None => quote! { py.None() },
        };

        // new=false -> ClassVar
        let field_type = if *new {
            format_ident!("{}", "_FIELD")
        } else {
            format_ident!("{}", "_FIELD_CLASSVAR")
        };

        let r = quote! {
            let field_name = ::pyo3::intern!(py, #pyname);
            // python <= 3.9 does not have kw_only
            let field = if py.version_info() >= (3, 10) {
                let args = (
                    #default, // default
                    #default_factory, // default_factory
                    ::pyo3::types::PyBool::new(py, #new), // new
                    ::pyo3::types::PyBool::new(py, #repr), // repr
                    py.None(), // hash
                    py.None(), // compare
                    py.None(), // metadata
                    ::pyo3::types::PyBool::new(py, #kw_only), // kw_only
                );
                Field.call1(args)
            } else {
                let args = (
                    #default, // default
                    #default_factory, // default_factory
                    ::pyo3::types::PyBool::new(py, #new), // new
                    ::pyo3::types::PyBool::new(py, #repr), // repr
                    py.None(), // hash
                    py.None(), // compare
                    py.None(), // metadata
                );
                Field.call1(args)
            }?;

            // Field does not have name, type and _field_type
            // in the constructor's arguments.
            // From dataclasses._get_field at
            // https://github.com/python/cpython/blob/ee66c333493105e014678be118850e138e3c62a8/Lib/dataclasses.py#L760-855
            field.setattr(pystr_name, field_name)?;
            field.setattr(pystr_type, #annotation)?;
            field.setattr(pystr_field_type, #field_type.as_unbound())?;

            // FIXME:
            // It is not support PEP 487,
            // it is required that the default value of `__new__()`
            // and of `__dataclass_fields__` must be same objs,
            // that is, must have different IDs.
            //
            // From dataclasses.Field (to support the PEP 487 __set_name__ protocol) at
            // https://github.com/python/cpython/blob/ee66c333493105e014678be118850e138e3c62a8/Lib/dataclasses.py#L341-L354
            field.call_method1(pystr_set_name, (&cls, field_name))?;

            fields.set_item(field_name, field)?;
        };

        Ok(r)
    })
    .collect::<Result<Vec<_>, syn::Error>>()?;

    let expanded = quote! {
        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            #[classattr]
            pub fn __dataclass_fields__(py: ::pyo3::Python<'_>) -> ::pyo3::PyResult<::pyo3::Bound<'_, ::pyo3::types::PyDict>> {
                // For supporting __set_name__ protocol
                let cls = py.get_type::<Self>();

                let dataclasses = ::pyo3::types::PyModule::import(py, "dataclasses")?;

                #[allow(non_snake_case)]
                let Field = dataclasses.getattr("Field")?;
                #[allow(non_snake_case)]
                let MISSING = dataclasses.getattr("MISSING")?;
                #[allow(non_snake_case)]
                let _FIELD = dataclasses.getattr("_FIELD")?;
                #[allow(non_snake_case)]
                let _FIELD_CLASSVAR = dataclasses.getattr("_FIELD_CLASSVAR")?;

                // cache attr. names
                let pystr_name = ::pyo3::intern!(py, "name");
                let pystr_type = ::pyo3::intern!(py, "type");
                let pystr_field_type = ::pyo3::intern!(py, "_field_type");
                let pystr_set_name = ::pyo3::intern!(py, "__set_name__");

                let fields = ::pyo3::types::PyDict::new(py);

                #(#assignments)*

                Ok(fields)
            }
        }
    };

    Ok(expanded.into())
}
