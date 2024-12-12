use pyderive_macros::*;
use pyo3::{prelude::*, py_run};

#[test]
fn test_no_get_set() {
    #[derive(PyReversed)]
    #[pyclass]
    #[derive(Default)]
    #[allow(dead_code)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(reversed(data)) == ()")
    });
}

#[test]
fn test_get_set() {
    #[derive(PyReversed)]
    #[pyclass]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(get)]
        fd_name_a: i64,
        #[pyo3(set)]
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(reversed(data)) == (0, )")
    });
}

#[test]
fn test_get_all() {
    #[derive(PyReversed)]
    #[pyclass(get_all)]
    #[derive(Default)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(reversed(data)) == (0, 0.0)")
    });
}

#[test]
fn test_set_all() {
    #[derive(PyReversed)]
    #[pyclass(set_all)]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(get)]
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(reversed(data)) == (0, )")
    });
}

#[test]
fn test_name_rename_all() {
    #[derive(PyReversed)]
    #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(name = "new_name")]
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(reversed(data)) == (0.0, 0)")
    });
}

#[test]
fn test_pyderive_true() {
    #[derive(PyReversed, Default)]
    #[pyclass]
    struct PyClass {
        #[pyderive(iter)]
        field: i64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(reversed(data)) == (0, )")
    });
}

#[test]
fn test_pyderive_false() {
    #[derive(PyReversed, Default)]
    #[pyclass(get_all)]
    struct PyClass {
        #[pyderive(iter = false)]
        field: i64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(reversed(data)) == tuple()")
    });
}

#[test]
fn test_nest_pyclass() {
    #[derive(PyReversed)]
    #[pyclass(get_all)]
    struct PyClassA {
        field_1: Py<PyClassB>,
        field_2: i64,
    }

    #[derive(PyReversed, Clone)]
    #[pyclass(get_all)]
    #[derive(PartialEq)]
    struct PyClassB {
        field: i64,
    }

    #[pymethods]
    impl PyClassA {
        #[new]
        #[pyo3(signature=(field_1, field_2))]
        fn new<'py>(py: Python<'py>, field_1: PyClassB, field_2: i64) -> Self {
            Self {
                field_1: field_1.into_pyobject(py).unwrap().unbind(),
                field_2,
            }
        }
    }

    #[pymethods]
    impl PyClassB {
        #[new]
        #[pyo3(signature=(field))]
        fn new(field: i64) -> Self {
            Self { field }
        }

        fn __eq__(&self, other: &Self) -> bool {
            self.eq(other)
        }
    }

    Python::with_gil(|py| {
        let py_class_a = py.get_type::<PyClassA>();
        let py_class_b = py.get_type::<PyClassB>();
        py_run!(
            py,
            py_class_a py_class_b,
            r#"
a = py_class_a(py_class_b(1), 1)

assert tuple(reversed(a)) == (1, py_class_b(1))
"#
        );
    });
}
