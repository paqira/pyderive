use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::common::FieldData;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let data = FieldData::try_from_input(&input)?;

    let fields = data.iter().filter(|d| d.get).collect::<Vec<_>>();

    let names = fields
        .iter()
        .map(|d| {
            let ident = &d.field.ident.clone().unwrap().to_string();

            quote! { #ident }
        })
        .collect::<Vec<_>>();

    let return_type = fields
        .iter()
        .map(|_| {
            quote! { &'static str }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            #[classattr]
            pub fn _fields() -> ( #(#return_type),* ) {
                ( #(#names),* )
            }
        }
    };

    Ok(expanded.into())
}
