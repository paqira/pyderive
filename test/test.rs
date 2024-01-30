use pyderive::*;
use pyo3::{prelude::*, py_run, types::*};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[cfg(test)]
mod test_repr {
    use super::*;

    #[test]
    fn test_no_get_set() {
        #[derive(PyRepr)]
        #[pyclass]
        #[derive(Default)]
        #[allow(dead_code)]
        struct PyClass {
            fd_name_a: i64,
            fd_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, r#"assert repr(data) == "PyClass()""#)
        });
    }

    #[test]
    fn test_get_set() {
        #[derive(PyRepr)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            fd_name_a: i64,
            #[pyo3(set)]
            fd_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert repr(data) == "PyClass(fd_name_a=0, fd_name_b=0.0)""#
            )
        });
    }

    #[test]
    fn test_get_all() {
        #[derive(PyRepr)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            fd_name_a: i64,
            fd_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert repr(data) == "PyClass(fd_name_a=0, fd_name_b=0.0)""#
            )
        });
    }

    #[test]
    fn test_set_all() {
        #[derive(PyRepr)]
        #[pyclass(set_all)]
        #[derive(Default)]
        struct PyClass {
            fd_name_a: i64,
            fd_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert repr(data) == "PyClass(fd_name_a=0, fd_name_b=0.0)""#
            )
        });
    }

    #[test]
    fn test_name_rename_all() {
        #[derive(PyRepr)]
        #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            fd_name_a: i64,
            fd_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert repr(data) == "PyClass(new_name=0, fdNameB=0.0)""#
            )
        });
    }

    #[test]
    fn test_pyderive_true() {
        #[derive(PyRepr)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyderive(repr)]
            field: i64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, r#"assert repr(data) == "PyClass(field=0)""#)
        });
    }

    #[test]
    fn test_pyderive_false() {
        #[derive(PyRepr)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            #[pyderive(repr = false)]
            field: i64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, r#"assert repr(data) == "PyClass()""#)
        });
    }
}

#[cfg(test)]
mod test_str {
    use super::*;

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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, r#"assert str(data) == "PyClass()""#)
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert str(data) == "PyClass(fd_name_a=0, fd_name_b=0.0)""#
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert str(data) == "PyClass(fd_name_a=0, fd_name_b=0.0)""#
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert str(data) == "PyClass(fd_name_a=0, fd_name_b=0.0)""#
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert str(data) == "PyClass(new_name=0, fdNameB=0.0)""#
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, r#"assert str(data) == "PyClass(field=0)""#)
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, r#"assert str(data) == "PyClass()""#)
        });
    }
}

#[cfg(test)]
mod test_iter {
    use super::*;

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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert tuple(data) == tuple()")
        });
    }
}

#[cfg(test)]
mod test_len {
    use super::*;

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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
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

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert len(data) == 0")
        });
    }
}

#[cfg(test)]
mod test_init {
    use super::*;

