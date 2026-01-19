use pyderive_macros::*;
use pyo3::{prelude::*, types::*};

#[test]
fn test_empty() {
    #[derive(PyNamedTupleReplace)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    #[pymethods]
    impl PyClass {
        #[new]
        pub fn __new__(a: i64, b: String, c: Py<PyTuple>) -> Self {
            Self { a, b, c }
        }
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "instance = py_class(1, 'str', (1, 'a'))
actual = instance._replace()

assert actual.a == 1
assert actual.b == 'str'
assert actual.c == (1, 'a')
assert id(instance) != id(actual)
"
        );
    });
}

#[test]
fn test_less() {
    #[derive(PyNamedTupleReplace)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    #[pymethods]
    impl PyClass {
        #[new]
        pub fn __new__(a: i64, b: String, c: Py<PyTuple>) -> Self {
            Self { a, b, c }
        }
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "instance = py_class(1, 'str', (1, 'a'))
actual = instance._replace(b='x')

assert actual.a == 1
assert actual.b == 'x'
assert actual.c == (1, 'a')
"
        );
    });
}

#[test]
fn test_match() {
    #[derive(PyNamedTupleReplace)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    #[pymethods]
    impl PyClass {
        #[new]
        pub fn __new__(a: i64, b: String, c: Py<PyTuple>) -> Self {
            Self { a, b, c }
        }
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "instance = py_class(1, 'str', (1, 'a'))
actual = instance._replace(b='x', a=0, c=tuple())

assert actual.a == 0
assert actual.b == 'x'
assert actual.c == tuple()
"
        );
    });
}

#[test]
fn test_more() {
    #[derive(PyNamedTupleReplace)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    #[pymethods]
    impl PyClass {
        #[new]
        pub fn __new__(a: i64, b: String, c: Py<PyTuple>) -> Self {
            Self { a, b, c }
        }
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "instance = py_class(1, 'str', (1, 'a'))
try:
    actual = instance._replace(b='x', a=0, c=tuple(), x=None, y=None)
except PyTypeError as e:
    assert str(e) == \"Got unexpected field names: ['x', 'y']\"
else:
    assert false, 'no error found'
"
        );
    });
}
