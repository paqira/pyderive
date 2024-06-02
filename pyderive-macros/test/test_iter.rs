use pyderive_macros::*;
use pyo3::{prelude::*, py_run};

#[test]
fn test_no_get_set() {
    #[derive(PyIter)]
    #[pyclass]
    #[derive(Default)]
    #[allow(dead_code)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(data) == ()")
    });
}

#[test]
fn test_get_set() {
    #[derive(PyIter)]
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
        py_run!(py, data, "assert tuple(data) == (0, )")
    });
}

#[test]
fn test_get_all() {
    #[derive(PyIter)]
    #[pyclass(get_all)]
    #[derive(Default)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(data) == (0, 0.0)")
    });
}

#[test]
fn test_set_all() {
    #[derive(PyIter)]
    #[pyclass(set_all)]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(get)]
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(data) == (0, )")
    });
}

#[test]
fn test_name_rename_all() {
    #[derive(PyIter)]
    #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(name = "new_name")]
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(data) == (0, 0.0)")
    });
}

#[test]
fn test_pyderive_true() {
    #[derive(PyIter, Default)]
    #[pyclass]
    struct PyClass {
        #[pyderive(iter)]
        field: i64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(data) == (0, )")
    });
}

#[test]
fn test_pyderive_false() {
    #[derive(PyIter, Default)]
    #[pyclass(get_all)]
    struct PyClass {
        #[pyderive(iter = false)]
        field: i64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert tuple(data) == tuple()")
    });
}

#[test]
fn test_nest_pyclass() {
    #[derive(PyIter)]
    #[pyclass(get_all)]
    struct PyClassA {
        field_1: PyClassB,
        field_2: i64,
    }

    #[derive(PyIter, Clone)]
    #[pyclass(get_all)]
    #[derive(PartialEq)]
    struct PyClassB {
        field: i64,
    }

    #[pymethods]
    impl PyClassA {
        #[new]
        #[pyo3(signature=(field_1, field_2))]
        fn new(field_1: PyClassB, field_2: i64) -> Self {
            Self { field_1, field_2 }
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

    impl ToPyObject for PyClassB {
        fn to_object(&self, py: Python<'_>) -> PyObject {
            self.clone().into_py(py)
        }
    }

    Python::with_gil(|py| {
        let py_class_a = py.get_type_bound::<PyClassA>();
        let py_class_b = py.get_type_bound::<PyClassB>();
        py_run!(
            py,
            py_class_a py_class_b,
            r#"
a = py_class_a(py_class_b(1), 1)

assert tuple(a) == (py_class_b(1), 1)
"#
        );
    });
}
