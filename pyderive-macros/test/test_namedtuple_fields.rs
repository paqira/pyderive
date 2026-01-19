use pyderive_macros::*;
use pyo3::{prelude::*, types::*};

#[test]
fn test() {
    #[derive(PyNamedTupleFields)]
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
            "assert py_class._fields == ('a', 'b', 'c')
"
        );
    });
}

#[test]
fn test_empty() {
    #[derive(PyNamedTupleFields)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {}

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "assert py_class._fields == tuple()
"
        );
    });
}
