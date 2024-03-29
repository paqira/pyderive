use pyderive::*;
use pyo3::{prelude::*, py_run};

#[test]
fn test() {
    #[pyclass]
    #[derive(Default, Hash)]
    #[allow(dead_code)]
    #[derive(PyHash)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: String,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert hash(data) == -9000812902462168605")
    });
}

#[test]
fn test_get_set() {
    #[pyclass]
    #[derive(Default, Hash, PyHash)]
    struct PyClass {
        #[pyo3(get)]
        fd_name_a: i64,
        #[pyo3(set)]
        fd_name_b: String,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert hash(data) == -9000812902462168605")
    });
}

#[test]
fn test_get_all() {
    #[pyclass(get_all)]
    #[derive(Default, Hash, PyHash)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: String,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert hash(data) == -9000812902462168605")
    });
}

#[test]
fn test_set_all() {
    #[pyclass(set_all)]
    #[derive(Default, Hash, PyHash)]
    struct PyClass {
        #[pyo3(get)]
        fd_name_a: i64,
        fd_name_b: String,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert hash(data) == -9000812902462168605")
    });
}

#[test]
fn test_name_rename_all() {
    #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
    #[derive(Default, Hash, PyHash)]
    struct PyClass {
        #[pyo3(name = "new_name")]
        fd_name_a: i64,
        fd_name_b: String,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert hash(data) == -9000812902462168605")
    });
}

#[test]
fn test_nest_pyclass() {
    #[pyclass(get_all)]
    #[derive(Hash, PyHash)]
    struct PyClassA {
        field: PyClassB,
    }

    #[derive(PyHash, Clone)]
    #[pyclass(get_all)]
    #[derive(Hash)]
    struct PyClassB {
        field: i64,
    }

    #[pymethods]
    impl PyClassA {
        #[new]
        fn new(field: PyClassB) -> Self {
            Self { field }
        }
    }

    #[pymethods]
    impl PyClassB {
        #[new]
        fn new(field: i64) -> Self {
            Self { field }
        }
    }

    Python::with_gil(|py| {
        let py_class_a = py.get_type_bound::<PyClassA>();
        let py_class_b = py.get_type_bound::<PyClassB>();
        pyo3::py_run!(
            py,
            py_class_a py_class_b,
            r#"
a = py_class_a(py_class_b(1))

assert hash(a) ==  2206609067086327257
"#
        );
    });
}
