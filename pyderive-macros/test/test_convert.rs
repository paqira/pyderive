// Note:
//   The instance python <= 10.0 does not have __complex__ attr. etc.

use pyo3::prelude::*;
use std::borrow::Cow;

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

#[test]
fn r#bool() {
    #[derive(PyBool)]
    #[pyclass(get_all)]
    struct PyClass {
        field: i64,
    }

    impl_new!(PyClass);

    impl From<&PyClass> for bool {
        fn from(value: &PyClass) -> Self {
            value.field.is_positive()
        }
    }

    Python::with_gil(|py| {
        let py_class = py.get_type::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = bool(py_class(1))
# assert hasattr(actual, "__bool__")
assert isinstance(actual, bool)
assert actual is True
"#
        );
    });
}

#[test]
fn bytes() {
    #[derive(PyBytes)]
    #[pyclass(get_all)]
    struct PyClass {
        a: u8,
        b: u8,
        c: u8,
    }

    #[pymethods]
    impl PyClass {
        #[new]
        fn new(a: u8, b: u8, c: u8) -> Self {
            Self { a, b, c }
        }
    }

    impl From<&PyClass> for Cow<'_, [u8]> {
        fn from(value: &PyClass) -> Self {
            Cow::from(vec![value.a, value.b, value.c])
        }
    }

    Python::with_gil(|py| {
        let py_class = py.get_type::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = bytes(py_class(1, 2, 3))
# assert hasattr(actual, "__bytes__")
assert isinstance(actual, bytes)
assert actual == b'\x01\x02\x03'
"#
        );
    });
}
#[test]
fn int() {
    #[derive(PyInt)]
    #[pyclass(get_all)]
    struct PyClass {
        field: i64,
    }

    impl_new!(PyClass);

    impl From<&PyClass> for i64 {
        fn from(value: &PyClass) -> Self {
            value.field
        }
    }

    Python::with_gil(|py| {
        let py_class = py.get_type::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = int(py_class(1))
# assert hasattr(actual, "__int__")
assert isinstance(actual, int)
assert actual == 1
"#
        );
    });
}

#[test]
fn index() {
    #[derive(PyIndex)]
    #[pyclass(get_all)]
    struct PyClass {
        field: i64,
    }

    impl_new!(PyClass);

    impl From<&PyClass> for isize {
        fn from(value: &PyClass) -> Self {
            value.field as isize
        }
    }

    Python::with_gil(|py| {
        let py_class = py.get_type::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = int(py_class(1))
# assert hasattr(actual, "__index__")
assert isinstance(actual, int)
assert actual == 1
"#
        );
    });
}

#[test]
fn float() {
    #[derive(PyFloat)]
    #[pyclass(get_all)]
    struct PyClass {
        field: i64,
    }

    impl_new!(PyClass);

    impl From<&PyClass> for f64 {
        fn from(value: &PyClass) -> Self {
            value.field as f64
        }
    }

    Python::with_gil(|py| {
        let py_class = py.get_type::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = float(py_class(1))
# assert hasattr(actual, "__float__")
assert isinstance(actual, float)
assert actual == 1.0
"#
        );
    });
}

#[test]
#[cfg(feature = "num-complex")]
fn complex() {
    use num_complex::Complex64;

    #[derive(PyComplex)]
    #[pyclass(get_all)]
    struct PyClass {
        field: Complex64,
    }

    #[pymethods]
    impl PyClass {
        #[new]
        fn new(field: Complex64) -> Self {
            Self { field }
        }
    }

    impl From<&PyClass> for Complex64 {
        fn from(value: &PyClass) -> Self {
            value.field * 2.0
        }
    }

    Python::with_gil(|py| {
        let py_class = py.get_type::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = complex(py_class(1.0 + 2.0j))
# assert hasattr(actual, "__complex__")
assert isinstance(actual, complex)
assert actual == 2.0 + 4.0j
"#
        );
    });
}
