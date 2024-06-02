use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let length = data.iter().filter(|d| d.len()).count();

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            pub fn __len__(&self) -> ::std::primitive::usize { #length }
        }
    };

    Ok(expanded.into())
}
