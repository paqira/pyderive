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
        let py_class = py.get_type_bound::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = bool(py_class(1))
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
        field: i64,
    }

    impl_new!(PyClass);

    impl From<&PyClass> for Cow<'_, [u8]> {
        fn from(_value: &PyClass) -> Self {
            vec![0, 1, 2, 3, 4, 5].into()
        }
    }

    Python::with_gil(|py| {
        let py_class = py.get_type_bound::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = bytes(py_class(1))
assert isinstance(actual, bytes)
assert actual == b'\x00\x01\x02\x03\x04\x05'
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
        let py_class = py.get_type_bound::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = int(py_class(1))
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
        let py_class = py.get_type_bound::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = int(py_class(1))
# assert isinstance(actual, int)
# assert actual == 1
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
        let py_class = py.get_type_bound::<PyClass>();
        pyo3::py_run!(
            py,
            py_class,
            r#"
actual = float(py_class(1))
assert isinstance(actual, float)
assert actual == 1.0
"#
        );
    });
}