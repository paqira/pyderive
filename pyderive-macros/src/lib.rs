use syn::{parse_macro_input, DeriveInput};

mod attr;
mod common;
mod internal;

#[proc_macro_derive(PyRepr, attributes(pyderive))]
pub fn py_repr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::repr::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyStr, attributes(pyderive))]
pub fn py_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::str::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyLen, attributes(pyderive))]
pub fn py_len(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::len::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyIter, attributes(pyderive))]
pub fn py_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::iter::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyReversed, attributes(pyderive))]
pub fn py_reversed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::reversed::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyNew, attributes(pyderive))]
pub fn py_new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::new::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyEq)]
pub fn py_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::eq::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyOrd)]
pub fn py_ord(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::ord::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyHash)]
pub fn py_hash(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::hash::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyMatchArgs, attributes(pyderive))]
pub fn py_match_args(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::match_args::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(PyDataclassFields, attributes(pyderive))]
pub fn py_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::dataclass_fields::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(ToPyObject)]
pub fn py_to_py_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::to_py_object::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}
