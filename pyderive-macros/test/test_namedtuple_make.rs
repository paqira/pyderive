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
