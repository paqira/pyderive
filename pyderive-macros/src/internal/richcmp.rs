use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        #[automatically_derived]
        impl #struct_name {
            pub fn __richcmp__(
                &self,
                other: &Self,
                op: ::pyo3::pyclass::CompareOp
            ) -> ::std::primitive::bool {
                use ::std::cmp::Ordering;
                use ::pyo3::pyclass::CompareOp;
                match op {
                    CompareOp::Eq => self.eq(other),
                    CompareOp::Ne => self.ne(other),
                    CompareOp::Lt => matches!(
                        self.partial_cmp(other),
                        ::std::option::Option::Some(Ordering::Less)
                    ),
                    CompareOp::Le => matches!(
                        self.partial_cmp(other),
                        ::std::option::Option::Some(Ordering::Less | Ordering::Equal)
                    ),
                    CompareOp::Gt => matches!(
                        self.partial_cmp(other),
                        ::std::option::Option::Some(Ordering::Greater)
                    ),
                    CompareOp::Ge => matches!(
                        self.partial_cmp(other),
                        ::std::option::Option::Some(Ordering::Greater | Ordering::Equal)
                    )
                }
            }
        }
    };

    Ok(expanded.into())
}
