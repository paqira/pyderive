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

// ops

macro_rules! impl_unary {
    ($derive:ident, $name:ident, $pyname:ident, $trait:ident::$method:ident) => {
        #[proc_macro_derive($derive)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            use quote::quote;

            let input = parse_macro_input!(input as DeriveInput);

            let struct_name = &input.ident;

            let expanded = quote! {
                #[pymethods]
                impl #struct_name {
                    pub fn $pyname(&self) -> <&Self as $trait>::Output {
                        use std::ops::$trait;
                        $trait::$method(self)
                    }
                }
            };

            expanded.into()
        }
    };
}

macro_rules! impl_binary {
    ($derive:ident, $name:ident, $pyname:ident, $trait:ident::$method:ident) => {
        #[proc_macro_derive($derive)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            use quote::quote;

            let input = parse_macro_input!(input as DeriveInput);

            let struct_name = &input.ident;

            let expanded = quote! {
                #[pymethods]
                impl #struct_name {
                    fn $pyname(&self, other: &Self) -> <&Self as $trait<&Self>>::Output {
                        use ::std::ops::$trait;
                        $trait::$method(self, other)
                    }
                }
            };

            expanded.into()
        }
    };
}

macro_rules! impl_reflected_binary {
    ($derive:ident, $name:ident, $pyname:ident, $trait:ident::$method:ident) => {
        #[proc_macro_derive($derive)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            use quote::quote;

            let input = parse_macro_input!(input as DeriveInput);

            let struct_name = &input.ident;

            let expanded = quote! {
                #[pymethods]
                impl #struct_name {
                    fn $pyname(&self, other: &Self) -> <&Self as $trait<&Self>>::Output {
                        use ::std::ops::$trait;
                        $trait::$method(other, self)
                    }
                }
            };

            expanded.into()
        }
    };
}

macro_rules! impl_binary_assign {
    ($derive:ident, $name:ident, $pyname:ident, $trait:ident::$method:ident) => {
        #[proc_macro_derive($derive)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            use quote::quote;

            let input = parse_macro_input!(input as DeriveInput);

            let struct_name = &input.ident;

            let expanded = quote! {
                #[pymethods]
                impl #struct_name {
                    fn $pyname(&mut self, other: &Self) {
                        use ::std::ops::$trait;
                        $trait::$method(self, other);
                    }
                }
            };

            expanded.into()
        }
    };
}

impl_unary!(PyNeg, py_neg, __neg__, Neg::neg);
impl_unary!(PyInvert, py_invert, __invert__, Not::not);

impl_binary!(PyAdd, py_add, __add__, Add::add);
impl_binary!(PySub, py_sub, __sub__, Sub::sub);
impl_binary!(PyMul, py_mul, __mul__, Mul::mul);
impl_binary!(PyMatMul, py_matmul, __matmul__, Mul::mul);
impl_binary!(PyTrueDiv, py_truediv, __truediv__, Div::div);
impl_binary!(PyFloorDiv, py_floordiv, __floordiv__, Div::div);
impl_binary!(PyMod, py_mod, __mod__, Rem::rem);

impl_binary!(PyLeftShift, py_lshift, __lshift__, Shl::shl);
impl_binary!(PyRightShift, py_rshift, __rshift__, Shr::shr);

impl_binary!(PyAnd, py_and, __and__, BitAnd::bitand);
impl_binary!(PyOr, py_or, __or__, BitOr::bitor);
impl_binary!(PyXor, py_xor, __xor__, BitXor::bitxor);

impl_reflected_binary!(PyReflectedAdd, py_radd, __radd__, Add::add);
impl_reflected_binary!(PyReflectedSub, py_rsub, __rsub__, Sub::sub);
impl_reflected_binary!(PyReflectedMul, py_rmul, __rmul__, Mul::mul);
impl_reflected_binary!(PyReflectedMatMul, py_rmatmul, __rmatmul__, Mul::mul);
impl_reflected_binary!(PyReflectedTrueDiv, py_rtruediv, __rtruediv__, Div::div);
impl_reflected_binary!(PyReflectedFloorDiv, py_rfloordiv, __rfloordiv__, Div::div);
impl_reflected_binary!(PyReflectedMod, py_rmod, __rmod__, Rem::rem);

