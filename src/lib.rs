//! This library provides derive macros of Python spacial methods and a class attribute for [PyO3].
//!
//! The field attribute `#[pyderive(..)]` helps to costomize implementations,
//! like [`dataclasses.field()`][dataclasses-field] of Python.
//!
//! It requires to enable `multiple-pymethods` feature of PyO3
//! because the derive macros that this library provides may implement multiple `#[pymethods]`.
//!
//! [dataclasses-field]: https://docs.python.org/3/library/dataclasses.html#dataclasses.field
//! [PyO3]: https://github.com/PyO3/pyo3
//!
//! # Example
//!
//! ```
//! // Enable `multiple-pymethods` feature of PyO3
//! use pyo3::prelude::*;
//! use pyderive::*;
//!
//! // Place #[derive(PyInit, ...)] before #[pyclass]
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
//! # Implements __init__() (technically __new__())
//! m = MyClass("a", 1, None)
//!
//! # Implements __match_args__ (supports Pattern Matching by positional arguments)
//! match m:
//!     case MyClass(a, b, c):
//!         assert a == "a"
//!         assert b == 1
//!         assert c is None
//!     case _:
//!         raise AssertionError
//!
//! # Implements __repr__()
//! assert str(m) == "MyClass(string='a', integer=1, option=None)"
//! assert repr(m) == "MyClass(string='a', integer=1, option=None)"
//!
//! # Implements __eq__() based on PartialEq/Eq trait
//! assert m == MyClass("a", 1, None)
//!
//! # Implements __hash__() based on Hash trait
//! assert hash(m) == 3289857268557676066
//! ```
//!
//! # Detail
//!
//! Some macros change implementations depend on `#[pyclass(..)]` and `#[pyo3(..)]` arguments,
//! hence it should place `#[derive(PyInit)]` etc. before `#[pyclass(..)]` and `#[pyo3(..)]`.
//!
//! We list the default implementation that the macros generate.
//!
//! | Derive Macro          | Derives                                                                      |
//! | --------------------- | ---------------------------------------------------------------------------- |
//! | [`PyInit`]            | `__init__()` (`__new__()`) with all fields                                   |
//! | [`PyMatchArgs`]       | `__match_args__` attr. contains `get` field names                            |
//! | [`PyRepr`]            | `__repr__()` returns `get` and `set` fields                                  |
//! | [`PyStr`]             | `__str__()` returns `get` and `set` fields                                   |
//! | [`PyIter`]            | `__iter__()` returns iterator of `get` fields                                |
//! | [`PyLen`]             | `__len__()` returns number of `get` fields                                   |
//! | [`PyDataclassFields`] | `__dataclass_fields__` getter (to support helper functions of [dataclasses]) |
//!
//! [dataclasses]: https://docs.python.org/3/library/dataclasses.html
//!
//! We call the field is *`get` (or `set`) field*
//! if the field has a `#[pyclass/pyo3(get)]` (or `#[pyclass/pyo3(set)]`) attribute or
//! its struct has a `#[pyclass/pyo3(get_all)]` (or `#[pyclass/pyo3(set_all)]`) attribute.
//!
//! The following derive macros depend on traits.
//!
//! | Derive Macro    | Derives                                                                                                         |
//! | --------------- | --------------------------------------------------------------------------------------------------------------- |
//! | [`PyEq`]        | `__eq__()` and `__ne__()` based on [`PartialEq`]/[`Eq`] trait                                                   |
//! | [`PyOrd`]       | `__lt__()`, `__le__()`, `__gt__()` and `__ge__()` based on [`PartialOrd`]/[`Ord`] trait                         |
//! | [`PyHash`]      | `__hash__()` based on [`Hash`] trait and [`hash_map::DefaultHasher`][std::collections::hash_map::DefaultHasher] |
//!
//! # Customize Implementation
//!
//! The field attributes `#[pyderive(..)]` is used to customize implementations
//! produced by [pyderive](crate)'s derive.
//!
//! ```
//! # use pyo3::prelude::*;
//! use pyderive::*;
//!
//! #[derive(PyInit, PyRepr)]
//! #[pyclass]
//! struct MyClass {
//!     str_field: String,
//!     #[pyderive(repr=false)]
//!     #[pyo3(get)]
//!     int_field: i64,
//!     #[pyderive(default=10)]
//!     opt_field: Option<i64>
//! }
//! ```
//!
//! It allows to ommit right-hand side,
//! and it evaluates to right-hand is `true`
//! expcept `default`, for example,
//! `#[pyderive(repr)]` is equivalent to `#[pyderive(repr=true)]`.
//!
//! - `#[pyderive(repr=<bool>)]`
//!
//!    If `repr=true`,
//!    the field is included in the string that the `__repr__()` method returns;
//!    if `repr=false`, it isn't.
//!
//! - `#[pyderive(str=<bool>)]`
//!
//!    If `str=true`,
//!    the field is included in the string that the `__str__()` method returns;
//!    if `str=false`, it isn't.
//!
//! - `#[pyderive(init=<bool>)]`
//!
//!    If `init=true`,
//!    the field is included as the parameters of the `__init__()` (`__new__()` precisely) method;
//!    If `init=false`, it isn't.
//!
//!    The attribute `#[pyderive(default=<expr>)]` is used to costomize default value.
//!    It supports any rust expression which PyO3 supports, e.g.,
//!
//!    ```
//!    # use pyderive::*;
//!    # use pyo3::prelude::*;
//!    #
//!    #[derive(PyInit)]
//!    #[pyclass]
//!    struct PyClass {
//!        #[pyderive(default = Some("str".to_string()))]
//!        field: Option<String>,
//!    }
//!    ```
//!
//!    We note that this internally produce `#[pyo3(signiture=..)]` attribute.
//!
//!     1. No `#[pyderive(..)]` (for example, just `field: i64`)
//!
//!         Pseudo-code:
//!
//!         ```python
//!         def __init__(self, field): self.field = field
//!         ```
//!
//!     2. `#[pyderive(init=false)]`
//!       
//!        The field is not included as the parameter,
//!        and initialized by [`Default::default()`] in the `__init__()` method.         
//!
//!         ```python
//!         def __init__(self): self.field = field::default()  # call rust method
//!         ```
//!
//!     3. `#[pyderive(default=<expr>)]`
//!
//!        The field is included as the parameter with default value `<expr>`.
//!
//!         ```python
//!         def __init__(self, field=<expr>): self.field = field
//!         ```
//!
//!     4. `#[pyderive(init=false, default=<expr>)]`
//!
//!        The field is not included as the parameter,
//!        and initialized with `<expr>` in the `__init__()` method.
//!
//!         ```python
//!         def __init__(self): self.field = <expr>
//!         ```
//!
//! - `#[pyderive(kw_only=true)]`
//!
//!    If `kw_only=true`,
//!    it puts `*,` in front of the field in the argument of the `__init__()` method,
//!    that is, the following fields are keyword only argument.
//!    Note, `kw_only=false` has no effect.
//!
//! - `#[pyderive(match_args=<bool>)]`
//!
//!    If `match_args=true`,
//!    the field is included in the `__match_args__` class attribute;
//!    if `match_args=false`, it isn't.
//!
//!    We note that, as far as I know,
//!    the field must be accessible on the pattern matching.
//!    For example,
//!    pattern matching works for *not* get field with a getter and `#[pyderive(match_args=true)]` attribute,
//!    but it doesn't if the field does not decorated with `#[pyderive(match_args=true)]`.
//!
//! - `#[pyderive(iter=<bool>)]`
//!
//!    If `iter=true`,
//!    the field is included in the iterator that `__iter__()` returns;
//!    if `iter=false`, it isn't.
//!
//! - `#[pyderive(len=<bool>)]`
//!
//!    If `len=true`,
//!    the field is counted by the `__iter__()`;
//!    if `len=false`, it isn't.
//!
//! - `#[pyderive(dataclass_field=<bool>)]`
//!
//!    If `dataclass_field=true`,
//!    the field is included to the return value of the `__dataclass_fields__` getter;
//!    if `dataclass_field=false`, it isn't. See [`PyDataclassFields`] for detail.
//!
extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput};

