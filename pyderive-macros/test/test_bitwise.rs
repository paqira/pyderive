use std::ops::*;

use pyo3::prelude::*;

use pyderive_macros::*;

macro_rules! impl_new {
    ($struct:ident) => {
        #[pymethods]
        impl $struct {
            #[new]
            fn new(field: i64) -> Self {
                Self { field }
            }
        }
    };
}

macro_rules! impl_unary {
    ($struct:ident, $trait:ident::$method:ident) => {
        impl $trait for &$struct {
            type Output = $struct;

            fn $method(self) -> Self::Output {
                PyClass {
                    field: $trait::$method(self.field),
                }
            }
        }
    };
}

macro_rules! impl_binary {
    ($struct:ident, $trait:ident::$method:ident) => {
        impl $trait for &$struct {
            type Output = $struct;

            fn $method(self, rhs: Self) -> Self::Output {
                PyClass {
                    field: $trait::$method(self.field, rhs.field),
                }
            }
        }
    };
}

macro_rules! impl_binary_assign {
    ($struct:ident, $trait:ident::$method:ident) => {
        impl $trait<&Self> for $struct {
            fn $method(&mut self, rhs: &Self) {
                $trait::$method(&mut self.field, rhs.field);
            }
        }
    };
}

#[test]
fn test() {
    #[derive(PyBitwise)]
    #[pyclass(get_all)]
    struct PyClass {
        field: i64,
    }

    impl_new!(PyClass);
    impl_unary!(PyClass, Not::not);
    impl_binary!(PyClass, BitAnd::bitand);
    impl_binary!(PyClass, BitOr::bitor);
    impl_binary!(PyClass, BitXor::bitxor);
    impl_binary!(PyClass, Shl::shl);
    impl_binary!(PyClass, Shr::shr);
    impl_binary_assign!(PyClass, BitAndAssign::bitand_assign);
    impl_binary_assign!(PyClass, BitOrAssign::bitor_assign);
    impl_binary_assign!(PyClass, BitXorAssign::bitxor_assign);
    impl_binary_assign!(PyClass, ShlAssign::shl_assign);
    impl_binary_assign!(PyClass, ShrAssign::shr_assign);

    Python::with_gil(|py| {
        let py_class = py.get_type::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
obj = py_class(0x7fff)
other = py_class(0x3)

assert obj.field == 0x7fff 
assert (~obj).field == -32768

assert (obj & other).field == 0x3
assert (obj | other).field == 0x7fff
assert (obj ^ other).field == 0x7ffc
assert (obj << other).field == 0x3fff8
assert (obj >> other).field == 0xfff

obj = py_class(0x7fff)
obj &= other
assert obj.field == 0x3

obj = py_class(0x7fff)
obj |= other
assert obj.field == 0x7fff

obj = py_class(0x7fff)
obj ^= other
assert obj.field == 0x7ffc

obj = py_class(0x7fff)
obj <<= other
assert obj.field == 0x3fff8

obj = py_class(0x7fff)
obj >>= other
assert obj.field == 0xfff
"#
        );
    });
}
