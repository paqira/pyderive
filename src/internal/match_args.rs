use std::iter;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = input.ident.clone();
    let data = FieldData::try_from_input(&input)?;

    let names = data
        .iter()
        .filter(|d| d.match_args.unwrap_or(d.get))
        .map(|d| d.pyname.to_owned())
        .collect::<Vec<_>>();

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
