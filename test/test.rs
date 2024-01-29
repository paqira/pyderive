use pyderive::*;
use pyo3::{prelude::*, py_run};

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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert repr(data) == "PyClass(field_name_a=0, field_name_b=0.0)""#
            )
        });
    }

    #[test]
    fn test_get_all() {
        #[derive(PyRepr)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert repr(data) == "PyClass(field_name_a=0, field_name_b=0.0)""#
            )
        });
    }

    #[test]
    fn test_set_all() {
        #[derive(PyRepr)]
        #[pyclass(set_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert repr(data) == "PyClass(field_name_a=0, field_name_b=0.0)""#
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
            field_name_a: i64,
            field_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert repr(data) == "PyClass(new_name=0, fieldNameB=0.0)""#
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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert str(data) == "PyClass(field_name_a=0, field_name_b=0.0)""#
            )
        });
    }

    #[test]
    fn test_get_all() {
        #[derive(PyStr)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert str(data) == "PyClass(field_name_a=0, field_name_b=0.0)""#
            )
        });
    }

    #[test]
    fn test_set_all() {
        #[derive(PyStr)]
        #[pyclass(set_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert str(data) == "PyClass(field_name_a=0, field_name_b=0.0)""#
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
            field_name_a: i64,
            field_name_b: f64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                r#"assert str(data) == "PyClass(new_name=0, fieldNameB=0.0)""#
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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: f64,
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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: f64,
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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            field_name_b: f64,
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
            field_name_a: i64,
            field_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(py, py_class, "py_class(0, '')");
            pyo3::py_run!(py, py_class, "py_class(field_name_b='', field_name_a=0)");
        });
    }

    #[test]
    fn test_get_set() {
        #[derive(PyInit)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(py, py_class, "assert py_class(0, '').field_name_a == 0")
        })
    }

    #[test]
    fn test_name_rename_all() {
        #[derive(PyInit)]
        #[pyclass(get_all, name = "renamedClass", rename_all = "camelCase")]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            field_name_a: i64,
            field_name_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("renamedClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(py, py_class, "assert py_class(0, '').new_name == 0");
            pyo3::py_run!(py, py_class, "assert py_class(0, '').fieldNameB == ''");
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(fieldNameB='', new_name=0).new_name == 0"
            );
            pyo3::py_run!(
                py,
                py_class,
                "assert py_class(fieldNameB='', new_name=0).fieldNameB == ''"
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
            #[pyderive(default = 100)]
            field: i64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(py, py_class, "assert py_class().field == 100");
            pyo3::py_run!(py, py_class, "assert py_class(0).field == 0");
        });
    }

    #[test]
    fn test_pyderive_c() {
        #[derive(PyInit)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            #[pyderive(init = false, default = 100)]
            field: i64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(py, py_class, "assert py_class().field == 100");
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
            field_a: i64,
            #[pyderive(kw_only)]
            field_b: i64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(py, py_class, "assert py_class(0, field_b=1).field_a == 0");
            pyo3::py_run!(py, py_class, "assert py_class(0, field_b=1).field_b == 1");
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
            field_a: i64,
            field_b: i64,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(py, py_class, "assert py_class(field_b=0).field_a == 100");
            pyo3::py_run!(py, py_class, "assert py_class(field_b=0).field_b == 0");
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
            field_name_a: i64,
            field_name_b: String,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(field_name_a: i64, field_name_b: String) -> Self {
                Self {
                    field_name_a,
                    field_name_b,
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
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: String,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(field_name_a: i64, field_name_b: String) -> Self {
                Self {
                    field_name_a,
                    field_name_b,
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
                "assert py_class(0, '').__match_args__ == ('field_name_a', )"
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
            field_name_a: i64,
            field_name_b: String,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(field_name_a: i64, field_name_b: String) -> Self {
                Self {
                    field_name_a,
                    field_name_b,
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
                "assert py_class(0, '').__match_args__ == ('new_name', 'fieldNameB')"
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
match py_class(field_name_b='', field_name_a=0):
    case py_class(new_name=a, fieldNameB=b) if a == 0 and b == '': pass
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
            field_a: i64,
            field_b: i64,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            pub fn new(field_a: i64, field_b: i64) -> Self {
                Self { field_a, field_b }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_class.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_class,
                r#"assert py_class(0, 0).__match_args__ == ("field_b", )"#
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
            field_name_a: i64,
            field_name_b: String,
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
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: String,
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
            field_name_a: i64,
            field_name_b: String,
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
            field_name_a: i64,
            field_name_b: String,
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
            field_name_a: i64,
            field_name_b: String,
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
            field_name_a: i64,
            field_name_b: String,
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
