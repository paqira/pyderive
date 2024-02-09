use pyderive::*;
use pyo3::{prelude::*, types::*};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rename() {
        #[derive(PyDataclassFields)]
        #[pyclass(get_all, name = "new_name", rename_all = "camelCase")]
        struct PyClass {
            #[pyo3(name = "renamed_field")]
            field_a: i64,
            field_the_name: String,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            fn new(field_a: i64, field_the_name: String) -> Self {
                Self {
                    field_a,
                    field_the_name,
                }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
from dataclasses import fields

for field in fields(py_class(0, "")):
    if field.name == "renamed_field":
        pass
    elif field.name == "fieldTheName":
        pass
    else:
        raise AssertionError
"#
            );
        });
    }

    #[test]
    fn test_on_class() {
        #[derive(PyDataclassFields)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            fn new(field: i64) -> Self {
                Self { field }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
from dataclasses import is_dataclass, fields, MISSING, _FIELD
import sys

assert is_dataclass(py_class) is True
for field in fields(py_class):
    if field.name == "field":
        assert field.type is None
        assert field.default is MISSING
        assert field.default_factory is MISSING
        assert field.init is True
        assert field.repr is True
        assert field.hash is None
        assert field.compare is None
        assert field.metadata == {}
        assert field._field_type is _FIELD
        if sys.version_info >= (3, 10):
            assert field.kw_only is False
    else:
        raise AssertionError
"#
            );
        });
    }

    #[test]
    fn test_variation() {
        #[derive(PyDataclassFields)]
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
                "a = py_class(
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
)

from dataclasses import is_dataclass, asdict, astuple