mod attr;
mod common;
mod internal;

/// Derive macro generating a [`__repr__()`][__repr__] fn/Python method.
///
/// It returns the string that contains `get` and `set` fileds as default,
/// and they are orderd by declaration.
/// It should place `#[derive(PyRepr)]` before `#[pyclass]`.
/// It requires [`ToPyObject`][pyo3_ToPyObject] trait
/// for child [`pyclass`][pyo3_pyclass]es.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
/// [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
///
/// If the filed is deocrated by `#[pyderive(repr=true)]` attribute,
/// the field is included in the string that `__str__()` returns;
/// if `#[pyderive(repr=false)]`, it isn't.
///
/// We note that `#[pyderive(repr)]` is equivalent to `#[pyderive(repr=true)]`.
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
/// // Place before `#[pyclass]`
/// #[derive(PyRepr)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
///     #[pyderive(repr=false)]
///     omitted: String,
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
///         omitted: "omitted".to_string(),
///     })?;
///     py_run!(py, val, r#"assert repr(val) == "PyClass(string='s', integer=1, float=1.0, tuple=('s', 1, 1.0), option=None)""#);
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

/// Derive macro generating a [`__str__()`][__str__] fn/Python method.
///
/// It returns the string that contains `get` and `set` fileds as default,
/// and they are orderd by declaration.
/// It should place `#[derive(PyStr)]` before `#[pyclass]`.
/// It requires [`ToPyObject`][pyo3_ToPyObject] trait
/// for child [`pyclass`][pyo3_pyclass]es.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
/// [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
///
/// If the filed is deocrated by `#[pyderive(str=true)]` attribute,
/// the field is included in the string that `__str__()` returns;
/// if `#[pyderive(str=false)]`, it isn't.
///
/// We note that `#[pyderive(str)]` is equivalent to `#[pyderive(str=true)]`.
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
/// // Place before `#[pyclass]`
/// #[derive(PyStr)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
///     #[pyderive(str=false)]
///     omitted: String,
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
///         omitted: "omitted".to_string(),
///     })?;
///     py_run!(py, val, r#"assert str(val) == "PyClass(string='s', integer=1, float=1.0, tuple=('s', 1, 1.0), option=None)""#);
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