    #[test]
    fn test_no_get_set() {
        #[derive(PyInit)]
        #[pyclass]
        #[allow(dead_code)]
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
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_class,
                "py_class(
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
                )"
            );
            pyo3::py_run!(
                py,
                py_class,
                "py_class(
fd_name_bool=True, fd_name_str='str', fd_name_int=1, fd_name_float=1.0, fn_name_bytes=b'str',
fd_name_opt_str=None, fd_name_opt_int=None,
fd_name_vec_str=[], fd_name_vec_int=[],
fd_name_vec_opt_str=[None], fd_name_vec_opt_int=[None],
fd_name_hs_str=set(), fd_name_js_int=set(),
fd_name_hm_str={}, fd_name_hm_int={},
fd_name_pybool=True, fd_name_pystr='str', fd_name_pyint=1, fd_name_pyfloat=1.0,
fd_name_opt_pystr=None, fd_name_opt_pyint=None,
fd_name_vec_pystr=[], fd_name_vec_pyint=[],
fd_name_vec_opt_pystr=[None], fd_name_vec_opt_pyint=[None],
            )"
            );
        });
    }

    #[test]
    fn test_get_set() {
        #[derive(PyInit)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            fd_name_a: i64,
            #[pyo3(set)]
            fd_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(py, py_class, "assert py_class(0, '').fd_name_a == 0")
        })
    }

    #[test]
    fn test_name_rename_all() {
        #[derive(PyInit)]
        #[pyclass(get_all, name = "renamedClass", rename_all = "camelCase")]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            fd_name_a: i64,
            fd_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("renamedClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(py, py_class, "assert py_class(0, '').new_name == 0");
            pyo3::py_run!(py, py_class, "assert py_class(0, '').fdNameB == ''");
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(fdNameB='', new_name=0).new_name == 0"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(fdNameB='', new_name=0).fdNameB == ''"
            );
        });
    }

    #[test]
    fn test_pyderive_a() {
        #[derive(PyInit)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            #[pyderive(init = false)]
            field: i64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(py, py_class, "assert py_class().field == 0");
            pyo3::py_run!(
                py,
                py_class,
                "try: py_class(0)
except TypeError: pass"
            );
        });
    }

    #[test]
    fn test_pyderive_b() {
        #[derive(PyInit)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            #[pyderive(default = true)]
            fd_name_bool: bool,
            #[pyderive(default = "str".to_string())]
            fd_name_str: String,
            #[pyderive(default = 1)]
            fd_name_int: i64,
            #[pyderive(default = 1.0)]
            fd_name_float: f64,
            #[pyderive(default = None)]
            fd_name_opt: Option<i64>,
            #[pyderive(default = "str".to_string())]
            fd_name_opt_str: Option<String>,
            #[pyderive(default = 10)]
            fd_name_opt_int: Option<i64>,
            #[pyderive(default = None)]
            fd_name_opt_pystr: Option<Py<PyString>>,
            #[pyderive(default = None)]
            fd_name_opt_pyint: Option<Py<PyLong>>,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_bool is True");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_str == 'str'");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_int == 1");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_float == 1.0");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_opt is None");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_opt_str == 'str'");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_opt_int == 10");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_opt_pystr is None");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_opt_pyint is None");

            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(
                fd_name_bool=False,
                fd_name_str='',
                fd_name_int=0,
                fd_name_float=0.0,
                fd_name_opt=1,
                fd_name_opt_str='',
                fd_name_opt_int=0,
                fd_name_opt_pystr='',
                fd_name_opt_pyint=0,
            ).fd_name_bool is False"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(
                fd_name_bool=False,
                fd_name_str='',
                fd_name_int=0,
                fd_name_float=0.0,
                fd_name_opt=1,
                fd_name_opt_str='',
                fd_name_opt_int=0,
                fd_name_opt_pystr='',
                fd_name_opt_pyint=0,
            ).fd_name_str == ''"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(
                fd_name_bool=False,
                fd_name_str='',
                fd_name_int=0,
                fd_name_float=0.0,
                fd_name_opt=1,
                fd_name_opt_str='',
                fd_name_opt_int=0,
                fd_name_opt_pystr='',
                fd_name_opt_pyint=0,
            ).fd_name_int == 0"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(
                fd_name_bool=False,
                fd_name_str='',
                fd_name_int=0,
                fd_name_float=0.0,
                fd_name_opt=1,
                fd_name_opt_str='',
                fd_name_opt_int=0,
                fd_name_opt_pystr='',
                fd_name_opt_pyint=0,
            ).fd_name_float == 0.0"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(
                fd_name_bool=False,
                fd_name_str='',
                fd_name_int=0,
                fd_name_float=0.0,
                fd_name_opt=1,
                fd_name_opt_str='',
                fd_name_opt_int=0,
                fd_name_opt_pystr='',
                fd_name_opt_pyint=0,
            ).fd_name_opt == 1"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(
                fd_name_bool=False,
                fd_name_str='',
                fd_name_int=0,
                fd_name_float=0.0,
                fd_name_opt=1,
                fd_name_opt_str='',
                fd_name_opt_int=0,
                fd_name_opt_pystr='',
                fd_name_opt_pyint=0,
            ).fd_name_opt_str == ''"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(
                fd_name_bool=False,
                fd_name_str='',
                fd_name_int=0,
                fd_name_float=0.0,
                fd_name_opt=1,
                fd_name_opt_str='',
                fd_name_opt_int=0,
                fd_name_opt_pystr='',
                fd_name_opt_pyint=0,
            ).fd_name_opt_int == 0"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(
                fd_name_bool=False,
                fd_name_str='',
                fd_name_int=0,
                fd_name_float=0.0,
                fd_name_opt=1,
                fd_name_opt_str='',
                fd_name_opt_int=0,
                fd_name_opt_pystr='',
                fd_name_opt_pyint=0,
            ).fd_name_opt_pyint == 0"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(
                fd_name_bool=False,
                fd_name_str='',
                fd_name_int=0,
                fd_name_float=0.0,
                fd_name_opt=1,
                fd_name_opt_str='',
                fd_name_opt_int=0,
                fd_name_opt_pystr='',
                fd_name_opt_pyint=0
            ).fd_name_opt_pystr == ''"
            );
        });
    }

    #[test]
    fn test_pyderive_c() {
        #[derive(PyInit)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            #[pyderive(init = false, default = true)]
            fd_name_bool: bool,
            #[pyderive(init = false, default = "str".to_string())]
            fd_name_str: String,
            #[pyderive(init = false, default = 1)]
            fd_name_int: i64,
            #[pyderive(init = false, default = 1.0)]
            fd_name_float: f64,
            #[pyderive(init = false, default = None)]
            fd_name_opt: Option<i64>,
            #[pyderive(init = false, default = Some("str".to_string()))]
            fd_name_opt_str: Option<String>,
            #[pyderive(init = false, default = Some(10))]
            fd_name_opt_int: Option<i64>,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_bool is True");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_str == 'str'");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_int == 1");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_float == 1.0");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_opt is None");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_opt_str == 'str'");
            pyo3::py_run!(py, py_class, "assert py_class().fd_name_opt_int == 10");

            pyo3::py_run!(
                py,
                py_class,
                "try: py_class(0)
except TypeError: pass"
            );
        });
    }

    #[test]
    fn test_pyderive_kw_only() {
        #[derive(PyInit)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            fd_a: i64,
            #[pyderive(kw_only)]
            fd_b: i64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(py, py_class, "assert py_class(0, fd_b=1).fd_a == 0");
            pyo3::py_run!(py, py_class, "assert py_class(0, fd_b=1).fd_b == 1");

            pyo3::py_run!(
                py,
                py_class,
                "try: py_class(0, 1)
except TypeError: pass"
            );
        });
    }

    #[test]
    fn test_pyderive_kw_only_default() {
        #[derive(PyInit)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            #[pyderive(kw_only, default = 100)]
            fd_a: i64,
            fd_b: i64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(py, py_class, "assert py_class(fd_b=0).fd_a == 100");
            pyo3::py_run!(py, py_class, "assert py_class(fd_b=0).fd_b == 0");
        });
    }
}

