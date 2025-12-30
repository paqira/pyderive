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
    #[derive(PyNumeric)]
    #[pyclass(get_all)]
    struct PyClass {
        field: i64,
    }

    impl_new!(PyClass);
    impl_unary!(PyClass, Neg::neg);
    impl_binary!(PyClass, Add::add);
    impl_binary!(PyClass, Sub::sub);
    impl_binary!(PyClass, Mul::mul);
    impl_binary!(PyClass, Div::div);
    impl_binary!(PyClass, Rem::rem);
    impl_binary_assign!(PyClass, AddAssign::add_assign);
    impl_binary_assign!(PyClass, SubAssign::sub_assign);
    impl_binary_assign!(PyClass, MulAssign::mul_assign);
    impl_binary_assign!(PyClass, DivAssign::div_assign);
    impl_binary_assign!(PyClass, RemAssign::rem_assign);

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
obj = py_class(5)
other = py_class(3)

assert obj.field == 5
assert (+obj).field == 5
assert (-obj).field == -5
assert id(obj) == id(+obj)

assert (obj + other).field == 8
assert (obj - other).field == 2
assert (obj * other).field == 15
assert (obj / other).field == 1
assert (obj % other).field == 2

a, b = divmod(obj, other) 
assert a.field == 1
assert b.field == 2

obj = py_class(5)
obj += other
assert obj.field == 8

obj = py_class(5)
obj -= other
assert obj.field == 2

obj = py_class(5)
obj *= other
assert obj.field == 15

obj = py_class(5)
obj /= other
assert obj.field == 1

obj = py_class(5)
obj %= other
assert obj.field == 2
"#
        );
    });
}
