use pyderive_macros::*;
use pyo3::{prelude::*, types::*};

#[test]
fn test() {
    #[derive(PyNamedTupleAsdict)]
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
            "actual = py_class(1, 'str', (1, 'a'))

assert actual._asdict() == {'a': 1, 'b': 'str', 'c': (1, 'a')}
"
        );
    });
}

#[test]
fn test_empty() {
    #[derive(PyNamedTupleAsdict)]
    #[pyclass(get_all)]
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

        pyo3::py_run!(
            py,
            py_class,
            "actual = py_class()

assert actual._asdict() == {}
"
        );
    });
}

#[test]
fn test_rename() {
    #[derive(PyNamedTupleAsdict)]
    #[pyclass(get_all, rename_all = "camelCase")]
    #[allow(dead_code)]
    struct PyClass {
        #[pyo3(name = "renamed_field")]
        a: i64,
        aaa_bbb_ccc: String,
    }

    #[pymethods]
    impl PyClass {
        #[new]
        pub fn __new__(a: i64, aaa_bbb_ccc: String) -> Self {
            Self { a, aaa_bbb_ccc }
        }
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "actual = py_class(1, 'str')

assert actual._asdict() == {'renamed_field': 1, 'aaaBbbCcc': 'str'}
"
        );
    });
}
