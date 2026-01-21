use pyderive_macros::*;
use pyo3::{prelude::*, types::*};

#[test]
fn test_list() {
    #[derive(PyNamedTupleMake)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "actual = py_class._make([1, 'str', (1, 'a')])

assert actual.a == 1
assert actual.b == 'str'
assert actual.c == (1, 'a')
"
        );
    });
}

#[test]
fn test_tuple() {
    #[derive(PyNamedTupleMake)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "actual = py_class._make((1, 'str', (1, 'a')))

assert actual.a == 1
assert actual.b == 'str'
assert actual.c == (1, 'a')
"
        );
    });
}

#[test]
fn test_empty() {
    #[derive(PyNamedTupleMake)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "try:
    py_class._make([])
except TypeError as e:
    assert str(e) == 'Expected 3 arguments, got 0'
else:
    assert fasle, 'no error found'
"
        );
    });
}

#[test]
fn test_less() {
    #[derive(PyNamedTupleMake)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "try:
    py_class._make([1, 'a'])
except TypeError as e:
    assert str(e) == 'Expected 3 arguments, got 2'
else:
    assert fasle, 'no error found'
"
        );
    });
}

#[test]
fn test_more() {
    #[derive(PyNamedTupleMake)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "try:
    py_class._make([1, 'a', (1, 2), 'a'])
except TypeError as e:
    assert str(e) == 'Expected 3 arguments, got 4'
else:
    assert fasle, 'no error found'
"
        );
    });
}

#[test]
fn test_empty_struct() {
    #[derive(PyNamedTupleMake, PartialEq, Eq)]
    #[pyclass(get_all, eq)]
    #[allow(dead_code)]
    struct PyClass {}

    #[pymethods]
    impl PyClass {
        #[new]
        pub fn __new__() -> Self {
            Self {}
        }
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(py, py_class, "assert py_class() == py_class._make([])");
    });
}
