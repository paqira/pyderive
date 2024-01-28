use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{attr::StructOption, common::FieldData};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = input.ident.clone();
    let struct_option = StructOption::try_from(&input.attrs)?;
    let field_data = FieldData::try_from_data(input, &struct_option)?;

    let length = field_data
        .iter()
        .filter(|d| d.get())
        .collect::<Vec<_>>()
        .len();

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            pub fn __len__(&self) -> usize { #length }
        }
    };

    Ok(expanded.into())
}
