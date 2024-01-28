//! # derive_pysp
//!
//! ```no_run, ignore
//! // Enable `multiple-pymethods` feature of pyo3
//! use derive_pysp::*;
//!
//! // Put #[derive(__init__, ...)] before #[pyclass] to read its attr.
//! #[derive(__init__, __match_args__, __repr__, __eq__, __hash__)]
//! #[pyclass(get_all)]
//! #[derive(PartialEq, Hash)]
//! struct MyClass {
//!     string: String,
//!     integer: i64,
//!     option: Option<i64>
//! }
//! ```
//! ```python
//! # In Python
//! from rust_module import MyClass
//!
//! # Derives __init__ (technically __new__)
//! m = MyClass("a", 1, None)
//! # Derives __match_args__ (supports positional attributes)
//! match m:
//!     case MyClass(a, b, c):
//!         assert a == "a"
//!         assert b == 1
//!         assert c is None
//!     case _:
//!         raise AssertionError
//! # Derives __repr__
//! assert repr(m) == "MyClass(string='a', integer=1, option=None)"
//! # Derives __eq__ based on PartialEq/Eq trait
//! assert m == m
//! # Derives __hash__ based on Hash trait
//! assert hash(m) == 3289857268557676066
//! ```
//!
//! `derive_pysp` provides derive macros of Python special methods and a class attribute.
//!
//! It requires to enable `multiple-pymethods` feature of pyo3 because this may derive multiple `#[pymethods]`.
//!
//! *Note that implementing any of `__eq__`, `__lt__`, `__le__`, `__gt__` and `__ge__` methods will cause Python not to generate a default `__hash__` implementation, so consider also implementing `__hash__`.*
extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput};

mod attr;
mod common;
mod internal;

