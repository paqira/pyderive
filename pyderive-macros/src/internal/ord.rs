use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            pub fn __lt__(&self, other: &Self) -> ::std::primitive::bool {
                use ::std::cmp::Ordering;
                matches!(
                    self.partial_cmp(other),
                    ::std::option::Option::Some(Ordering::Less)
                )
            }

            pub fn __le__(&self, other: &Self) -> ::std::primitive::bool {
                use ::std::cmp::Ordering;
                matches!(
                    self.partial_cmp(other),
                    ::std::option::Option::Some(Ordering::Less | Ordering::Equal)
                )
            }

            pub fn __gt__(&self, other: &Self) -> ::std::primitive::bool {
                use ::std::cmp::Ordering;
                matches!(
                    self.partial_cmp(other),
                    ::std::option::Option::Some(Ordering::Greater)
                )
            }

            pub fn __ge__(&self, other: &Self) -> ::std::primitive::bool {
                use ::std::cmp::Ordering;
                matches!(
                    self.partial_cmp(other),
                    ::std::option::Option::Some(Ordering::Greater | Ordering::Equal)
                )
            }
        }
    };

    Ok(expanded.into())
}
