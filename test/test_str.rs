use pyderive::*;
use pyo3::{prelude::*, py_run, types::*};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[test]
fn test_variation() {
    #[derive(PyStr)]
    #[pyclass(get_all)]
    struct PyClass {
        fd_name_bool: bool,
        fd_name_str: String,
        fd_name_int: i64,
        fd_name_float: f64,
        fn_name_bytes: Vec<u8>,

        fd_name_opt_str: Option<String>,
        fd_name_opt_int: Option<i64>,

        fd_name_vec_str: Vec<String>,
        fd_name_vec_int: Vec<i64>,

        fd_name_vec_opt_str: Vec<Option<String>>,
        fd_name_vec_opt_int: Vec<Option<i64>>,

        fd_name_hs_str: HashSet<String>,
        fd_name_js_int: BTreeSet<i64>,

        fd_name_hm_str: HashMap<String, String>,
        fd_name_hm_int: BTreeMap<i64, i64>,

        fd_name_pybool: Py<PyBool>,
        fd_name_pystr: Py<PyString>,
        fd_name_pyint: Py<PyLong>,
        fd_name_pyfloat: Py<PyFloat>,

        fd_name_opt_pystr: Option<Py<PyString>>,
        fd_name_opt_pyint: Option<Py<PyLong>>,

        fd_name_vec_pystr: Vec<Py<PyString>>,
        fd_name_vec_pyint: Vec<Py<PyLong>>,

        fd_name_vec_opt_pystr: Vec<Option<Py<PyString>>>,
        fd_name_vec_opt_pyint: Vec<Option<Py<PyLong>>>,

        fd_name_pystr_abspath: ::pyo3::Py<PyString>,
    }

    #[pymethods]
    impl PyClass {
        #[new]
        #[pyo3(signature=(
                fd_name_bool,
                fd_name_str,
                fd_name_int,
                fd_name_float,
                fn_name_bytes,

                fd_name_opt_str,
                fd_name_opt_int,

                fd_name_vec_str,
                fd_name_vec_int,

                fd_name_vec_opt_str,
                fd_name_vec_opt_int,

                fd_name_hs_str,
                fd_name_js_int,

                fd_name_hm_str,
                fd_name_hm_int,

                fd_name_pybool,
                fd_name_pystr,
                fd_name_pyint,
                fd_name_pyfloat,

                fd_name_opt_pystr,
                fd_name_opt_pyint,

                fd_name_vec_pystr,
                fd_name_vec_pyint,

                fd_name_vec_opt_pystr,
                fd_name_vec_opt_pyint,

                fd_name_pystr_abspath,
            ))]
        fn new(
            fd_name_bool: bool,
            fd_name_str: String,
            fd_name_int: i64,
            fd_name_float: f64,
            fn_name_bytes: Vec<u8>,

            fd_name_opt_str: Option<String>,
            fd_name_opt_int: Option<i64>,

            fd_name_vec_str: Vec<String>,
            fd_name_vec_int: Vec<i64>,

            fd_name_vec_opt_str: Vec<Option<String>>,
            fd_name_vec_opt_int: Vec<Option<i64>>,

            fd_name_hs_str: HashSet<String>,
            fd_name_js_int: BTreeSet<i64>,

            fd_name_hm_str: HashMap<String, String>,
            fd_name_hm_int: BTreeMap<i64, i64>,

            fd_name_pybool: Py<PyBool>,
            fd_name_pystr: Py<PyString>,
            fd_name_pyint: Py<PyLong>,
            fd_name_pyfloat: Py<PyFloat>,

            fd_name_opt_pystr: Option<Py<PyString>>,
            fd_name_opt_pyint: Option<Py<PyLong>>,

            fd_name_vec_pystr: Vec<Py<PyString>>,
            fd_name_vec_pyint: Vec<Py<PyLong>>,

            fd_name_vec_opt_pystr: Vec<Option<Py<PyString>>>,
            fd_name_vec_opt_pyint: Vec<Option<Py<PyLong>>>,

            fd_name_pystr_abspath: ::pyo3::Py<PyString>,
        ) -> Self {
            Self {
                fd_name_bool,
                fd_name_str,
                fd_name_int,
                fd_name_float,
                fn_name_bytes,
                fd_name_opt_str,
                fd_name_opt_int,
                fd_name_vec_str,
                fd_name_vec_int,
                fd_name_vec_opt_str,
                fd_name_vec_opt_int,
                fd_name_hs_str,
                fd_name_js_int,
                fd_name_hm_str,
                fd_name_hm_int,
                fd_name_pybool,
                fd_name_pystr,
                fd_name_pyint,
                fd_name_pyfloat,
                fd_name_opt_pystr,
                fd_name_opt_pyint,
                fd_name_vec_pystr,
                fd_name_vec_pyint,
                fd_name_vec_opt_pystr,
                fd_name_vec_opt_pyint,
                fd_name_pystr_abspath,
            }
        }
    }

    Python::with_gil(|py| {
        let py_class = py.get_type_bound::<PyClass>();
        assert_eq!("builtins.PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            r#"a = py_class(
    True, 'str', 1, 1.0, b'str',
    'str', 1,
    ['str'], [1],
    ['str'], [1],
    {'str'}, {1},
    {'str': 'str'}, {1: 1},
    True, 'str', 1, 1.0,
    'str', 1,
    ['str'], [1],
    ['str'], [1],
    'str',
)

assert str(a) == "builtins.PyClass(fd_name_bool=True, fd_name_str='str', fd_name_int=1, fd_name_float=1.0, fn_name_bytes=[115, 116, 114], fd_name_opt_str='str', fd_name_opt_int=1, fd_name_vec_str=['str'], fd_name_vec_int=[1], fd_name_vec_opt_str=['str'], fd_name_vec_opt_int=[1], fd_name_hs_str={'str'}, fd_name_js_int={1}, fd_name_hm_str={'str': 'str'}, fd_name_hm_int={1: 1}, fd_name_pybool=True, fd_name_pystr='str', fd_name_pyint=1, fd_name_pyfloat=1.0, fd_name_opt_pystr='str', fd_name_opt_pyint=1, fd_name_vec_pystr=['str'], fd_name_vec_pyint=[1], fd_name_vec_opt_pystr=['str'], fd_name_vec_opt_pyint=[1], fd_name_pystr_abspath='str')""#
        );
    });
}

