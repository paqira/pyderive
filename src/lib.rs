//! ```
//! # use pyo3::prelude::*;
//! // Enable `multiple-pymethods` feature of pyo3
//! use pyderive::*;
//!
//! // Place #[derive(PyInit, ...)] before #[pyclass] to read its attr.
//! #[derive(PyInit, PyMatchArgs, PyRepr, PyEq, PyHash)]
//! #[pyclass(get_all)]
//! #[derive(PartialEq, Hash)]
//! struct MyClass {
//!     string: String,
//!     integer: i64,
//!     option: Option<i64>
//! }
//! ```
//! ```python
//! # Python script
//! from rust_module import MyClass
//!
//! # Derives __init__ (technically __new__)
//! m = MyClass("a", 1, None)
//!
//! # Derives __match_args__ (supports pattern matching by positional arg)
//! match m:
//!     case MyClass(a, b, c):
//!         assert a == "a"
//!         assert b == 1
//!         assert c is None
//!     case _:
//!         raise AssertionError
//!
//! # Derives __repr__
//! assert repr(m) == "MyClass(string='a', integer=1, option=None)"
//!
//! # Derives __eq__ based on PartialEq/Eq trait
//! assert m == m
//!
//! # Derives __hash__ based on Hash trait
//! assert hash(m) == 3289857268557676066
//! ```
//!
//! `pyderive` provides derive macros of Python special methods and a class attribute.
//!
//! It requires to enable `multiple-pymethods` feature of pyo3 because this may derive multiple `#[pymethods]`.
//!
//! *Note that implementing any of `__eq__`, `__lt__`, `__le__`, `__gt__` and `__ge__` methods will cause Python not to generate a default `__hash__` implementation, so consider also implementing `__hash__`.*
//!
//! # Customize Implementation
//!
//! The field attributes `#[pyderive]` is used to customize implementations produced by [pyderive](crate)'s derive.
//!
//! ```rust
//! #[derive(PyInit, PyRepr)]
//! #[pyclass]
//! struct MyClass {
//!     str_field: String,
//!     #[pyderive(repr=false)]
//!     #[pyo3(get)]
//!     int_field: i64,
//!     #[pyderive(default=10)]
//!     opt_field: Option<String>
//! }
//! ```
//!
//! The `#[pyderive]` overwrites default behavior.
//!
//! - `#[pyderive(repr=true/false)]`
//!
//!    If `true`, the field is included in the string that the generated `__repr__` method returns.
//!
//!    Notes, `#[pyderive(repr)]` is equivalent to `#[pyderive(repr=true)]`.
//! - `#[pyderive(str=true/false)]`
//!
//!    If `true`, the field is included in the string that the generated `__str__` method returns.
//!
//!    Notes, `#[pyderive(str)]` is equivalent to `#[pyderive(str=true)]`.
//! - `#[pyderive(init=true/false)]`
//!
//!    If `true`, the field is included as a parameter of generated `__init__` (`__new__` precisely) method.
//!
//!    Notes, `#[pyderive(init)]` is equivalent to `#[pyderive(init=true)]`.
//!    
//!    This supports default value with `#[pyderive(default)]` attribute.
//!    We note that this internally produce `#[pyo3(signiture)]` field attribute.
//!    We list the equivarent Python code of `init` and `default` specification:
//!
//!     1. `#[pyderive] field: i64` or just `field: i64` (no `#[pyderive]`)
//!
//!         ```python
//!         def __init__(self, field): self.field = field
//!         ```
//!     2. `#[pyderive(init=false)] field: i64`
//!       
//!        The field is not included as the parameter,
//!        and initialized by [`Default::default()`] in the `__init__` method.         
//!
//!         ```python
//!         def __init__(self): self.field = field::default()  # call rust method
//!         ```
//!     3. `#[pyderive(default=<Literal>)] field: i64`
//!
//!        The field is included as the parameter with default value `<Literal>`.
//!
//!         ```python
//!         def __init__(self, field=<Literal>): self.field = field
//!         ```
//!     4. `#[pyderive(init=false, default=<Literal>)] field: i64`
//!
//!        The field is not included as the parameter,
//!        and initialized by `<Literal>` in the `__init__` method.     
//!
//!         ```python
//!         def __init__(self): self.field = <Literal>
//!         ```
//! - `#[pyderive(kw_only=true/false)]`
//!
//!    If `true`, put `*,` in front of this field in the argument of generated `__init__` method,
//!    hence, the following fields are keyword only argument.
//!
//!    Notes, `#[pyderive(kw_only)]` is equivalent to `#[pyderive(kw_only=true)]`
//! - `#[pyderive(match_args=true/false)]`
//!
//!    If `true`, the field is included in the generated `__match_args__` class attribute.
//!
//!    Notes, `#[pyderive(match_args)]` is equivalent to `#[pyderive(match_args=true)]`
//! - `#[pyderive(iter=true/false)]`
//!
//!    If `true`, the field is included in the iterator that generated `__iter__` method returs.
//!
//!    Notes, `#[pyderive(iter)]` is equivalent to `#[pyderive(iter=true)]`
//! - `#[pyderive(len=true/false)]`
//!
//!    If `true`, `__len__` the field is counted in the generated `__len__` method.
//!
//!    Notes, `#[pyderive(len)]` is equivalent to `#[pyderive(len=true)]`
extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput};

