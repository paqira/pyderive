use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn implementation(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = input.ident;

    let expanded = quote! {
        #[automatically_derived]
        impl ::pyo3::ToPyObject for #struct_name {
            fn to_object(&self, py: ::pyo3::Python<'_>) -> ::pyo3::PyObject {
                self.clone().into_py(py)
            }
        }
    };

    Ok(expanded.into())
}
