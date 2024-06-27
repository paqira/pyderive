use pyderive_macros::*;
use pyo3::prelude::*;

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

    Python::with_gil(|py| {
        let py_class = py.get_type_bound::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        if py.version_info() >= (3, 10) {
            pyo3::py_run!(
                py,
                py_class,
                "
match py_class(0, ''):
    case py_class(): pass
    case _: raise AssertionError
"
            );
        }
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

    Python::with_gil(|py| {
        let py_class = py.get_type_bound::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "assert py_class(0, '').__match_args__ == ('fd_name_a', )"
        );

        if py.version_info() >= (3, 10) {
            pyo3::py_run!(
                py,
                py_class,
                "
match py_class(0, ''):
    case py_class(a) if a == 0: pass
    case _: raise AssertionError"
            );
        }
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

    Python::with_gil(|py| {
        let py_class = py.get_type_bound::<PyClass>();
        assert_eq!("renamedClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "assert py_class(0, '').__match_args__ == ('new_name', 'fdNameB')"
        );

        if py.version_info() >= (3, 10) {
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
        }
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

    Python::with_gil(|py| {
        let py_class = py.get_type_bound::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "assert py_class(0).__match_args__ == ('field', )"
        );

        if py.version_info() >= (3, 10) {
            pyo3::py_run!(
                py,
                py_class,
                "
match py_class(0):
    case py_class(a) if a == 1: pass
    case _: raise AssertionError
"
            );
        }
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

    Python::with_gil(|py| {
        let py_class = py.get_type_bound::<PyClass>();
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

    Python::with_gil(|py| {
        let py_class = py.get_type_bound::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            r#"assert py_class(0, 0).__match_args__ == ("fd_b", )"#
        );
    });
}

#[test]
fn test_nest_pyclass() {
    #[derive(PyMatchArgs)]
    #[pyclass(get_all)]
    struct PyClassA {
        field: PyClassB,
    }

    #[derive(PyMatchArgs, Clone)]
    #[pyclass(get_all)]
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
        pyo3::py_run!(
            py,
            py_class_a py_class_b,
            r#"
a = py_class_a(py_class_b(1))

assert a.__match_args__ == ('field', )
"#
        );
    });
}