#[cfg(test)]
mod test_match_args {
    use super::*;

    #[test]
    fn test_no_get_set() {
        #[derive(PyMatchArgs)]
        #[pyclass]
        #[allow(dead_code)]
        struct PyClass {
            fd_name_a: i64,
            fd_name_b: String,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(fd_name_a: i64, fd_name_b: String) -> Self {
                Self {
                    fd_name_a,
                    fd_name_b,
                }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_class,
                "
match py_class(0, ''):
    case py_class(): pass
    case _: raise AssertionError
"
            );
        })
    }

    #[test]
    fn test_get_set() {
        #[derive(PyMatchArgs)]
        #[pyclass]
        struct PyClass {
            #[pyo3(get)]
            fd_name_a: i64,
            #[pyo3(set)]
            fd_name_b: String,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(fd_name_a: i64, fd_name_b: String) -> Self {
                Self {
                    fd_name_a,
                    fd_name_b,
                }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(0, '').__match_args__ == ('fd_name_a', )"
            );
            pyo3::py_run!(
                py,
                py_class,
                "
match py_class(0, ''):
    case py_class(a) if a == 0: pass
    case _: raise AssertionError"
            );
        })
    }

    #[test]
    fn test_name_rename_all() {
        #[derive(PyMatchArgs)]
        #[pyclass(get_all, name = "renamedClass", rename_all = "camelCase")]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            fd_name_a: i64,
            fd_name_b: String,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(fd_name_a: i64, fd_name_b: String) -> Self {
                Self {
                    fd_name_a,
                    fd_name_b,
                }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("renamedClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(0, '').__match_args__ == ('new_name', 'fdNameB')"
            );

            pyo3::py_run!(
                py,
                py_class,
                "
match py_class(0, ''):
    case py_class(a, b) if a == 0 and b == '': pass
    case _: raise AssertionError"
            );

            pyo3::py_run!(
                py,
                py_class,
                "
match py_class(fd_name_b='', fd_name_a=0):
    case py_class(new_name=a, fdNameB=b) if a == 0 and b == '': pass
    case _: raise AssertionError"
            );
        });
    }

    #[test]
    fn test_pyderive_true() {
        #[derive(PyMatchArgs)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyderive(match_args)]
            #[allow(dead_code)]
            field: i64,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(field: i64) -> Self {
                Self { field }
            }
            #[getter]
            pub fn field(&self) -> i64 {
                1
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(0).__match_args__ == ('field', )"
            );

            pyo3::py_run!(
                py,
                py_class,
                "
match py_class(0):
    case py_class(a) if a == 1: pass
    case _: raise AssertionError
"
            );
        });
    }

    #[test]
    fn test_pyderive_false_empty() {
        #[derive(PyMatchArgs)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            #[pyderive(match_args = false)]
            field: i64,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(field: i64) -> Self {
                Self { field }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_class,
                r#"assert not hasattr(py_class(0), "__match_args__")"#
            );
        });
    }

    #[test]
    fn test_pyderive_false() {
        #[derive(PyMatchArgs)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            #[pyderive(match_args = false)]
            fd_a: i64,
            fd_b: i64,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(fd_a: i64, fd_b: i64) -> Self {
                Self { fd_a, fd_b }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_class,
                r#"assert py_class(0, 0).__match_args__ == ("fd_b", )"#
            );
        });
    }
}

