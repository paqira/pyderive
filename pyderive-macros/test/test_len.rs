use pyderive_macros::*;
use pyo3::{prelude::*, py_run};

#[test]
fn test_no_get_set() {
    #[derive(PyLen)]
    #[pyclass]
    #[derive(Default)]
    #[allow(dead_code)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::attach(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert len(data) == 0")
    });
}

#[test]
fn test_get_set() {
    #[derive(PyLen)]
    #[pyclass]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(get)]
        fd_name_a: i64,
        #[pyo3(set)]
        fd_name_b: f64,
    }

    Python::attach(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert len(data) == 1")
    });
}

#[test]
fn test_get_all() {
    #[derive(PyLen)]
    #[pyclass(get_all)]
    #[derive(Default)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::attach(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert len(data) == 2")
    });
}

#[test]
fn test_set_all() {
    #[derive(PyLen)]
    #[pyclass(set_all)]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(get)]
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::attach(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert len(data) == 1")
    });
}

#[test]
fn test_name_rename_all() {
    #[derive(PyLen)]
    #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(name = "new_name")]
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::attach(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert len(data) == 2")
    });
}

#[test]
fn test_pyderive_true() {
    #[derive(PyLen)]
    #[pyclass]
    #[derive(Default)]
    struct PyClass {
        #[pyderive(len)]
        #[allow(dead_code)]
        field: i64,
    }

    Python::attach(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert len(data) == 1")
    });
}

#[test]
fn test_pyderive_false() {
    #[derive(PyLen)]
    #[pyclass(get_all)]
    #[derive(Default)]
    struct PyClass {
        #[pyderive(len = false)]
        field: i64,
    }

    Python::attach(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert len(data) == 0")
    });
}

#[test]
fn test_nest_pyclass() {
    #[derive(PyLen)]
    #[pyclass(get_all)]
    struct PyClassA {
        field: PyClassB,
    }

    #[derive(PyLen, Clone)]
    #[pyclass(get_all)]
    struct PyClassB {
        field: i64,
    }

    #[pymethods]
    impl PyClassA {
        #[new]
        #[pyo3(signature=(field))]
        fn new(field: PyClassB) -> Self {
            Self { field }
        }
    }

    #[pymethods]
    impl PyClassB {
        #[new]
        #[pyo3(signature=(field))]
        fn new(field: i64) -> Self {
            Self { field }
        }
    }

    Python::attach(|py| {
        let py_class_a = py.get_type::<PyClassA>();
        let py_class_b = py.get_type::<PyClassB>();
        py_run!(
            py,
            py_class_a py_class_b,
            r#"
a = py_class_a(py_class_b(1))

assert len(a) == 1
"#
        );
    });
}
