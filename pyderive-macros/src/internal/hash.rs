use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            pub fn __hash__(&self) -> ::std::primitive::u64 {
                use ::std::collections::hash_map::DefaultHasher;
                use ::std::hash::{Hash, Hasher};

                let mut s = DefaultHasher::new();
                self.hash(&mut s);
                s.finish()
            }
        }
    };

    Ok(expanded.into())
}