/// Derive macro generating a [`__len__()`][__len__] fn/Python method.
///
/// That returns number of `get` fields as default.
/// It should place `#[derive(PyLen)]` before `#[pyclass]`.
///
/// If the filed is deocrated by `#[pyderive(len=true)]` attribute,
/// the field is counted by the `__len__()`; if `#[pyderive(len=false)]`, it isn't.
///
/// We note that `#[pyderive(len)]` is equivalent to `#[pyderive(len=true)]`.
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
/// // Place before `#[pyclass]`
/// #[derive(PyLen)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
///     #[pyderive(len=false)]
///     omitted: String,
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
///         omitted: "omitted".to_string(),
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

/// Derive macro generating a [`__iter__()`][__iter__] fn/Python method.
///
/// It returns an iterator of `get` fileds as default,
/// and they are orderd by declaration.
/// It should place `#[derive(PyIter)]` before `#[pyclass]`.
/// It requires [`ToPyObject`][pyo3_ToPyObject] trait
/// for child [`pyclass`][pyo3_pyclass]es.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
/// [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
///
/// If the filed is deocrated by `#[pyderive(iter=true)]` attribute,
/// the field is included to the iterartor that `__iter__()` returns;
/// if `#[pyderive(iter=false)]`, it isn't.
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
/// // Place before `#[pyclass]`
/// #[derive(PyIter)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
///     #[pyderive(iter=false)]
///     omitted: String,
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
///         omitted: "omitted".to_string(),
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

/// Derive macro generating a [`__init__()`][__init__] Python method (technically [`__new__()`][__new__]).
///
/// It has all fields as the argumetns as default,
/// and they are orderd by declaration.
/// It should place `#[derive(PyInit)]` before `#[pyclass]`.
///
/// If the filed is deocrated by `#[pyderive(init=true)]` attribute,
/// the field is included to the arguments of the `__init__()`;
/// if `#[pyderive(init=false)]`, it isn't.
/// See the [Customize Implementation of the crate doc](crate) for detail.
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
/// // Place before `#[pyclass]`
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
/// let test = "
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
///     Python::with_gil(|py| Python::run(py, test, None, None)).is_ok()
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

