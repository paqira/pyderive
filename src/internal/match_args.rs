use std::iter;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{attr::StructOption, common::FieldData};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = input.ident.clone();
    let struct_option = StructOption::try_from(&input.attrs)?;
    let field_data = FieldData::try_from_data(input, &struct_option)?;

    let names = field_data
        .iter()
        .filter(|d| d.get())
        .map(|d| d.pyname())
        .collect::<Vec<_>>();

    let types = iter::repeat(quote! { &'static str }).take(names.len());

    let expanded = if names.is_empty() {
        quote! {}
    } else {
        quote! {
            #[pymethods]
                impl #struct_name {
                    #[classattr]
                    const __match_args__: (#(#types),* ,) = (#(#names),* ,);
                }
        }
    };

    Ok(expanded.into())
}