/// Derive [`__repr__`][__repr__] Python method prints `get` and `set` fileds.
///
/// Place `#[derive(__repr__)]` before `#[pyclass]` to read its attributes.
///
/// [__repr__]: https://docs.python.org/reference/datamodel.html#object.__repr__
/// [repr]: https://docs.python.org/library/functions.html#repr
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use derive_pysp::__repr__;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(__repr__)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
/// }
///
/// # pyo3::prepare_freethreaded_python();
/// Python::with_gil(|py| -> PyResult<()> {
///     let val = PyCell::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///     })?;
///     py_run!(py, val, "assert repr(val) == \"PyClass(string='s', integer=1, float=1.0, tuple=('s', 1, 1.0), option=None)\"");
///     Ok(())
/// });
/// ```
#[proc_macro_derive(__repr__)]
pub fn py_repr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::repr::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__str__`][__str__] Python method prints `get` and `set` fileds.
///
/// Place `#[derive(__str__)]` before `#[pyclass]` to read its attributes.
///
/// [__str__]: https://docs.python.org/reference/datamodel.html#object.__str__
/// [str]: https://docs.python.org/library/functions.html#str
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use derive_pysp::__str__;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(__str__)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
/// }
///
/// # pyo3::prepare_freethreaded_python();
/// Python::with_gil(|py| -> PyResult<()> {
///     let val = PyCell::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///     })?;
///     py_run!(py, val, "assert str(val) == \"PyClass(string='s', integer=1, float=1.0, tuple=('s', 1, 1.0), option=None)\"");
///     Ok(())
/// });
/// ```
#[proc_macro_derive(__str__)]
pub fn py_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::str::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__len__`][__len__] Python method returns number of `get` fields.
///
/// Place `#[derive(__len__)]` before `#[pyclass]` to read its attributes.
///
/// [__len__]: https://docs.python.org/reference/datamodel.html#object.__len__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use derive_pysp::*;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(__len__)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
/// }
///
/// # pyo3::prepare_freethreaded_python();
/// Python::with_gil(|py| -> PyResult<()> {
///     let val = PyCell::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///     })?;
///     py_run!(py, val, "assert len(val) == 5");
///     Ok(())
/// });
/// ```
#[proc_macro_derive(__len__)]
pub fn py_len(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::len::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__iter__`][__iter__] python method return iterator of `get` fields.
///
/// Place `#[derive(__iter__)]` before `#[pyclass]` to read its attributes.
///
/// [__iter__]: https://docs.python.org/reference/datamodel.html#object.__iter__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// # use derive_pysp::*;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(__iter__)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
/// }
///
/// # pyo3::prepare_freethreaded_python();
/// Python::with_gil(|py| -> PyResult<()> {
///     let val = PyCell::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///     })?;
///     py_run!(py, val, "assert tuple(val) == ('s', 1, 1.0, ('s', 1, 1.0), None)");
///     Ok(())
/// });
/// ```
#[proc_macro_derive(__iter__)]
pub fn py_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::iter::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__init__`][__init__] (technically [`__new__`][__new__]) Python method with all fields.
///
/// Place `#[derive(__init__)]` before `#[pyclass]` to read its attributes.
///
/// [__init__]: https://docs.python.org/reference/datamodel.html#object.__init__
/// [__new__]: https://docs.python.org/reference/datamodel.html#object.__new__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use derive_pysp::*;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(__init__)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
/// }
/// #
/// # // preliminary
/// # #[pymodule]
/// # fn module(py: Python<'_>, m: &PyModule) -> PyResult<()> {
/// #    m.add_class::<PyClass>()?;
/// #    Ok(())
/// # }
/// # pyo3::append_to_inittab!(module);
/// # pyo3::prepare_freethreaded_python();
///
/// let script = "
/// from module import PyClass
///
/// a = PyClass('s', 1, 1.0, ('s', 1, 1.0), None)
/// assert a.string == 's'
/// assert a.integer == 1
/// assert a.float == 1.0
/// assert a.tuple == ('s', 1, 1.0)
/// assert a.option is None
/// ";
///
/// assert!(
///     Python::with_gil(|py| Python::run(py, script, None, None))
///     .is_ok()
/// );
/// ```
#[proc_macro_derive(__init__)]
pub fn py_init(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::new::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__eq__`][__eq__] Python method based on [`PartialEq`]/[`Eq`] trait.
///
/// *Note that implementing any of `__eq__` method will cause
/// Python not to generate a default `__hash__` implementation,
/// so consider also implementing `__hash__`.*
///
/// We note that this implements:
///
/// ```ignore
/// pub fn __eq__(&self, other: &Self) -> bool {
///     self.eq(other)
/// }
/// pub fn __ne__(&self, other: &Self) -> bool {
///     self.ne(other)
/// }
/// ```
///
/// [__eq__]: https://docs.python.org/reference/datamodel.html#object.__eq__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// # use derive_pysp::*;
/// #[derive(__eq__)]
/// #[pyclass]
/// #[derive(PartialEq)]
/// struct PyClass {
///     val: f64,
/// }
///
/// # pyo3::prepare_freethreaded_python();
/// Python::with_gil(|py| -> PyResult<()> {
///     let val1 = PyCell::new(py, PyClass { val: 0.0 })?;
///     let val2 = PyCell::new(py, PyClass { val: 1.0 })?;
///     py_run!(py, val1 val2, "assert val1 == val1");
///     py_run!(py, val1 val2, "assert val1 != val2");
///     py_run!(py, val1 val2, "assert val1 != 1");
///
///     let val1 = PyCell::new(py, PyClass { val: f64::NAN })?;
///     py_run!(py, val1, "assert val1 != val1");
///
///     Ok(())
/// });
/// ```
#[proc_macro_derive(__eq__)]
pub fn py_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::eq::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__lt__`][__lt__], [`__le__`][__le__], [`__gt__`][__gt__] and [`__ge__`][__ge__] Python methods based on [`PartialOrd`]/[`Ord`] trait.
///
/// This throws [`TypeError`][TypeError] on incompatible comparision,
/// and returns `false` when [`PartialOrd::partial_cmp`] returns [`None`].
///
/// *Note that implementing any of `__lt__`, `__le__`, `__gt__` and `__ge__` methods
/// will cause Python not to generate a default `__hash__` implementation,
/// so consider also implementing `__hash__`.*
///
/// We note that, for example, this implements:
///
/// ```ignore
/// pub fn __lt__(&self, other: &Self) -> pyo3::PyResult<bool> {
///     match self.partial_cmp(other) {
///         Some(std::cmp::Ordering::Less) => Ok(true),
///         _ => Ok(false),
///     }
/// }
/// ```
///
/// [__lt__]: https://docs.python.org/reference/datamodel.html#object.__lt__
/// [__le__]: https://docs.python.org/reference/datamodel.html#object.__le__
/// [__gt__]: https://docs.python.org/reference/datamodel.html#object.__gt__
/// [__ge__]: https://docs.python.org/reference/datamodel.html#object.__ge__
/// [TypeError]: https://docs.python.org/library/exceptions.html#TypeError
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// # use derive_pysp::*;
/// #[derive(__ord__)]
/// #[pyclass]
/// #[derive(PartialOrd, PartialEq)]
/// struct PyClass {
///     val: f64,
/// }
///
/// # pyo3::prepare_freethreaded_python();
/// Python::with_gil(|py| -> PyResult<()> {
///     let val1 = PyCell::new(py, PyClass { val: 0.0 })?;
///     let val2 = PyCell::new(py, PyClass { val: 1.0 })?;
///     py_run!(py, val1 val2, "assert val1 < val2");
///     py_run!(py, val1 val2, "assert val1 <= val2");
///
///     py_run!(py, val1 val2, "assert not val1 < val1");
///     py_run!(py, val1 val2, "assert val1 <= val1");
///
///     py_run!(py, val1 val2, "assert not val1 > val2");
///     py_run!(py, val1 val2, "assert not val1 >= val2");
///
///     py_run!(py, val1 val2, "assert not val1 > val1");
///     py_run!(py, val1 val2, "assert val1 >= val1");
///
///     let val1 = PyCell::new(py, PyClass { val: f64::NAN })?;
///     py_run!(py, val1, "assert not val1 < val1");
///     py_run!(py, val1, "
/// try:
///     val1 < 1
/// except TypeError:
///     pass
/// else:
///     raise AssertionError");
///
///     Ok(())
/// });
/// ```
#[proc_macro_derive(__ord__)]
pub fn py_ord(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::ord::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__hash__`][__hash__] Python method based on [`Hash`] trait.
///
/// We note that this implements:
///
/// ```ignore
/// pub fn __hash__(&self) -> u64 {
///     use std::collections::hash_map::DefaultHasher;
///     use std::hash::{Hash, Hasher};
///
///     let mut s = DefaultHasher::new();
///     self.hash(&mut s);
///     s.finish()
/// }
/// ```
///
/// [__hash__]: https://docs.python.org/reference/datamodel.html#object.__hash__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// # use derive_pysp::*;
/// #[derive(__hash__)]
/// #[pyclass]
/// #[derive(Hash)]
/// struct PyClass {
///     string: String,
///     integer: i64,
/// }
///
/// # pyo3::prepare_freethreaded_python();
/// Python::with_gil(|py| -> PyResult<()> {
///     let val = PyCell::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///     })?;
///     py_run!(py, val, "assert hash(val) == -1989231435886966707");
///
///     Ok(())
/// });
/// ```
#[proc_macro_derive(__hash__)]
pub fn py_hash(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::hash::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__match_args__`][__match_args__] Python class variable with `get` fields.
///
/// It contains all `get` field names in declaration order, but not the other fileds.
///
/// Place `#[derive(__match_args__)]` before `#[pyclass]` to read its attributes.
///
/// We note that it does not generates `__match_args__` if `get` field is not exists.
///
/// [__match_args__]: https://docs.python.org/reference/datamodel.html#object.__match_args__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use derive_pysp::*;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(__init__, __match_args__)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
/// }
/// #
/// # // preliminary
/// # #[pymodule]
/// # fn module(py: Python<'_>, m: &PyModule) -> PyResult<()> {
/// #    m.add_class::<PyClass>()?;
/// #    Ok(())
/// # }
/// # pyo3::append_to_inittab!(module);
/// # pyo3::prepare_freethreaded_python();
///
/// let script = "
/// from module import PyClass
///
/// match PyClass('s', 1, 1.0, ('s', 1, 1.0), None):
///     case PyClass(a, b, c, d, e):
///         assert a == 's'
///         assert b == 1
///         assert c == 1.0
///         assert d == ('s', 1, 1.0)
///         assert e is None
///     case _:
///         raise AssertionError
/// ";
///
/// assert!(
///     Python::with_gil(|py|
///         if py.version_info() >= (3, 10) {
///             Python::run(py, script, None, None)
///         } else {
///             Ok(())
///         }
///     )
///     .is_ok()
/// )
/// ```
#[proc_macro_derive(__match_args__)]
pub fn py_match_args(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::match_args::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}
