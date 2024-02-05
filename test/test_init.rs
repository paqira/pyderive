use pyderive::*;
use pyo3::{prelude::*, types::*};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[cfg(test)]
mod test {
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

    #[test]
    fn test_nest_pyclass() {
        #[derive(PyInit)]
        #[pyclass(get_all)]
        struct PyClassA {
            field: PyClassB,
        }

        #[derive(PyInit, Clone)]
        #[pyclass(get_all)]
        struct PyClassB {
            field: i64,
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

assert isinstance(a.field, py_class_b)
assert a.field.field == 1
"#
            );
        });
    }
}