mod attr;
mod common;
mod internal;

/// Derive [`__repr__`][__repr__] that prints `get` and `set` fileds.
///
/// Place `#[derive(__repr__)]` before `#[pyclass]` to read its attributes.
///
/// See the [Customize Implementation of crate doc](crate) to customize implementation.
///
/// [__repr__]: https://docs.python.org/reference/datamodel.html#object.__repr__
/// [repr]: https://docs.python.org/library/functions.html#repr
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyderive::*;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(PyRepr)]
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
#[proc_macro_derive(PyRepr, attributes(pyderive))]
pub fn py_repr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::repr::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__str__`][__str__] that prints `get` and `set` fileds.
///
/// Place `#[derive(__str__)]` before `#[pyclass]` to read its attributes.
///
/// See the [Customize Implementation of crate doc](crate) to customize implementation.
///
/// [__str__]: https://docs.python.org/reference/datamodel.html#object.__str__
/// [str]: https://docs.python.org/library/functions.html#str
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyderive::*;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(PyStr)]
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
#[proc_macro_derive(PyStr, attributes(pyderive))]
pub fn py_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::str::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__len__`][__len__] that returns number of `get` fields.
///
/// Place `#[derive(__len__)]` before `#[pyclass]` to read its attributes.
///
/// See the [Customize Implementation of crate doc](crate) to customize implementation.
///
/// [__len__]: https://docs.python.org/reference/datamodel.html#object.__len__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyderive::*;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(PyLen)]
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
#[proc_macro_derive(PyLen, attributes(pyderive))]
pub fn py_len(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::len::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__iter__`][__iter__] that return iterator of `get` fields.
///
/// Place `#[derive(__iter__)]` before `#[pyclass]` to read its attributes.
///
/// See the [Customize Implementation of crate doc](crate) to customize implementation.
///
/// [__iter__]: https://docs.python.org/reference/datamodel.html#object.__iter__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// # use pyderive::*;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(PyIter)]
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
#[proc_macro_derive(PyIter, attributes(pyderive))]
pub fn py_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::iter::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__init__`][__init__] (technically [`__new__`][__new__]).
///
/// Place `#[derive(__init__)]` before `#[pyclass]` to read its attributes.
///
/// See the [Customize Implementation of crate doc](crate) to customize implementation.
///
/// [__init__]: https://docs.python.org/reference/datamodel.html#object.__init__
/// [__new__]: https://docs.python.org/reference/datamodel.html#object.__new__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyderive::*;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(PyInit)]
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
#[proc_macro_derive(PyInit, attributes(pyderive))]
pub fn py_init(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::init::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__eq__`][__eq__] based on [`PartialEq`]/[`Eq`] trait.
///
/// *Note that implementing any of `__eq__` method will cause
/// Python not to generate a default `__hash__` implementation,
/// so consider also implementing `__hash__`.*
///
/// We note that this implements:
///
/// ```no_run, ignore
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
/// # use pyderive::*;
/// #[derive(PyEq)]
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
#[proc_macro_derive(PyEq)]
pub fn py_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::eq::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__lt__`][__lt__], [`__le__`][__le__], [`__gt__`][__gt__] and [`__ge__`][__ge__] based on [`PartialOrd`]/[`Ord`] trait.
///
/// The generated methods return `False` when [`PartialOrd::partial_cmp`] returns [`None`],
/// and throw [`TypeError`][TypeError] on incompatible comparision.
///
/// *Note that implementing any of `__lt__`, `__le__`, `__gt__` and `__ge__` methods
/// will cause Python not to generate a default `__hash__` implementation,
/// so consider also implementing `__hash__`.*
///
/// We note that, for example, this implements:
///
/// ```no_run, ignore
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
/// # use pyderive::*;
/// #[derive(PyOrder)]
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
#[proc_macro_derive(PyOrder)]
pub fn py_ord(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::order::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__hash__`][__hash__] based on [`Hash`] trait.
///
/// We note that this implements:
///
/// ```no_run, ignore
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
/// # use pyderive::*;
/// #[derive(PyHash)]
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
#[proc_macro_derive(PyHash)]
pub fn py_hash(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::hash::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive [`__match_args__`][__match_args__] with `get` fields.
///
/// It contains all `get` field names in declaration order, but not the other fileds.
///
/// Place `#[derive(__match_args__)]` before `#[pyclass]` to read its attributes.
///
/// We note that it does not generates `__match_args__` if `get` field is not exists.
///
/// See the [Customize Implementation of crate doc](crate) to customize implementation.
///
/// [__match_args__]: https://docs.python.org/reference/datamodel.html#object.__match_args__
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyderive::*;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Put before `#[pyclass]` to read its attributes.
/// #[derive(PyInit, PyMatchArgs)]
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
#[proc_macro_derive(PyMatchArgs, attributes(pyderive))]
pub fn py_match_args(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::match_args::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}