impl_reflected_binary!(PyReflectedLeftShift, py_rlshift, __rlshift__, Shl::shl);
impl_reflected_binary!(PyReflectedRightShift, py_rrshift, __rrshift__, Shr::shr);

impl_reflected_binary!(PyReflectedAnd, py_rand, __rand__, BitAnd::bitand);
impl_reflected_binary!(PyReflectedOr, py_ror, __ror__, BitOr::bitor);
impl_reflected_binary!(PyReflectedXor, py_rxor, __rxor__, BitXor::bitxor);

impl_binary_assign!(PyAddAssign, py_iadd, __iadd__, AddAssign::add_assign);
impl_binary_assign!(PySubAssign, py_isub, __isub__, SubAssign::sub_assign);
impl_binary_assign!(PyMulAssign, py_imul, __imul__, MulAssign::mul_assign);
impl_binary_assign!(
    PyMatMulAssign,
    py_imatmul,
    __imatmul__,
    MulAssign::mul_assign
);
impl_binary_assign!(
    PyTrueDivAssign,
    py_itruediv,
    __itruediv__,
    DivAssign::div_assign
);
impl_binary_assign!(
    PyFloorDivAssign,
    py_ifloordiv,
    __ifloordiv__,
    DivAssign::div_assign
);
impl_binary_assign!(PyModAssign, py_imod, __imod__, RemAssign::rem_assign);

impl_binary_assign!(
    PyLeftShiftAssign,
    py_ilshift,
    __ilshift__,
    ShlAssign::shl_assign
);
impl_binary_assign!(
    PyRightShiftAssign,
    py_irshift,
    __irshift__,
    ShrAssign::shr_assign
);

impl_binary_assign!(PyAndAssign, py_iand, __iand__, BitAndAssign::bitand_assign);
impl_binary_assign!(PyOrAssign, py_ior, __ior__, BitOrAssign::bitor_assign);
impl_binary_assign!(PyXorAssign, py_ixor, __ixor__, BitXorAssign::bitxor_assign);

// FIXME: return exact `self` (same id obj)
#[proc_macro_derive(PyPos)]
pub fn py_pos(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use quote::quote;

    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            fn __pos__(self_: PyRef<'_, Self>) -> PyRef<'_, Self> {
                self_
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(PyDivMod)]
pub fn py_divmod(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use quote::quote;

    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            fn __divmod__(&self, other: &Self) -> (
                <&Self as Div<&Self>>::Output,
                <&Self as Rem<&Self>>::Output
            ) {
                use std::ops::{Div, Rem};
                (Div::div(self, other), Rem::rem(self, other))
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(PyReflectedDivMod)]
pub fn py_rdivmod(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use quote::quote;

    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            fn __rdivmod__(&self, other: &Self) -> (
                <&Self as Div<&Self>>::Output,
                <&Self as Rem<&Self>>::Output
            ) {
                use ::std::ops::{Div, Rem};
                (Div::div(other, self), Rem::rem(other, self))
            }
        }
    };

    expanded.into()
}

// convert

macro_rules! impl_convert {
    ($derive:ident, $name:ident, $pyname:ident, $ty:ty) => {
        #[proc_macro_derive($derive)]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            use quote::quote;

            let input = parse_macro_input!(input as DeriveInput);

            let struct_name = &input.ident;

            let expanded = quote! {
                #[pymethods]
                impl #struct_name {
                    fn $pyname(&self) -> $ty {
                        Into::into(self)
                    }
                }
            };
            expanded.into()
        }
    };
}

impl_convert!(PyBool, py_bool, __bool__, bool);
impl_convert!(PyInt, py_int, __int__, i64);
impl_convert!(PyIndex, py_index, __index__, isize);
impl_convert!(PyFloat, py_float, __float__, f64);

#[proc_macro_derive(PyBytes)]
pub fn py_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use quote::quote;

    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let expanded = quote! {
        #[pymethods]
        impl #struct_name {
            fn __bytes__(&self) -> Cow<[u8]> {
                use ::std::borrow::Cow;
                Into::into(self)
            }
        }
    };

    expanded.into()
}
