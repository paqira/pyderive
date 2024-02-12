use pyderive::*;
use pyo3::prelude::*;

#[test]
fn test() {
    #[derive(ToPyObject)]
    #[pyclass(get_all)]
    #[derive(Debug, Clone, PartialEq)]
    struct PyClass {
        field: i64,
    }

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let e = PyClass { field: 0 };
        let a = e.to_object(py).extract::<PyClass>(py).unwrap();
        assert_eq!(e, a);
    });
}
