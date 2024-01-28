use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{attr::StructOption, common::FieldData};

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = input.ident.clone();
    let struct_option = StructOption::try_from(&input.attrs)?;
    let field_data = FieldData::try_from_data(input, &struct_option)?;

    let init_args = field_data
        .iter()
        .map(|d| {
            let ty = d.ty();
            let pyident = d.pyident();

            quote! { #pyident: #ty }
        })
        .collect::<Vec<_>>();

    let self_args = field_data
        .iter()
        .map(|d| {
            let ident = d.ident();
            let pyident = d.pyident();

            quote! { #ident: #pyident }
        })
        .collect::<Vec<_>>();

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
