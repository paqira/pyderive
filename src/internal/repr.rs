use std::iter;

use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields};

use crate::common::{is_py, ClassAttrOption, FieldAttrOption};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let class_opt = ClassAttrOption::try_from_attrs(&input.attrs)?;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            ref e => return Err(syn::Error::new(e.span(), "unnamed field is not supported")),
        },
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "#[derive(__repr__)] supports struct, not enum and union",
            ))
        }
    };

    let struct_name = input.ident;

    // args of foramt!(..)
    let tokens = fields
        .named
        .iter()
        .map(|f| {
            let i = f.ident.as_ref().unwrap();
            let ty = &f.ty;
            let opt = FieldAttrOption::parse_field_attr(&f.attrs)?;
            Ok((i, ty, opt))
        })
        .filter(|r| {
            r.as_ref()
                .map_or(true, |(.., opt)| opt.is_visible(&class_opt))
        })
        .map(|r| {
            r.map(|(i, ty, opt)| {
                let name = opt.py_name(&i, &class_opt);
                if is_py(&ty) {
                    quote! { #name, py_ref.#i.as_ref(py).repr()? }
                } else {
                    quote! { #name, py_ref.#i.to_object(py).as_ref(py).repr()? }
                }
            })
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    // fmt of fotmat!(..)
    let fmt = iter::repeat("{}={}")
        .take(tokens.len())
        .collect::<Vec<_>>()
        .join(", ");
    let fmt = "{}(".to_string() + &fmt + ")";

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            pub fn __repr__(slf: &pyo3::PyCell<Self>) -> pyo3::PyResult<String> {
                let py = slf.py();
                let name: &str = slf.get_type().name()?;
                let py_ref = slf.borrow();
                let s = format!(#fmt, name, #(#tokens),*);
                pyo3::PyResult::Ok(s)
            }
        }
    };

    Ok(expanded.into())
}