#[test]
fn test_nest_pyclass() {
    #[derive(PyStr)]
    #[pyclass(get_all)]
    struct PyClassA {
        field: PyClassB,
    }

    // PyStr calls repr()
    #[derive(PyRepr, Clone)]
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

    impl ToPyObject for PyClassB {
        fn to_object(&self, py: Python<'_>) -> PyObject {
            self.clone().into_py(py)
        }
    }

    Python::with_gil(|py| {
        let py_class_a = py.get_type_bound::<PyClassA>();
        let py_class_b = py.get_type_bound::<PyClassB>();
        assert_eq!("builtins.PyClassA", py_class_a.name().unwrap().to_string());
        assert_eq!("builtins.PyClassB", py_class_b.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class_a py_class_b,
            r#"
a = py_class_a(py_class_b(1))

assert str(a) == "builtins.PyClassA(field=builtins.PyClassB(field=1))"
"#
        );
    });
}

#[test]
fn test_no_get_set() {
    #[derive(PyStr)]
    #[pyclass]
    #[derive(Default)]
    #[allow(dead_code)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, r#"assert str(data) == "builtins.PyClass()""#)
    });
}

#[test]
fn test_get_set() {
    #[derive(PyStr)]
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
        py_run!(
            py,
            data,
            r#"assert str(data) == "builtins.PyClass(fd_name_a=0, fd_name_b=0.0)""#
        )
    });
}

#[test]
fn test_get_all() {
    #[derive(PyStr)]
    #[pyclass(get_all)]
    #[derive(Default)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(
            py,
            data,
            r#"assert str(data) == "builtins.PyClass(fd_name_a=0, fd_name_b=0.0)""#
        )
    });
}

#[test]
fn test_set_all() {
    #[derive(PyStr)]
    #[pyclass(set_all)]
    #[derive(Default)]
    struct PyClass {
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(
            py,
            data,
            r#"assert str(data) == "builtins.PyClass(fd_name_a=0, fd_name_b=0.0)""#
        )
    });
}

#[test]
fn test_name_rename_all() {
    #[derive(PyStr)]
    #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(name = "new_name")]
        fd_name_a: i64,
        fd_name_b: f64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(
            py,
            data,
            r#"assert str(data) == "builtins.PyClass(new_name=0, fdNameB=0.0)""#
        )
    });
}

#[test]
fn test_pyderive_true() {
    #[derive(PyStr)]
    #[pyclass]
    #[derive(Default)]
    struct PyClass {
        #[pyderive(str)]
        field: i64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(
            py,
            data,
            r#"assert str(data) == "builtins.PyClass(field=0)""#
        )
    });
}

#[test]
fn test_pyderive_false() {
    #[derive(PyStr)]
    #[pyclass]
    #[derive(Default)]
    struct PyClass {
        #[pyo3(get)]
        #[pyderive(str = false)]
        field: i64,
    }

    Python::with_gil(|py| {
        let data = Py::new(py, PyClass::default()).unwrap();
        py_run!(py, data, r#"assert str(data) == "builtins.PyClass()""#)
    });
}
