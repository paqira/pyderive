use pyderive_macros::*;
use pyo3::{prelude::*, types::*};

#[test]
fn test_empty() {
    #[derive(PyNamedTupleFieldDefaults)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        b: String,
        c: Py<PyTuple>,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "assert py_class._field_defaults == {}
"
        );
    });
}

#[test]
fn test_full() {
    #[derive(PyNamedTupleFieldDefaults)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        #[pyderive(default = 1)]
        a: i64,
        #[pyderive(default = "a")]
        b: String,
        #[pyderive(default=(1, 2, 3))]
        c: Py<PyTuple>,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "assert py_class._field_defaults == {'a': 1, 'b': 'a', 'c': (1, 2, 3)}
"
        );
    });
}

#[test]
fn test_partial() {
    #[derive(PyNamedTupleFieldDefaults)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        a: i64,
        #[pyderive(default = "a")]
        b: String,
        #[pyderive(default = (1, 2, 3))]
        c: Py<PyTuple>,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "assert py_class._field_defaults == {'b': 'a', 'c': (1, 2, 3)}
"
        );
    });
}

#[test]
fn test_new_false() {
    #[derive(PyNamedTupleFieldDefaults)]
    #[pyclass(get_all)]
    #[allow(dead_code)]
    struct PyClass {
        #[pyderive(new = false)]
        a: i64,
        #[pyderive(new=false, default="a".to_string())]
        b: String,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "assert py_class._field_defaults == {'a':0, 'b': 'a'}
"
        );
    });
}

#[test]
fn test_rename() {
    #[derive(PyNamedTupleFieldDefaults)]
    #[pyclass(get_all, rename_all = "camelCase")]
    #[allow(dead_code)]
    struct PyClass {
        #[pyo3(name = "renamed_field")]
        #[pyderive(default = 0)]
        a: i64,
        #[pyderive(default="a".to_string())]
        aaa_bbb_ccc: String,
    }

    Python::attach(|py| {
        let py_class = py.get_type::<PyClass>();
        assert_eq!("PyClass", py_class.name().unwrap().to_string());

        pyo3::py_run!(
            py,
            py_class,
            "assert py_class._field_defaults == {'renamed_field':0, 'aaaBbbCcc': 'a'}
"
        );
    });
}
