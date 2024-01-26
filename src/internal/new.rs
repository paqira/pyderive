use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Data, DeriveInput, Fields};

use crate::common::{ClassAttrOption, FieldAttrOption};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let class_opt = ClassAttrOption::try_from_attrs(&input.attrs)?;

    let struct_name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            ref e => return Err(syn::Error::new(e.span(), "unnamed field is not supported")),
        },
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "#[derive(__init__)] supports struct, not enum and union",
            ))
        }
    };

    let init_args = fields
        .named
        .iter()
        .map(|f| {
            let i = f.ident.as_ref().unwrap();
            let ty = &f.ty;
            let name = FieldAttrOption::parse_field_attr(&f.attrs)?.py_name(&i, &class_opt);
            let name = format_ident!("{}", name);
            Ok(quote! { #name: #ty })
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    let self_args = fields
        .named
        .iter()
        .map(|f| {
            let i = f.ident.as_ref().unwrap();
            let name = FieldAttrOption::parse_field_attr(&f.attrs)?.py_name(&i, &class_opt);
            let name = format_ident!("{}", name);
            Ok(quote! { #i: #name })
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            #[new]
            pub fn __generated_python_new(
                #(#init_args),*
            ) -> Self {
                Self { #(#self_args),* }
            }
        }
    };

    Ok(expanded.into())
}
