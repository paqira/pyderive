//! Provides derive macros that implements conversion by built-in functions.

/// Derive macro generating an impl of [`__bool__`][py] method by [`Into<bool>`] trait.
///
/// # Expansion
///
/// This implements, for example:
///
/// ```
/// # use pyo3::prelude::*;
/// # #[pyclass]
/// # struct PyClass {}
/// # impl From<&PyClass> for bool {
/// #    fn from(v: &PyClass) -> bool { true }
/// # }
/// #[pymethods]
/// impl PyClass {
///     fn __bool__(&self) -> bool {
///         Into::into(self)
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
///
/// use pyderive::PyNew;
/// use pyderive::convert::PyBool;
///
/// #[derive(PyNew, PyBool)]
/// #[pyclass]
/// struct PyClass {
///     field: i64
/// }
///
/// impl From<&PyClass> for bool {
///     fn from(value: &PyClass) -> Self {
///         value.field.is_positive()
///     }
/// }
///
/// let test = "
/// actual = bool(PyClass(1))
/// assert isinstance(actual, bool)
/// assert actual is True
/// ";
///
/// Python::with_gil(|py| {
///     let PyClass = py.get_type_bound::<PyClass>();
///     py_run!(py, PyClass, test)
/// });
/// ```
///
/// [py]: https://docs.python.org/3/reference/datamodel.html#object.__bool__
pub use pyderive_macros::PyBool;
/// Derive macro generating an impl of [`__bytes__`][py] method by [`Into<Cow<[u8]>>`][core::convert::Into] trait.
///
/// # Expansion
///
/// This implements:
///
/// ```
/// # use std::borrow::Cow;
/// # use pyo3::prelude::*;
/// # #[pyclass]
/// # struct PyClass {}
/// # impl From<&PyClass> for Cow<'_, [u8]> {
/// # fn from(_value: &PyClass) -> Self {
/// #     vec![].into()
/// #     }
/// # }
/// #[pymethods]
/// impl PyClass {
///     fn __bytes__(&self) -> Cow<[u8]> {
///         Into::into(self)
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use std::borrow::Cow;
///
/// use pyo3::{prelude::*, py_run};
///
/// use pyderive::PyNew;
/// use pyderive::convert::PyBytes;
///
/// #[derive(PyNew, PyBytes)]
/// #[pyclass]
/// struct PyClass {
///     a: u8,
///     b: u8,
///     c: u8
/// }
///
/// impl From<&PyClass> for Cow<'_, [u8]> {
///     fn from(value: &PyClass) -> Self {
///         vec![value.a, value.b, value.c].into()
///     }
/// }
///
/// let test = r#"
/// actual = bytes(PyClass(1, 2, 3))
/// assert isinstance(actual, bytes)
/// assert actual == b'\x01\x02\x03'
/// "#;
///
/// Python::with_gil(|py| {
///     let PyClass = py.get_type_bound::<PyClass>();
///     py_run!(py, PyClass, test)
/// });
/// ```
///
/// [py]: https://docs.python.org/3/reference/datamodel.html#object.__bytes__
pub use pyderive_macros::PyBytes;
/// Derive macro generating an impl of [`__complex__`][py] method by [`Into<num_complex::Complex64>`] trait.
///
/// # Expansion
///
/// This implements, for example:
///
/// ```
/// # use pyo3::prelude::*;
/// # use num_complex;
/// # #[pyclass]
/// # struct PyClass {}
/// # impl From<&PyClass> for num_complex::Complex64 {
/// #    fn from(v: &PyClass) -> num_complex::Complex64 { Self::new(0., 0.) }
/// # }
/// #[pymethods]
/// impl PyClass {
///     fn __complex__(&self) -> num_complex::Complex64 {
///         Into::into(self)
///     }
/// }
/// ```
///
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use num_complex::Complex64;
///
/// use pyderive::PyNew;
/// use pyderive::convert::PyComplex;
///
/// #[derive(PyNew, PyComplex)]
/// #[pyclass]
/// struct PyClass {
///     c: Complex64,
/// }
///
/// impl From<&PyClass> for Complex64 {
///     fn from(value: &PyClass) -> Complex64 {
///         value.c * 2.0
///     }
/// }
///
/// let test = "
/// actual = complex(PyClass(1.0 + 2.0j))
/// assert isinstance(actual, complex)
/// assert actual == 2.0 + 4.0j
/// ";
///
/// Python::with_gil(|py| {
///     let PyClass = py.get_type_bound::<PyClass>();
///     py_run!(py, PyClass, test)
/// });
/// ```
///
/// [py]: https://docs.python.org/3/reference/datamodel.html#object.__complex__
#[cfg_attr(docsrs, doc(cfg(feature = "num-complex")))]
#[cfg(feature = "num-complex")]
pub use pyderive_macros::PyComplex;
/// Derive macro generating an impl of [`__float__`][py] method by [`Into<f64>`] trait.
///
/// # Expansion
///
/// This implements, for example:
///
/// ```
/// # use pyo3::prelude::*;
/// # #[pyclass]
/// # struct PyClass {}
/// # impl From<&PyClass> for f64 {
/// #    fn from(v: &PyClass) -> f64 { 0.0 }
/// # }
/// #[pymethods]
/// impl PyClass {
///     fn __float__(&self) -> f64 {
///         Into::into(self)
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
///
/// use pyderive::PyNew;
/// use pyderive::convert::PyFloat;
///
/// #[derive(PyNew, PyFloat)]
/// #[pyclass]
/// struct PyClass {
///     field: i64
/// }
///
/// impl From<&PyClass> for f64 {
///     fn from(value: &PyClass) -> f64 {
///         (value.field * 2) as f64
///     }
/// }
///
/// let test = "
/// actual = float(PyClass(1))
/// assert isinstance(actual, float)
/// assert actual == 2.0
/// ";
///
/// Python::with_gil(|py| {
///     let PyClass = py.get_type_bound::<PyClass>();
///     py_run!(py, PyClass, test)
/// });
/// ```
///
/// [py]: https://docs.python.org/3/reference/datamodel.html#object.__float__
pub use pyderive_macros::PyFloat;
/// Derive macro generating an impl of [`__index__`][py] method by [`Into<isize>`] trait.
///
/// # Expansion
///
/// This implements, for example:
///
/// ```
/// # use pyo3::prelude::*;
/// # #[pyclass]
/// # struct PyClass {}
/// # impl From<&PyClass> for isize {
/// #    fn from(v: &PyClass) -> isize { 0 }
/// # }
/// #[pymethods]
/// impl PyClass {
///     fn __index__(&self) -> isize {
///         Into::into(self)
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
///
/// use pyderive::PyNew;
/// use pyderive::convert::PyIndex;
///
/// #[derive(PyNew, PyIndex)]
/// #[pyclass]
/// struct PyClass {
///     field: i64
/// }
///
/// impl From<&PyClass> for isize {
///     fn from(value: &PyClass) -> isize {
///         (value.field * 2) as isize
///     }
/// }
///
/// let test = "
/// actual = int(PyClass(1))
/// assert isinstance(actual, int)
/// assert actual == 2
/// ";
///
/// Python::with_gil(|py| {
///     let PyClass = py.get_type_bound::<PyClass>();
///     py_run!(py, PyClass, test)
/// });
/// ```
///
/// [py]: https://docs.python.org/3/reference/datamodel.html#object.__index__
pub use pyderive_macros::PyIndex;
/// Derive macro generating an impl of [`__int__`][py] method by [`Into<i64>`] trait.
///
/// # Expansion
///
/// This implements, for example:
///
/// ```
/// # use pyo3::prelude::*;
/// # #[pyclass]
/// # struct PyClass {}
/// # impl From<&PyClass> for i64 {
/// #    fn from(v: &PyClass) -> i64 { 0 }
/// # }
/// #[pymethods]
/// impl PyClass {
///     fn __int__(&self) -> i64 {
///         Into::into(self)
///     }
/// }
/// ```
///
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
///
/// use pyderive::PyNew;
/// use pyderive::convert::PyInt;
///
/// #[derive(PyNew, PyInt)]
/// #[pyclass]
/// struct PyClass {
///     field: i64
/// }
///
/// impl From<&PyClass> for i64 {
///     fn from(value: &PyClass) -> i64 {
///         value.field * 2
///     }
/// }
///
/// let test = "
/// actual = int(PyClass(1))
/// assert isinstance(actual, int)
/// assert actual == 2
/// ";
///
/// Python::with_gil(|py| {
///     let PyClass = py.get_type_bound::<PyClass>();
///     py_run!(py, PyClass, test)
/// });
/// ```
///
/// [py]: https://docs.python.org/3/reference/datamodel.html#object.__int__
pub use pyderive_macros::PyInt;
