use pyderive::*;
use pyo3::{prelude::*, py_run};

#[test]
fn test_eq() {
    #[pyclass]
    #[derive(Default, PartialEq, Eq, PyEq)]
    #[allow(dead_code)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: String,
    }

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let data = PyCell::new(py, PyClass::default()).unwrap();
        py_run!(py, data, "assert data == data");
        py_run!(py, data, "assert data != 1");
    });
}

#[test]
fn test_patial_ord() {
    #[pyclass]
    #[derive(Default, PartialEq, PyEq)]
    #[allow(dead_code)]
    struct PyClass {
        f: f64,
    }

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let data1 = PyCell::new(py, PyClass::default()).unwrap();
        let data2 = PyCell::new(py, PyClass::default()).unwrap();
        let data3 = PyCell::new(py, PyClass { f: f64::NAN }).unwrap();
        py_run!(py, data1 data2 data3, "assert data1 == data2");
        py_run!(py, data1 data2 data3, "assert data1 != data3");
        py_run!(py, data1 data2 data3, "assert not data3 == data3");
        py_run!(py, data1 data2 data3, "assert data3 != data3");
        py_run!(py, data1 data2 data3, "try: assert not data3 < 1
except TypeError: pass");
    });
}

#[test]
fn test_nest_pyclass() {
    #[pyclass(get_all)]
    #[derive(PartialEq, PyEq)]
    struct PyClassA {
        field: PyClassB,
    }

    #[pyclass(get_all)]
    #[derive(PartialEq, PyEq, Clone)]
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

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let py_class_a = py.get_type::<PyClassA>();
        let py_class_b = py.get_type::<PyClassB>();
        pyo3::py_run!(
            py,
            py_class_a py_class_b,
            r#"
a = py_class_a(py_class_b(1))

assert a == a
assert a == py_class_a(py_class_b(1))
assert a != py_class_a(py_class_b(2))
"#
        );
    });
}