#[cfg(test)]
mod test_hash {
    use super::*;

    #[test]
    fn test() {
        #[derive(PyHash)]
        #[pyclass]
        #[derive(Default, Hash)]
        #[allow(dead_code)]
        struct PyClass {
            fd_name_a: i64,
            fd_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert hash(data) == -9000812902462168605")
        });
    }

    #[test]
    fn test_get_set() {
        #[derive(PyHash)]
        #[pyclass]
        #[derive(Default, Hash)]
        struct PyClass {
            #[pyo3(get)]
            fd_name_a: i64,
            #[pyo3(set)]
            fd_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert hash(data) == -9000812902462168605")
        });
    }

    #[test]
    fn test_get_all() {
        #[derive(PyHash)]
        #[pyclass(get_all)]
        #[derive(Default, Hash)]
        struct PyClass {
            fd_name_a: i64,
            fd_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert hash(data) == -9000812902462168605")
        });
    }

    #[test]
    fn test_set_all() {
        #[derive(PyHash)]
        #[pyclass(set_all)]
        #[derive(Default, Hash)]
        struct PyClass {
            #[pyo3(get)]
            fd_name_a: i64,
            fd_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert hash(data) == -9000812902462168605")
        });
    }

    #[test]
    fn test_name_rename_all() {
        #[derive(PyHash)]
        #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
        #[derive(Default, Hash)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            fd_name_a: i64,
            fd_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert hash(data) == -9000812902462168605")
        });
    }
}

#[cfg(test)]
mod test_eq {
    use super::*;

    #[test]
    fn test_eq() {
        #[derive(PyEq)]
        #[pyclass]
        #[derive(Default, PartialEq, Eq)]
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
        #[derive(PyEq)]
        #[pyclass]
        #[derive(Default, PartialEq)]
        #[allow(dead_code)]
        struct PyClass {
            f: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data1 = PyCell::new(py, PyClass::default()).unwrap();
            let data2 = PyCell::new(py, PyClass { f: f64::NAN }).unwrap();
            py_run!(py, data1 data2,  "assert data1 != data2");
            py_run!(py, data1 data2,  "assert not data2 == data2");
            py_run!(py, data1 data2,  "assert data2 != data2");
            py_run!(py, data1 data2,  "try: assert not data2 < 1
except TypeError: pass");
        });
    }
}

#[cfg(test)]
mod test_order {
    use super::*;

    #[test]
    fn test_ord() {
        #[derive(PyOrder)]
        #[pyclass]
        #[derive(Default, PartialOrd, PartialEq, Eq, Ord)]
        #[allow(dead_code)]
        struct PyClass {
            fa: i64,
            fb: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data1 = PyCell::new(py, PyClass::default()).unwrap();
            let data2 = PyCell::new(
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
    fn test_patial_ord() {
        #[derive(PyOrder)]
        #[pyclass]
        #[derive(Default, PartialOrd, PartialEq)]
        #[allow(dead_code)]
        struct PyClass {
            f: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data1 = PyCell::new(py, PyClass::default()).unwrap();
            let data2 = PyCell::new(py, PyClass { f: f64::NAN }).unwrap();
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
}
