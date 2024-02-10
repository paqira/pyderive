use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            pub fn __eq__(&self, other: &Self) -> ::std::primitive::bool { self.eq(other) }
            pub fn __ne__(&self, other: &Self) -> ::std::primitive::bool { self.ne(other) }
        }
    };

    Ok(expanded.into())
}