/// Derive macro generating a [`__eq__()`][__eq__] and [`__ne__()`][__ne__] fn/Python methods.
///
/// The implementation is based on [`PartialEq`]/[`Eq`] trait.
///
/// *Note that implementing any of `__eq__()` method will cause
/// Python not to generate a default `__hash__()` implementation,
/// so consider also implementing `__hash__()`.*
///
/// # Expansion
///
/// This implements, for example;
///
/// ```
/// # use pyo3::prelude::*;
/// # #[pyclass]
/// # #[derive(PartialEq)]
/// # struct PyClass {}
/// #[pymethods]
/// impl PyClass {
///     pub fn __eq__(&self, other: &Self) -> bool {
///         self.eq(other)
///     }
///     pub fn __ne__(&self, other: &Self) -> bool {
///         self.ne(other)
///     }
/// }
/// ```
///
/// [__eq__]: https://docs.python.org/reference/datamodel.html#object.__eq__
/// [__ne__]: https://docs.python.org/reference/datamodel.html#object.__ne__
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

/// Derive macro generating [`__lt__()`][__lt__], [`__le__()`][__le__], [`__gt__()`][__gt__] and [`__ge__()`][__ge__] fn/Python methods.
///
/// The implementation is based on [`PartialOrd`]/[`Ord`] trait.
///
/// The generated methods return `False` when [`PartialOrd::partial_cmp`] returns [`None`].
///
/// *Note that implementing any of `__lt__()`, `__le__()`, `__gt__()` and `__ge__()` methods
/// will cause Python not to generate a default `__hash__()` implementation,
/// so consider also implementing `__hash__()`.*
///
/// # Expansion
///
/// This implements, for example;
///
/// ```
/// # use std::cmp::Ordering;
/// # use pyo3::prelude::*;
/// # #[pyclass]
/// # #[derive(PartialOrd, PartialEq)]
/// # struct PyClass {}
/// #[pymethods]
/// impl PyClass {
///     pub fn __lt__(&self, other: &Self) -> pyo3::PyResult<bool> {
///         match self.partial_cmp(other) {
///             Some(Ordering::Less) => Ok(true),
///             _ => Ok(false),
///         }
///     }
///     // and __le__, __gt__ and __ge__
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
/// #[derive(PyOrd)]
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
///     py_run!(py, val1 val2, "assert not val1 > val2");
///     py_run!(py, val1 val2, "assert not val1 >= val2");
///     
///     let test = "
/// try:
///     val1 < 1
/// except TypeError:
///     pass
/// else:
///     raise AssertionError";
///     py_run!(py, val1, test);
///
///     let val1 = PyCell::new(py, PyClass { val: f64::NAN })?;
///     py_run!(py, val1, "assert not val1 < val1");
///
///     Ok(())
/// });
/// ```
#[proc_macro_derive(PyOrd)]
pub fn py_ord(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::ord::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}

/// Derive macro generating a [`__hash__()`][__hash__] fn/Python method.
///
/// The implementation is based on [`Hash`] trait.
///
/// # Expansion
///
/// This implements, for example;
///
/// ```
/// # use pyo3::prelude::*;
/// # #[pyclass]
/// # #[derive(Hash)]
/// # struct PyClass {}
/// #[pymethods]
/// impl PyClass {
///     pub fn __hash__(&self) -> u64 {
///         use std::collections::hash_map::DefaultHasher;
///         use std::hash::{Hash, Hasher};
///
///         let mut s = DefaultHasher::new();
///         self.hash(&mut s);
///         s.finish()
///     }
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

/// Derive macro generating a [`__match_args__`][__match_args__] const/Python class attribute.
///
/// It contains `get` field as default,
/// and they are orderd by declaration.
/// It should place `#[derive(PyMatchArgs)]` before `#[pyclass]`.
///
/// If the filed is deocrated by `#[pyderive(match_args=true)]` attribute,
/// the field is included to the `__match_args__`;
/// if `#[pyderive(match_args=false)]`, it isn't.
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
/// // Place before `#[pyclass]`
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
/// let test = "
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
///             Python::run(py, test, None, None)
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