assert is_dataclass(a) is True
assert asdict(a) == {'fd_name_bool': True, 'fd_name_str': 'str', 'fd_name_int': 1, 'fd_name_float': 1.0, 'fn_name_bytes': [115, 116, 114], 'fd_name_opt_str': 'str', 'fd_name_opt_int': 1, 'fd_name_vec_str': ['str'], 'fd_name_vec_int': [1], 'fd_name_vec_opt_str': ['str'], 'fd_name_vec_opt_int': [1], 'fd_name_hs_str': {'str'}, 'fd_name_js_int': {1}, 'fd_name_hm_str': {'str': 'str'}, 'fd_name_hm_int': {1: 1}, 'fd_name_pybool': True, 'fd_name_pystr': 'str', 'fd_name_pyint': 1, 'fd_name_pyfloat': 1.0, 'fd_name_opt_pystr': 'str', 'fd_name_opt_pyint': 1, 'fd_name_vec_pystr': ['str'], 'fd_name_vec_pyint': [1], 'fd_name_vec_opt_pystr': ['str'], 'fd_name_vec_opt_pyint': [1]}
astuple(a) == (True, 'str', 1, 1.0, [115, 116, 114], 'str', 1, ['str'], [1], ['str'], [1], {'str'}, {1}, {'str': 'str'}, {1: 1}, True, 'str', 1, 1.0, 'str', 1, ['str'], [1], ['str'], [1])
"
            );
        });
    }

    #[test]
    fn test_options() {
        #[derive(PyDataclassFields)]
        #[pyclass(get_all)]
        struct PyClass {
            field: i64,
            #[pyderive(new = false)]
            class_: i64,
            #[pyderive(default = 1)]
            new_defualt: i64,
            #[pyderive(new = false, default = 1)]
            class_defualt: i64,
            #[pyderive(default = 1, default_factory = true)]
            new_defualt_factory: i64,
            #[pyderive(new = false, default = 1, default_factory = true)]
            class_defualt_factory: i64,
            #[pyderive(dataclass_field = false)]
            ommit: i64,
            #[pyderive(repr = true)]
            repr: i64,
            #[pyderive(repr = false)]
            no_repr: i64,
            #[pyderive(kw_only = true)]
            kw_only: i64,
            follow_kw_only: i64,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            fn new(
                field: i64,
                class_: i64,
                new_defualt: i64,
                class_defualt: i64,
                new_defualt_factory: i64,
                class_defualt_factory: i64,
                ommit: i64,
                repr: i64,
                no_repr: i64,
                kw_only: i64,
                follow_kw_only: i64,
            ) -> Self {
                Self {
                    field,
                    class_,
                    new_defualt,
                    class_defualt,
                    new_defualt_factory,
                    class_defualt_factory,
                    ommit,
                    repr,
                    no_repr,
                    kw_only,
                    follow_kw_only,
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
                r#"
import sys
from dataclasses import _FIELD, _FIELD_CLASSVAR, MISSING, fields

a = py_class(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)

for field in fields(a):
    if field.name == "field":
        assert field.type is None
        assert field._field_type is _FIELD, field.name
        if sys.version_info >= (3, 10):
            assert field.kw_only is False, field.name
    elif field.name == "class_":
        assert field.type is None
        assert field._field_type is _FIELD_CLASSVAR, field.name
        if sys.version_info >= (3, 10):
            assert field.kw_only is False, field.name
    elif field.name == "new_defualt":
        assert field.type is None
        assert field.default == 1, field.name
        assert field.default_factory is MISSING, field.name
        assert field._field_type is _FIELD, field.name
        if sys.version_info >= (3, 10):
            assert field.kw_only is False, field.name
    elif field.name == "class_defualt":
        assert field.type is None
        assert field.default == 1, field.name
        assert field.default_factory is MISSING, field.name
        assert field._field_type is _FIELD_CLASSVAR, field.name
        if sys.version_info >= (3, 10):
            assert field.kw_only is False, field.name
    elif field.name == "new_defualt_factory":
        assert field.type is None
        assert field.default is MISSING, field.name
        assert field.default_factory() == 1, field.name
        assert field._field_type is _FIELD, field.name
        if sys.version_info >= (3, 10):
            assert field.kw_only is False, field.name
    elif field.name == "class_defualt_factory":
        assert field.type is None
        assert field.default is MISSING, field.name
        assert field.default_factory() == 1, field.name
        assert field._field_type is _FIELD_CLASSVAR, field.name
        if sys.version_info >= (3, 10):
            assert field.kw_only is False, field.name
    elif field.name == "repr":
        assert field.repr is True, field.name
        if sys.version_info >= (3, 10):
            assert field.kw_only is False, field.name
    elif field.name == "no_repr":
        assert field.repr is False, field.name
        if sys.version_info >= (3, 10):
            assert field.kw_only is False, field.name
    elif field.name == "kw_only":
        if sys.version_info >= (3, 10):
            assert field.kw_only is True, field.name
    elif field.name == "follow_kw_only":
        if sys.version_info >= (3, 10):
            assert field.kw_only is True, field.name
    else:
        raise AssertionError(field.name)
"#
            );
        });
    }

    #[test]
    fn test_nest_pyclass() {
        #[derive(PyDataclassFields)]
        #[pyclass(get_all)]
        struct PyClassA {
            field: PyClassB,
        }

        #[derive(PyDataclassFields, Clone)]
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

        impl ToPyObject for PyClassB {
            fn to_object(&self, py: Python<'_>) -> PyObject {
                self.clone().into_py(py)
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
from dataclasses import is_dataclass, asdict, astuple

a = py_class_a(py_class_b(1))
assert is_dataclass(a) is True
assert asdict(a) == {'field': {'field': 1}}
astuple(a) == ((1, ), )
"#
            );
        });
    }

    #[test]
    fn test_set_name() {
        #[derive(PyDataclassFields)]
        #[pyclass(get_all)]
        struct PyClassA {
            #[pyderive(default = PyClassB{ field: 1 })]
            field: PyClassB,
        }

        #[derive(PyDataclassFields, Clone)]
        #[pyclass(get_all)]
        struct PyClassB {
            field: i64,
        }

        #[pymethods]
        impl PyClassA {
            #[new]
            #[pyo3(signature = ( field=PyClassB{ field: 1 } ))]
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

        impl ToPyObject for PyClassB {
            fn to_object(&self, py: Python<'_>) -> PyObject {
                self.clone().into_py(py)
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
from dataclasses import fields

a = py_class_a().field
b = next(iter(tuple(fields(py_class_a)))).default
assert id(a) != id(b)
"#
            );
        });
    }

    #[test]
    fn test_annotation() {
        #[derive(PyDataclassFields)]
        #[pyclass(get_all)]
        struct PyClass {
            #[pyderive(annotation = "Literal[1]")]
            field: i64,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            fn new(field: i64) -> Self {
                Self { field }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
from dataclasses import fields

for field in fields(py_class(10)):
    if field.name == "field":
        assert field.type == "'Literal[1]'"
"#
            );
        });
    }

    #[test]
    fn test_default_factory() {
        #[derive(PyDataclassFields)]
        #[pyclass(get_all)]
        struct PyClass {
            #[pyderive(
                default=(|| {let r: Vec<i64> = Vec::new();r})(),
                default_factory=true,
            )]
            field: Vec<i64>,
        }

        #[pymethods]
        impl PyClass {
            #[new]
            fn new(field: Vec<i64>) -> Self {
                Self { field }
            }
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"
from dataclasses import fields

for field in fields(py_class([10, ])):
    if field.name == "field":
        a = field.default_factory()
        b = field.default_factory()
        a.append(1)
        assert a == [1]
        assert b == []
"#
            );
        });
    }
}
