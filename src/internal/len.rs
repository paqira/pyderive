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
                "#[derive(__len__)] supports struct, not enum and union",
            ))
        }
    };

    let length = fields
        .named
        .iter()
        .map(|f| {
            let r = FieldAttrOption::parse_field_attr(&f.attrs)?.is_gettable(&class_opt);
            Ok(r)
        })
        // drop Ok(None)
        .filter(|r| match r {
            Ok(false) => false,
            _ => true,
        })
        .collect::<Result<Vec<_>, syn::Error>>()?
        .len();

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            pub fn __len__(&self) -> usize { #length }
        }
    };

    Ok(expanded.into())
}
