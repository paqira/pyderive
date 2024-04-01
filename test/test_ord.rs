use pyderive::*;
use pyo3::{prelude::*, py_run};

#[test]
fn test_ord() {
    #[pyclass]
    #[derive(Default, PartialOrd, PartialEq, Eq, Ord, PyOrd)]
    #[allow(dead_code)]
    struct PyClass {
        fa: i64,
        fb: String,
    }

    Python::with_gil(|py| {
        let data1 = Py::new(py, PyClass::default()).unwrap();
        let data2 = Py::new(
            py,
            PyClass {
                fa: 1,
                fb: String::default(),
            },
        )
        .unwrap();
        py_run!(py, data1 data2,  "assert data1 < data2");
        py_run!(py, data1 data2,  "assert data1 <= data2");
        py_run!(py, data1 data2,  "assert data1 <= data1");
        py_run!(py, data1 data2,  "assert data2 > data1");
        py_run!(py, data1 data2,  "assert data2 >= data1");
        py_run!(py, data1 data2,  "assert data1 >= data1");
        py_run!(py, data1 data2,  "try: assert not data1 < 1
except TypeError: pass");
    });
}

#[test]
fn test_partial_ord() {
    #[pyclass]
    #[derive(Default, PartialOrd, PartialEq, PyOrd)]
    #[allow(dead_code)]
    struct PyClass {
        f: f64,
    }

    Python::with_gil(|py| {
        let data1 = Py::new(py, PyClass::default()).unwrap();
        let data2 = Py::new(py, PyClass { f: f64::NAN }).unwrap();
        py_run!(py, data1 data2,  "assert not data1 < data2");
        py_run!(py, data1 data2,  "assert not data1 <= data2");
        py_run!(py, data1 data2,  "assert not data2 <= data2");
        py_run!(py, data1 data2,  "assert not data2 > data1");
        py_run!(py, data1 data2,  "assert not data2 >= data1");
        py_run!(py, data1 data2,  "assert not data2 >= data2");
        py_run!(py, data1 data2,  "try: assert not data2 < 1
except TypeError: pass");
    });
}

#[test]
fn test_nest_pyclass() {
    #[pyclass(get_all)]
    #[derive(PartialOrd, PartialEq, PyOrd)]
    struct PyClassA {
        field: PyClassB,
    }

    #[derive(PyOrd, Clone)]
    #[pyclass(get_all)]
    #[derive(PartialOrd, PartialEq)]
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
        py_run!(
            py,
            py_class_a py_class_b,
            r#"
a = py_class_a(py_class_b(1))
b = py_class_a(py_class_b(2))

assert a < b
assert a <= b
assert not a > b
assert not a >= b
"#
        );
    });
}
