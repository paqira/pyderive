use std::iter;

use proc_macro::TokenStream;
use quote::quote;
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
                "#[derive(__match_args__)] supports struct, not enum and union",
            ))
        }
    };

    let names = fields
        .named
        .iter()
        .map(|f| {
            let i = f.ident.as_ref().unwrap();
            let opt = FieldAttrOption::parse_field_attr(&f.attrs)?;
            Ok((i, opt))
        })
        .filter(|r| {
            r.as_ref()
                .map_or(true, |(.., opt)| opt.is_gettable(&class_opt))
        })
        .map(|r| r.map(|(i, opt)| opt.py_name(&i, &class_opt)))
        .collect::<Result<Vec<_>, syn::Error>>()?;
    let types = iter::repeat(quote! { &'static str }).take(names.len());

    let expanded = if names.is_empty() {
        quote! {}
    } else {
        quote! {
            #[pymethods]
                impl #struct_name {
                    #[classattr]
                    #[allow(non_upper_case_globals)]
                    const __match_args__: (#(#types),* ,) = (#(#names),* ,);
                }
        }
    };

    Ok(expanded.into())
}
