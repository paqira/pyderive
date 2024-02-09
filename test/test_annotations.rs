use pyderive::*;
use pyo3::prelude::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rename() {
        #[derive(PyAnnotations)]
        #[pyclass(get_all, name = "new_name", rename_all = "camelCase")]
        struct PyClass {
            #[pyderive(annotation = "int")]
            #[pyo3(name = "renamed_field")]
            field_a: i64,
            #[pyderive(annotation = "str")]
            field_the_name: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"assert py_class.__annotations__ == {'renamed_field': "'int'", 'fieldTheName': "'str'"}"#
            );
        });
    }

    #[test]
    fn test_noannotations() {
        #[derive(PyAnnotations)]
        #[pyclass]
        #[allow(dead_code)]
        struct PyClass {
            field_a: i64,
            field_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            if py.version_info() >= (3, 10) {
                pyo3::py_run!(py, py_class, "assert py_class.__annotations__ == {}");
            } else {
                // Python < 3.10 does not generate __annotations__ attribute
                // if the field is not annotated
                pyo3::py_run!(
                    py,
                    py_class,
                    "assert hasattr(py_class, '__annotations__') is False"
                );
            }
        })
    }

    #[test]
    fn test_annotations() {
        #[derive(PyAnnotations)]
        #[pyclass]
        #[allow(dead_code)]
        struct PyClass {
            #[pyderive(annotation = "int")]
            field_a: i64,
            #[pyderive(annotation = "str")]
            field_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"assert py_class.__annotations__ == {'field_a': "'int'", 'field_b': "'str'"}"#
            );
        })
    }

    #[test]
    fn test_renamed() {
        #[derive(PyAnnotations)]
        #[pyclass(rename_all = "camelCase")]
        #[allow(dead_code)]
        struct PyClass {
            #[pyderive(annotation = "int")]
            field_a: i64,
            #[pyderive(annotation = "str")]
            #[pyo3(get, name = "new_name")]
            field_b: String,
        }

        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let py_class = py.get_type::<PyClass>();
            pyo3::py_run!(
                py,
                py_class,
                r#"assert py_class.__annotations__ == {'fieldA': "'int'", 'new_name': "'str'"}"#
            );
        })
    }
}