/// Derive macro generating a `__dataclass_fields__` getter to support helper functions of [dataclasses].
///
/// It returns a [`dataclasses.Field`][Field] dict that helper functions of [dataclasses][dataclasses] use.
/// It supportes
/// [`dataclasses.is_dataclass()`][is_dataclass],
/// [`dataclasses.fields()`][fields],
/// [`dataclasses.asdict()`][asdict] (include nest),
/// [`dataclasses.astuple()`][astuple] (include nest)
/// and [`dataclasses.replace()`][replace].
///
/// It should place `#[derive(PyDataclassField)]` before `#[pyclass]`.
/// This does not generate other fn/method,
/// use [`PyInit`] etc. to implement `__init__()` etc.
/// It requires [`ToPyObject`][pyo3_ToPyObject] trait
/// for child [`pyclass`][pyo3_pyclass]es.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
/// [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
///
/// The resulting dict contains all fields as default.
///
/// If the filed is deocrated by `#[pyderive(dataclass_field=true)]` attribute,
/// the field is included to the dict that `__dataclass_fields__` returns;
/// if `#[pyderive(dataclass_field=false)]`, it isn't.
///
/// ## Notice
///
/// 1. It strognly recomends that all fields in the argument of the constructor
///    (all fields on pyderive as default) must bet `get`, like `dataclass` does.
/// 2. This cannot to handle `default_factory` field of `Field`.
///    The `default` value assigns to the `default` field, not `default_factory`.
/// 3. The resulting `Field`'s fields, `name`, `type`, `init`, `repr` and `kw_only`,
///    have *proper* value but others not, see Appendix.
/// 4. This handles `init=false` field as [`typing.ClassVar`][ClassVar],
///    see Appendix.
/// 5. This derive macro depends on internal behavior of Python.
///
/// ## Appendix
///
/// Table of *non-proper* value fields of `Field`:
///
/// | Field Name        | Value                                                                |
/// | ----------------- | -------------------------------------------------------------------- |
/// | `default`         | `default` value given by `#[pyderive(..)]` or `dataclasses.MISSING`  |
/// | `default_factory` | `dataclasses.MISSING`                                                |
/// | `hash`            | `None`                                                               |
/// | `compare`         | `None`                                                               |
/// | `metadata`        | `None`                                                               |
///
/// Table of `#[pyderive(..)]` v.s. handling:
///
/// | Field Attr.                 | Handling                                      |
/// | --------------------------- | --------------------------------------------- |
/// |`init=true` (default)        | Dataclass field                               |
/// |`init=false`                 | [`typing.ClassVar` field][dataclass_ClassVar] |
/// |`dataclass_field=false`      | Ommit from `__dataclass_fields__`             |
///
/// [dataclasses]: https://docs.python.org/3/library/dataclasses.html
/// [dataclass]: https://docs.python.org/3/library/dataclasses.html#dataclasses.dataclass
/// [Field]: https://docs.python.org/3/library/dataclasses.html#dataclasses.Field
/// [fields]: https://docs.python.org/3/library/dataclasses.html#dataclasses.fields
/// [asdict]: https://docs.python.org/3/library/dataclasses.html#dataclasses.asdict
/// [astuple]: https://docs.python.org/3/library/dataclasses.html#dataclasses.astuple
/// [replace]: https://docs.python.org/3/library/dataclasses.html#dataclasses.replace
/// [is_dataclass]: https://docs.python.org/3/library/dataclasses.html#dataclasses.is_dataclass
/// [ClassVar]: https://docs.python.org/3/library/typing.html#typing.ClassVar
/// [dataclass_ClassVar]: https://docs.python.org/3/library/dataclasses.html#class-variables
///
/// # Example
///
/// ```
/// # use std::error::Error;
/// # use pyderive::*;
/// # use pyo3::prelude::*;
/// # use pyo3::py_run;
/// // Place before `#[pyclass]`
/// #[derive(PyInit, PyDataclassFields)]
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
///
///     py_run!(py, val,
/// "
/// from dataclasses import is_dataclass, asdict, astuple
///
/// assert is_dataclass(val) is True
/// assert asdict(val) == {'string': 's', 'integer': 1, 'float': 1.0, 'tuple': ('s', 1, 1.0), 'option': None}
/// assert astuple(val) == ('s', 1, 1.0, ('s', 1, 1.0), None)
/// "
///     );
///     Ok(())
/// });
/// ```
#[proc_macro_derive(PyDataclassFields, attributes(pyderive))]
pub fn py_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match internal::dataclass_field::implementation(input) {
        Ok(r) => r,
        Err(e) => e.into_compile_error().into(),
    }
}
