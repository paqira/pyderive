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
            py_run!(py, data, "assert repr(data) == \"PyClass()\"")
        });
    }

    #[test]
    fn test_get_set() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyRepr)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                "assert repr(data) == \"PyClass(field_name_a=0, field_name_b=0.0)\""
            )
        });
    }

    #[test]
    fn test_get_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyRepr)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                "assert repr(data) == \"PyClass(field_name_a=0, field_name_b=0.0)\""
            )
        });
    }

    #[test]
    fn test_set_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyRepr)]
        #[pyclass(set_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                "assert repr(data) == \"PyClass(field_name_a=0, field_name_b=0.0)\""
            )
        });
    }

    #[test]
    fn test_name_rename_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyRepr)]
        #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                "assert repr(data) == \"PyClass(new_name=0, fieldNameB=0.0)\""
            )
        });
    }
}

#[cfg(test)]
mod test_str {
    use super::*;

    #[test]
    fn test_no_get_set() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyStr)]
        #[pyclass]
        #[derive(Default)]
        #[allow(dead_code)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert str(data) == \"PyClass()\"")
        });
    }

    #[test]
    fn test_get_set() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyStr)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                "assert str(data) == \"PyClass(field_name_a=0, field_name_b=0.0)\""
            )
        });
    }

    #[test]
    fn test_get_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyStr)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                "assert str(data) == \"PyClass(field_name_a=0, field_name_b=0.0)\""
            )
        });
    }

    #[test]
    fn test_set_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyStr)]
        #[pyclass(set_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                "assert str(data) == \"PyClass(field_name_a=0, field_name_b=0.0)\""
            )
        });
    }

    #[test]
    fn test_name_rename_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyStr)]
        #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(
                py,
                data,
                "assert str(data) == \"PyClass(new_name=0, fieldNameB=0.0)\""
            )
        });
    }
}

#[cfg(test)]
mod test_iter {
    use super::*;

    #[test]
    fn test_no_get_set() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyIter)]
        #[pyclass]
        #[derive(Default)]
        #[allow(dead_code)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert tuple(data) == ()")
        });
    }

    #[test]
    fn test_get_set() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyIter)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert tuple(data) == (0, )")
        });
    }

    #[test]
    fn test_get_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyIter)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert tuple(data) == (0, 0.0)")
        });
    }

    #[test]
    fn test_set_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyIter)]
        #[pyclass(set_all)]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert tuple(data) == (0, )")
        });
    }

    #[test]
    fn test_name_rename_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyIter)]
        #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert tuple(data) == (0, 0.0)")
        });
    }
}

#[cfg(test)]
mod test_len {
    use super::*;

    #[test]
    fn test_no_get_set() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyLen)]
        #[pyclass]
        #[derive(Default)]
        #[allow(dead_code)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert len(data) == 0")
        });
    }

    #[test]
    fn test_get_set() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyLen)]
        #[pyclass]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert len(data) == 1")
        });
    }

    #[test]
    fn test_get_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyLen)]
        #[pyclass(get_all)]
        #[derive(Default)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert len(data) == 2")
        });
    }

    #[test]
    fn test_set_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyLen)]
        #[pyclass(set_all)]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(get)]
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert len(data) == 1")
        });
    }

    #[test]
    fn test_name_rename_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyLen)]
        #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
        #[derive(Default)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            field_name_a: i64,
            field_name_b: f64,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert len(data) == 2")
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
            let py_c = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_c.name().unwrap().to_string());
            pyo3::py_run!(py, py_c, "py_c(0, '')");
            pyo3::py_run!(py, py_c, "py_c(field_name_b='', field_name_a=0)");
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
            let py_c = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_c.name().unwrap().to_string());
            pyo3::py_run!(py, py_c, "assert py_c(0, '').field_name_a == 0")
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
            let py_c = py.get_type::<PyClass>();
            assert_eq!("renamedClass", py_c.name().unwrap().to_string());
            pyo3::py_run!(py, py_c, "assert py_c(0, '').new_name == 0");
            pyo3::py_run!(py, py_c, "assert py_c(0, '').fieldNameB == ''");
            pyo3::py_run!(
                py,
                py_c,
                "assert py_c(fieldNameB='', new_name=0).new_name == 0"
            );
            pyo3::py_run!(
                py,
                py_c,
                "assert py_c(fieldNameB='', new_name=0).fieldNameB == ''"
            );
        });
    }
}

#[cfg(test)]
mod test_hash {
    use super::*;

    #[test]
    fn test() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyHash)]
        #[pyclass]
        #[derive(Default, Hash)]
        #[allow(dead_code)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: String,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert hash(data) == -9000812902462168605")
        });
    }

    #[test]
    fn test_get_set() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyHash)]
        #[pyclass]
        #[derive(Default, Hash)]
        struct PyClass {
            #[pyo3(get)]
            field_name_a: i64,
            #[pyo3(set)]
            field_name_b: String,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert hash(data) == -9000812902462168605")
        });
    }

    #[test]
    fn test_get_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyHash)]
        #[pyclass(get_all)]
        #[derive(Default, Hash)]
        struct PyClass {
            field_name_a: i64,
            field_name_b: String,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert hash(data) == -9000812902462168605")
        });
    }

    #[test]
    fn test_set_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyHash)]
        #[pyclass(set_all)]
        #[derive(Default, Hash)]
        struct PyClass {
            #[pyo3(get)]
            field_name_a: i64,
            field_name_b: String,
        }

        Python::with_gil(|py| {
            let data = PyCell::new(py, PyClass::default()).unwrap();
            py_run!(py, data, "assert hash(data) == -9000812902462168605")
        });
    }

    #[test]
    fn test_name_rename_all() {
        pyo3::prepare_freethreaded_python();

        #[derive(PyHash)]
        #[pyclass(get_all, name = "PyClass", rename_all = "camelCase")]
        #[derive(Default, Hash)]
        struct PyClass {
            #[pyo3(name = "new_name")]
            field_name_a: i64,
            field_name_b: String,
        }

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
            let py_c = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_c.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_c,
                "
match py_c(0, ''):
    case py_c(): pass
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
            let py_c = py.get_type::<PyClass>();
            assert_eq!("PyClass", py_c.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_c,
                "assert py_c(0, '').__match_args__ == ('field_name_a', )"
            );
            pyo3::py_run!(
                py,
                py_c,
                "
match py_c(0, ''):
    case py_c(a) if a == 0: pass
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
            let py_c = py.get_type::<PyClass>();
            assert_eq!("renamedClass", py_c.name().unwrap().to_string());
            pyo3::py_run!(
                py,
                py_c,
                "assert py_c(0, '').__match_args__ == ('new_name', 'fieldNameB')"
            );

            pyo3::py_run!(
                py,
                py_c,
                "
match py_c(0, ''):
    case py_c(a, b) if a == 0 and b == '': pass
    case _: raise AssertionError"
            );

            pyo3::py_run!(
                py,
                py_c,
                "
match py_c(field_name_b='', field_name_a=0):
    case py_c(new_name=a, fieldNameB=b) if a == 0 and b == '': pass
    case _: raise AssertionError"
            );
        });
    }
}
