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

#[cfg(test)]
mod unary {
    use super::*;

    #[test]
    fn neg() {
        #[derive(PyNeg)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_unary!(PyClass, Neg::neg);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = -py_class(1)
assert actual.field == -1
"#
            );
        });
    }

    #[test]
    fn invert() {
        #[derive(PyInvert)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_unary!(PyClass, Not::not);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = ~py_class(1)
assert actual.field == -2
"#
            );
        });
    }

    #[test]
    fn pos() {
        #[derive(PyPos)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = +py_class(1)
assert actual.field == 1
"#
            );
        });
    }
}

#[cfg(test)]
mod binary {
    use super::*;

    #[test]
    fn add() {
        #[derive(PyAdd)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Add::add);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(1) + py_class(1)
assert actual.field == 2
"#
            );
        });
    }

    #[test]
    fn sub() {
        #[derive(PySub)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Sub::sub);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(1) - py_class(1)
assert actual.field == 0
"#
            );
        });
    }

    #[test]
    fn mul() {
        #[derive(PyMul)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Mul::mul);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(2) * py_class(3)
assert actual.field == 6
"#
            );
        });
    }

    #[test]
    fn true_div() {
        #[derive(PyTrueDiv)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Div::div);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(6) / py_class(3)
assert actual.field == 2
"#
            );
        });
    }

    #[test]
    fn floor_div() {
        #[derive(PyFloorDiv)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Div::div);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(6) // py_class(3)
assert actual.field == 2
"#
            );
        });
    }

    #[test]
    fn r#mod() {
        #[derive(PyMod)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Rem::rem);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7) % py_class(3)
assert actual.field == 1
"#
            );
        });
    }

    #[test]
    fn lshift() {
        #[derive(PyLeftShift)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Shl::shl);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(100) << py_class(3)
assert actual.field == 800
"#
            );
        });
    }

    #[test]
    fn rshift() {
        #[derive(PyRightShift)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Shr::shr);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(100) >> py_class(3)
assert actual.field == 12
"#
            );
        });
    }

    #[test]
    fn and() {
        #[derive(PyAnd)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, BitAnd::bitand);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7) & py_class(3)
assert actual.field == 3
"#
            );
        });
    }

    #[test]
    fn or() {
        #[derive(PyOr)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, BitOr::bitor);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7) | py_class(3)
assert actual.field == 7
"#
            );
        });
    }

    #[test]
    fn xor() {
        #[derive(PyXor)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, BitXor::bitxor);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7) ^ py_class(3)
assert actual.field == 4
"#
            );
        });
    }
}

#[cfg(test)]
mod assign {
    use super::*;

    #[test]
    fn add_assign() {
        #[derive(PyAddAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, AddAssign::add_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(1)
actual += py_class(1)
assert actual.field == 2
"#
            );
        });
    }

    #[test]
    fn sub_assign() {
        #[derive(PySubAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, SubAssign::sub_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(1)
actual -= py_class(1)
assert actual.field == 0
"#
            );
        });
    }

    #[test]
    fn mul_assign() {
        #[derive(PyMulAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, MulAssign::mul_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(2)
actual *= py_class(3)
assert actual.field == 6
"#
            );
        });
    }

    #[test]
    fn true_div_assign() {
        #[derive(PyTrueDivAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, DivAssign::div_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(6)
actual /= py_class(3)
assert actual.field == 2
"#
            );
        });
    }

    #[test]
    fn floor_div_assign() {
        #[derive(PyFloorDivAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, DivAssign::div_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(6)
actual //= py_class(3)
assert actual.field == 2
"#
            );
        });
    }

    #[test]
    fn mod_assign() {
        #[derive(PyModAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, RemAssign::rem_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7)
actual %= py_class(3)
assert actual.field == 1
"#
            );
        });
    }

    #[test]
    fn lshift_assign() {
        #[derive(PyLeftShiftAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, ShlAssign::shl_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(100)
actual <<= py_class(3)
assert actual.field == 800
"#
            );
        });
    }

    #[test]
    fn rshift_assign() {
        #[derive(PyRightShiftAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, ShrAssign::shr_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(100)
actual >>= py_class(3)
assert actual.field == 12
"#
            );
        });
    }

    #[test]
    fn and_assign() {
        #[derive(PyAndAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, BitAndAssign::bitand_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7)
actual &= py_class(3)
assert actual.field == 3
"#
            );
        });
    }

    #[test]
    fn or_assign() {
        #[derive(PyOrAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, BitOrAssign::bitor_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7)
actual |= py_class(3)
assert actual.field == 7
"#
            );
        });
    }

    #[test]
    fn xor_assign() {
        #[derive(PyXorAssign, Clone)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary_assign!(PyClass, BitXorAssign::bitxor_assign);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7)
actual ^= py_class(3)
assert actual.field == 4
"#
            );
        });
    }
}

#[cfg(test)]
mod reflect {
    use super::*;

    #[test]
    fn add() {
        #[derive(PyReflectedAdd)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Add::add);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(1) + py_class(1)
assert actual.field == 2
"#
            );
        });
    }

    #[test]
    fn sub() {
        #[derive(PyReflectedSub)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Sub::sub);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(2) - py_class(1)
assert actual.field == 1
"#
            );
        });
    }

    #[test]
    fn mul() {
        #[derive(PyReflectedMul)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Mul::mul);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(2) * py_class(3)
assert actual.field == 6
"#
            );
        });
    }

    #[test]
    fn true_div() {
        #[derive(PyReflectedTrueDiv)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Div::div);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(6) / py_class(3)
assert actual.field == 2
"#
            );
        });
    }

    #[test]
    fn floor_div() {
        #[derive(PyReflectedFloorDiv)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Div::div);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(6) // py_class(3)
assert actual.field == 2
"#
            );
        });
    }

    #[test]
    fn r#mod() {
        #[derive(PyReflectedMod)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Rem::rem);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7) % py_class(3)
assert actual.field == 1
"#
            );
        });
    }

    #[test]
    fn lshift() {
        #[derive(PyReflectedLeftShift)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Shl::shl);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(100) << py_class(3)
assert actual.field == 800
"#
            );
        });
    }

    #[test]
    fn rshift() {
        #[derive(PyReflectedRightShift)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, Shr::shr);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(100) >> py_class(3)
assert actual.field == 12
"#
            );
        });
    }

    #[test]
    fn and() {
        #[derive(PyReflectedAnd)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, BitAnd::bitand);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7) & py_class(3)
assert actual.field == 3
"#
            );
        });
    }

    #[test]
    fn or() {
        #[derive(PyReflectedOr)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, BitOr::bitor);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7) | py_class(3)
assert actual.field == 7
"#
            );
        });
    }

    #[test]
    fn xor() {
        #[derive(PyReflectedXor)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        impl_new!(PyClass);
        impl_binary!(PyClass, BitXor::bitxor);

        Python::with_gil(|py| {
            let py_class = py.get_type_bound::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
actual = py_class(7) ^ py_class(3)
assert actual.field == 4
"#
            );
        });
    }
}
