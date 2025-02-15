#![cfg_attr(docsrs, feature(doc_cfg))]

//! This library provides derive macros of Python spacial methods and a class attributes for [PyO3].
//!
//! The field attribute `#[pyderive(..)]` helps to customize implementations,
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
//! // Place #[derive(PyNew, ...)] before #[pyclass]
//! #[derive(PyNew, PyMatchArgs, PyRepr, PyEq)]
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
//!
//! # Derives __new__()
//! m = MyClass("a", 1, None)
//!
//! # Derives __match_args__ (supports Pattern Matching by positional arguments)
//! match m:
//!     case MyClass(a, b, c):
//!         assert a == "a"
//!         assert b == 1
//!         assert c is None
//!     case _:
//!         raise AssertionError
//!
//! # Derives __repr__(), calls Python repr() recursively
//! assert str(m) == "MyClass(string='a', integer=1, option=None)"
//! assert repr(m) == "MyClass(string='a', integer=1, option=None)"
//!
//! # Derives __eq__() that depends on PartialEq trait
//! assert m == MyClass("a", 1, None)
//! ```
//!
//! # Detail
//!
//! Some macros change implementations depend on `#[pyclass(..)]` and `#[pyo3(..)]` arguments,
//! hence it should place `#[derive(PyNew)]` etc. before `#[pyclass(..)]` and `#[pyo3(..)]`.
//!
//! We list the default implementations that the macros generate.
//!
//! | Derive Macro          | Derives                                              |
//! | --------------------- | ---------------------------------------------------- |
//! | [`PyNew`]             | `__new__()` with all fields                          |
//! | [`PyMatchArgs`]       | `__match_args__` class attr. with `get` fields       |
//! | [`PyRepr`]            | `__repr__()` returns `get` and `set` fields          |
//! | [`PyStr`]             | `__str__()` returns `get` and `set` fields           |
//! | [`PyIter`]            | `__iter__()` returns an iterator of `get` fields     |
//! | [`PyReversed`]        | `__reversed__()` returns an iterator of `get` fields |
//! | [`PyLen`]             | `__len__()` returns number of `get` fields           |
//! | [`PyDataclassFields`] | `__dataclass_fields__` class attr. with all fields   |
//!
//! Notes, methods implemented by [`PyRepr`] and [`PyStr`] are recursively calls `repr()` or `str()` like a Python `dataclass`.
//!
//! We call the field is *`get` (or `set`) field*
//! if the field has a `#[pyclass/pyo3(get)]` (or `#[pyclass/pyo3(set)]`) attribute or
//! its struct has a `#[pyclass/pyo3(get_all)]` (or `#[pyclass/pyo3(set_all)]`) attribute.
//!
//! The following derive macros depend on traits.
//!
//! | Derive Macro    | Derives                                                                                            |
//! | --------------- | -------------------------------------------------------------------------------------------------- |
//! | [`PyEq`]        | `__eq__()` and `__ne__()`, depends on [`PartialEq`]                                                |
//! | [`PyOrd`]       | `__lt__()`, `__le__()`, `__gt__()` and `__ge__()`, depend on [`PartialOrd`]                        |
//! | [`PyRichCmp`]   | `==`, `!=`, `>`, `>=`, `<` and `<=` by `__richcmp__()`, depend on [`PartialEq`] and [`PartialOrd`] |
//! | [`PyNumeric`]   | Numeric op traits (`__add__()` etc.)                                                               |
//! | [`PyBitwise`]   | Bitwise op traits (`__and__()` etc.)                                                               |
//!
//! Notes, implementation of [`PyEq`] and [`PyOrd`] does not use `__richcmp__()`.
//!
//! Module [`pyderive::ops`](mod@ops) and [`pyderive::convert`](mod@convert) provides
//! derive macros that implement individual method that enumerating numeric type (`__add__()` etc.) and
//! called by builtin functions (`__int__()` etc.).
//!
//! [pyo3_IntoPy]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.IntoPy.html
//! [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
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
//! #[derive(PyNew, PyRepr)]
//! #[pyclass]
//! struct MyClass {
//!     string: String,
//!     #[pyderive(repr=false)]
//!     #[pyo3(get)]
//!     integer: i64,
//!     #[pyderive(default=10)]
//!     option: Option<i64>
//! }
//! ```
//!
//! It allows to omit the right-hand side,
//! and it evaluates to the right-hand as `true`
//! except `default` , for example,
//! `#[pyderive(repr)]` is equivalent to `#[pyderive(repr=true)]`.
//!
//! - `#[pyderive(repr=<bool>)]`
//!
//!    If `repr=true`,
//!    the field is included in the string that the `__repr__()` method returns;
//!    if `repr=false`, it isn't.
//!
//!    The derive macro [`PyDataclassFields`] reads this attribute also,
//!    see [`PyDataclassFields`] for detail.
//!
//! - `#[pyderive(str=<bool>)]`
//!
//!    If `str=true`,
//!    the field is included in the string that the `__str__()` method returns;
//!    if `str=false`, it isn't.
//!
//! - `#[pyderive(new=<bool>)]`
//!
//!    If `new=false`,
//!    the field is excluded from the arguments of the `__new__()` method.
//!    Notes, `new=true` has no effect.
//!
//!    The derive macro [`PyDataclassFields`] reads this attribute also,
//!    see [`PyDataclassFields`] for detail.
//!
//! - `#[pyderive(default=<expr>)]`
//!
//!    This is used to customize default value for the `__new__()` method.
//!    It supports any rust expression which PyO3 supports, e.g.,
//!
//!    ```
//!    # use pyderive::*;
//!    # use pyo3::prelude::*;
//!    #
//!    #[derive(PyNew)]
//!    #[pyclass]
//!    struct PyClass {
//!        #[pyderive(default = Some("str".to_string()))]
//!        field: Option<String>,
//!    }
//!    ```
//!
//!    We note that this internally produces `#[pyo3(signature = ..)]` attribute.
//!
//!     1. No `#[pyderive(..)]` (for example, just `field: i64`)
//!
//!        Pseudocode:
//!
//!        ```python
//!        def __new__(cls, field):
//!             self = super().__new__(cls)
//!             self.field = field
//!             return self
//!        ```
//!
//!     2. `#[pyderive(new=false)]`
//!        
//!        The field is excluded from the arguments,
//!        and initialized by [`Default::default()`] in the `__new__()` method.
//!        We note that it is evaluated on every `__new__()` call.
//!
//!        Pseudocode:
//!
//!        ```python
//!        def __new__(cls):
//!             self = super().__new__(cls)
//!             self.field = field::default()  # call rust fn
//!             return self
//!        ```
//!
//!     3. `#[pyderive(default=<expr>)]`
//!
//!        The field is included to the arguments with default value `<expr>`.
//!        We note that `<expr>` (rust code) is evaluated on every `__new__()` call (PyO3 feature).
//!
//!        Pseudocode:
//!
//!        ```python
//!        def __new__(cls, field=<expr>):
//!             self = super().__new__(cls)
//!             self.field = field
//!             return self
//!        ```
//!
//!     4. `#[pyderive(new=false, default=<expr>)]`
//!
//!        The field is excluded from the arguments,
//!        and initialized with `<expr>` in the `__new__()` method.
//!        We note that `<expr>` (rust code) is evaluated on every `__new__()` call.
//!
//!        Pseudocode:
//!
//!        ```python
//!        def __new__(cls):
//!             self = super().__new__(cls)
//!             self.field = <expr>
//!             return self
//!        ```
//!
//! - `#[pyderive(default_factory=true)]`
//!
//!    If `default_factory=true`,
//!    let the `default_factory` attribute of `Field`obj be `lambda: <expr>`,
//!    and let the `default` attribute be [`dataclasses.MISSING`][MISSING],
//!    where `<expr>` is given by `#[pyderive(default=<expr>)]`.
//!    Notes, `default_factory=false` has no effect,
//!    If the field is not marked by `#[pyderive(default=<expr>)]`, this ignores.
//!    
//!    See [`PyDataclassFields`] for detail.
//!
//! - `#[pyderive(kw_only=true)]`
//!
//!    If `kw_only=true`,
//!    the following fields are keyword only arguments in the `__new__()` method,
//!    like [`*`][keyword-only-arguments] and [`dataclasses.KW_ONLY`][KW_ONLY].
//!    Note, `kw_only=false` has no effect.
//!
//!    The derive macro [`PyDataclassFields`] reads this attribute also,
//!    see [`PyDataclassFields`] for detail.
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
//!    pattern matching does *not* work with *not `get` field without a getter*
//!    (even if `match_args=true`), but it does work if the field has a getter.
//!
//! - `#[pyderive(iter=<bool>)]`
//!
//!    If `iter=true`,
//!    the field is included in the iterator that `__iter__()` and `__reversed__()` return;
//!    if `iter=false`, it isn't.
//!
//! - `#[pyderive(len=<bool>)]`
//!
//!    If `len=true`,
//!    the field is counted by the `__len__()`;
//!    if `len=false`, it isn't.
//!
//! - `#[pyderive(dataclass_field=false)]`
//!
//!    If `dataclass_field=false`,
//!    the field is excluded from the `__dataclass_fields__` dict.
//!    Notes, `dataclass_field=true` has no effect.
//!
//!    See [`PyDataclassFields`] for detail.
//!
//! - `#[pyderive(annotation=<str>)]`
//!
//!    The derive macro [`PyDataclassFields`] reads this attribute,
//!    see [`PyDataclassFields`] for detail.
//!
//! [keyword-only-arguments]: https://docs.python.org/3/tutorial/controlflow.html#keyword-only-arguments
//! [KW_ONLY]: https://docs.python.org/3/library/dataclasses.html#dataclasses.KW_ONLY
//! [MISSING]: https://docs.python.org/3/library/dataclasses.html#dataclasses.MISSING

pub mod convert;
pub mod ops;

/// Derive macro generating a `__dataclass_fields__` fn/Python class attribute.
///
/// It returns a [`dataclasses.Field`][Field] dict that helper functions of the [dataclasses] module read.
/// It supports [`is_dataclass()`][is_dataclass], [`fields()`][fields],
/// [`asdict()`][asdict] (include nest), [`astuple()`][astuple] (include nest)
/// and [`replace()`][replace] of the dataclasses module.
///
/// The resulting dict contains all fields as default.
///
/// If the filed is marked by `#[pyderive(dataclass_field=false)]` attribute,
/// the field is excluded from the dict that `__dataclass_fields__` returns.
/// Notes, `dataclass_field=true` has no effect.
///
/// - It should place `#[derive(PyDataclassField)]` before `#[pyclass]`.
/// - All fields in the arguments of the `__new__()` method should be `get` field, like `dataclass` does.
/// - It requires [`IntoPyObject`][pyo3_IntoPyObject] trait for fields.
///
/// This does not generate other fn/method,
/// use [`PyNew`] etc. to implement `__new__()` etc.
///
/// [pyo3_IntoPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.IntoPyObject.html
/// [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
/// // Place before `#[pyclass]`
/// #[derive(PyNew, PyDataclassFields)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
///     #[pyderive(dataclass_field=false)]
///     excluded: String,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///         excluded: "s".to_string(),
///     })?;
///
///     let test = "
/// from dataclasses import is_dataclass, asdict, astuple
///
/// assert is_dataclass(a) is True
/// assert asdict(a) == {'string': 's', 'integer': 1, 'float': 1.0, 'tuple': ('s', 1, 1.0), 'option': None}
/// assert astuple(a) == ('s', 1, 1.0, ('s', 1, 1.0), None)
/// ";
///     py_run!(py, a, test);
///
///     Ok(())
/// });
/// ```
///
/// # Implementation Notes
///
/// | `dataclasses.Field` Attribute | Compatibility                      |
/// | ----------------------------- | ---------------------------------- |
/// | `name`                        | ✅                                 |
/// | `type`                        | ❌ (✅ if `annotation` given)      |
/// | `default`                     | ✅ (`<expr>` or `MISSING`)         |
/// | `default_factory`             | ✅ (`lambda: <expr>` or `MISSING`) |
/// | `new`                         | ✅                                 |
/// | `repr`                        | ✅                                 |
/// | `hash`                        | ❌ (`None` for pyderive)           |
/// | `compare`                     | ❌ (`None` for pyderive)           |
/// | `metadata`                    | ✅ (empty for pyderive)            |
/// | `kw_only`                     | ✅                                 |
///
/// 1. The `type` attribute of `Field` is `None` as default.
///    If the field is marked by `#[pyderive(annotation=<type>)]`,
///    this uses the given `<type>` as `type` attribute.
/// 2. If the field is marked by `#[pyderive(default_factory=true)]`,
///    the `default` attribute of the resulting `Field` obj is [`MISSING`][MISSING]
///    and the `default_factory` is `lambda: <expr>`.
///    Notes, it evaluates `<expr>` on every `Field.default_factory` call.
///
///    | Rust Field Attribute                | Python `default` Attribute | Python `default_factory` Attribute |
///    | ----------------------------------- | -------------------------- | ---------------------------------- |
///    | `#[pyderive(default_factory=true)]` | `MISSING`                  | `lambda: <expr>`                   |
///    | Other                               | `<expr>`                   | `MISSING`                          |
/// 3. Attributes `hash` and `compare` are `None`.
/// 4. This marks `new=false` field as a [`ClassVar` field][dataclass_ClassVar].
///
///    | Field Attribute        | Result                                 |
///    | ---------------------- | -------------------------------------- |
///    |`new=true` (default)    | Dataclass field                        |
///    |`new=false`             | [`ClassVar` field][dataclass_ClassVar] |
///    |`dataclass_field=false` | Exclude from `__dataclass_fields__`    |
/// 5. The [PEP 487][PEP487] ([`__set_name__()`][set_name] hook) is not supported
///    (The default value of `__dataclass_fields__` is a different object
///    from `__new__()`'s one, that is, they have different object IDs.
///    This calls `__set_name__()` of `__dataclass_fields__` only,
///    but not `__new__()`'s one).
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
/// [MISSING]: https://docs.python.org/3/library/dataclasses.html#dataclasses.MISSING
/// [PEP487]: https://peps.python.org/pep-0487/
/// [set_name]: https://docs.python.org/3/reference/datamodel.html#object.__set_name__
pub use pyderive_macros::PyDataclassFields;
/// Derive macro generating a [`__eq__()`][__eq__] and [`__ne__()`][__ne__] fn/Python methods.
///
/// The implementation requires [`PartialEq`] impl.
///
/// *Note that implementing `__eq__()` and `__ne__()` methods will cause
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
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
/// #[derive(PyEq)]
/// #[pyclass]
/// #[derive(PartialEq)]
/// struct PyClass {
///     field: f64,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass { field: 0.0 })?;
///     let b = Py::new(py, PyClass { field: 1.0 })?;
///     let c = Py::new(py, PyClass { field: f64::NAN })?;
///
///     py_run!(py, a b, "assert a == a");
///     py_run!(py, a b, "assert a != b");
///     py_run!(py, c, "assert c != c");
///     py_run!(py, a, "assert a != 1");
///
///     Ok(())
/// });
/// ```
pub use pyderive_macros::PyEq;
/// Derive macro generating a [`__hash__()`][__hash__] fn/Python method.
///
/// The implementation requires [`Hash`] impl.
///
/// <section class="warning">
/// PyO3 supports <code>#[pyclass(hash)]</code> since 0.22, it is recommended to use it.
/// </section>
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
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
/// #[derive(PyHash)]
/// #[pyclass]
/// #[derive(Hash)]
/// struct PyClass {
///     string: String,
///     integer: i64,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///     })?;
///
///     py_run!(py, a, "assert hash(a) == -1989231435886966707");
///
///     Ok(())
/// });
/// ```
#[deprecated(
    since = "0.8.0",
    note = "PyO3 provides the equivalent feature `#[pyclass(hash)]` since PyO3 0.23.0."
)]
pub use pyderive_macros::PyHash;
/// Derive macro generating a [`__iter__()`][__iter__] fn/Python method.
///
/// It returns an iterator of `get` fields as default,
/// in the order of declaration.
///
/// If the filed is marked by `#[pyderive(iter=true)]` attribute,
/// the field is included to the iterator that `__iter__()` returns;
/// if `#[pyderive(iter=false)]`, it isn't.
///
/// - It should place `#[derive(PyIter)]` before `#[pyclass]`.
/// - It requires [`IntoPyObject`][pyo3_IntoPyObject] trait for fields.
/// - Calling `__next__()` is thread-safe, it raises `PyRuntimeError` when it fails to take a lock.
///
/// [pyo3_IntoPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.IntoPyObject.html
/// [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
/// [__iter__]: https://docs.python.org/reference/datamodel.html#object.__iter__
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
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
///     excluded: String,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///         excluded: "excluded".to_string(),
///     })?;
///
///     py_run!(py, a, "assert tuple(a) == ('s', 1, 1.0, ('s', 1, 1.0), None)");
///
///     Ok(())
/// });
/// ```
pub use pyderive_macros::PyIter;
/// Derive macro generating a [`__len__()`][__len__] fn/Python method.
///
/// That returns number of `get` fields as default.
///
/// If the filed is marked by `#[pyderive(len=true)]` attribute,
/// the field is counted by the `__len__()`; if `#[pyderive(len=false)]`, it isn't.
///
/// - It should place `#[derive(PyLen)]` before `#[pyclass]`.
///
/// [__len__]: https://docs.python.org/reference/datamodel.html#object.__len__
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
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
///     excluded: String,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///         excluded: "excluded".to_string(),
///     })?;
///
///     py_run!(py, a, "assert len(a) == 5");
///
///     Ok(())
/// });
/// ```
pub use pyderive_macros::PyLen;
/// Derive macro generating a [`__match_args__`][__match_args__] const/Python class attribute.
///
/// It contains `get` fields as default,
/// in the order of declaration.
///
/// If the filed is marked by `#[pyderive(match_args=true)]` attribute,
/// the field is included to the `__match_args__`;
/// if `#[pyderive(match_args=false)]`, it isn't.
///
/// - It should place `#[derive(PyMatchArgs)]` before `#[pyclass]`.
///
/// [__match_args__]: https://docs.python.org/reference/datamodel.html#object.__match_args__
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
/// // Place before `#[pyclass]`
/// #[derive(PyNew, PyMatchArgs)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
///     #[pyderive(match_args=false)]
///     excluded: String,
/// }
///
/// let test = "
/// match PyClass('s', 1, 1.0, ('s', 1, 1.0), None, 's'):
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
/// Python::with_gil(|py| {
///     if py.version_info() >= (3, 10) {
///         let PyClass = py.get_type::<PyClass>();
///
///         py_run!(py, PyClass, test)
///     }
/// });
/// ```
pub use pyderive_macros::PyMatchArgs;
/// Derive macro generating a [`__new__()`][__new__] Python method.
///
/// It has all fields as the arguments as default,
/// in the order of declaration.
///
/// If the filed is marked by `#[pyderive(new=false)]` attribute,
/// the field is excluded from the arguments of the `__new__()` method.
/// Notes, `new=true` has no effect.
///
/// - It should place `#[derive(PyNew)]` before `#[pyclass]`.
///
/// See the [Customize Implementation](crate) section of the crate doc for detail.
///
/// [__new__]: https://docs.python.org/reference/datamodel.html#object.__new__
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
/// // Place before `#[pyclass]`
/// #[derive(PyNew)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
///     #[pyderive(new=false)]
///     excluded: String,
/// }
///
/// let test = "
/// a = PyClass('s', 1, 1.0, ('s', 1, 1.0), None)
/// assert a.string == 's'
/// assert a.integer == 1
/// assert a.float == 1.0
/// assert a.tuple == ('s', 1, 1.0)
/// assert a.option is None
/// assert a.excluded == ''
/// ";
///
/// Python::with_gil(|py| {
///     let PyClass = py.get_type::<PyClass>();
///
///     py_run!(py, PyClass, test)
/// });
/// ```
pub use pyderive_macros::PyNew;
/// Derive macro generating [`__lt__()`][__lt__], [`__le__()`][__le__], [`__gt__()`][__gt__] and [`__ge__()`][__ge__] fn/Python methods.
///
/// The implementation requires [`PartialOrd`] impl.
///
/// <section class="warning">
/// PyO3 supports <code>#[pyclass(ord)]</code> since 0.22.
/// </section>
///
/// The generated methods return `False` when [`PartialOrd::partial_cmp`] returns [`None`].
///
/// *Note that implementing `__lt__()`, `__le__()`, `__gt__()` and `__ge__()` methods
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
///     pub fn __lt__(&self, other: &Self) -> bool {
///         matches!(self.partial_cmp(other), Some(Ordering::Less))
///     }
///     // and __le__, __gt__ and __ge__
/// }
/// ```
///
/// [__lt__]: https://docs.python.org/reference/datamodel.html#object.__lt__
/// [__le__]: https://docs.python.org/reference/datamodel.html#object.__le__
/// [__gt__]: https://docs.python.org/reference/datamodel.html#object.__gt__
/// [__ge__]: https://docs.python.org/reference/datamodel.html#object.__ge__
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
/// #[derive(PyOrd)]
/// #[pyclass]
/// #[derive(PartialOrd, PartialEq)]
/// struct PyClass {
///     field: f64,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass { field: 0.0 })?;
///     let b = Py::new(py, PyClass { field: 1.0 })?;
///     let c = Py::new(py, PyClass { field: f64::NAN })?;
///
///     py_run!(py, a b, "assert a < b");
///     py_run!(py, a b, "assert a <= b");
///     py_run!(py, a b, "assert not a > b");
///     py_run!(py, a b, "assert not a >= b");
///     py_run!(py, c, "assert not c < c");
///
///     let test = "
/// try:
///     a < 1
/// except TypeError:
///     pass
/// else:
///     raise AssertionError
/// ";
///     py_run!(py, a, test);
///
///     Ok(())
/// });
/// ```
pub use pyderive_macros::PyOrd;
/// Derive macro generating a [`__repr__()`][__repr__] fn/Python method.
///
/// It returns the string that contains `get` and `set` fields as default,
/// in the order of declaration.
///
/// If the filed is marked by `#[pyderive(repr=true)]` attribute,
/// the field is included in the string that `__str__()` returns;
/// if `#[pyderive(repr=false)]`, it isn't.
///
/// - It should place `#[derive(PyRepr)]` before `#[pyclass]`.
/// - It requires [`IntoPyObject`][pyo3_IntoPyObject] trait for fields.
/// - This recursively calls `repr()` like a dataclass.
///
/// [pyo3_IntoPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.IntoPyObject.html
/// [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
/// [__repr__]: https://docs.python.org/reference/datamodel.html#object.__repr__
/// [repr]: https://docs.python.org/library/functions.html#repr
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
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
///     excluded: String,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///         excluded: "excluded".to_string(),
///     })?;
///
///     py_run!(py, a, r#"assert repr(a) == "PyClass(string='s', integer=1, float=1.0, tuple=('s', 1, 1.0), option=None)""#);
///
///     Ok(())
/// });
/// ```
pub use pyderive_macros::PyRepr;
/// Derive macro generating a [`__reversed__()`][__reversed__] fn/Python method.
///
/// It returns an iterator of `get` fields as default,
/// in the reverse order of declaration.
///
/// This is a reversed one of a derive macro, [`PyIter`].
///
/// If the filed is marked by `#[pyderive(iter=true)]` attribute,
/// the field is included to the iterator that `__reversed__()` returns;
/// if `#[pyderive(iter=false)]`, it isn't.
///
/// - It should place `#[derive(PyReversed)]` before `#[pyclass]`.
/// - It requires [`IntoPyObject`][pyo3_IntoPyObject] trait for fields.
/// - Calling `__next__()` is thread-safe, it raises `PyRuntimeError` when it fails to take a lock.
///
/// [pyo3_IntoPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.IntoPyObject.html
/// [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
/// [__reversed__]: https://docs.python.org/reference/datamodel.html#object.__reversed__
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
/// // Place before `#[pyclass]`
/// #[derive(PyReversed)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     string: String,
///     integer: i64,
///     float: f64,
///     tuple: (String, i64, f64),
///     option: Option<String>,
///     #[pyderive(iter=false)]
///     excluded: String,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///         excluded: "excluded".to_string(),
///     })?;
///
///     py_run!(py, a, "assert tuple(reversed(a)) == (None, ('s', 1, 1.0), 1.0, 1, 's')");
///
///     Ok(())
/// });
/// ```
pub use pyderive_macros::PyReversed;
/// Derive macro generating `__richcmp__` fn that provides Python comparison operations (`==`, `!=`, `<`, `<=`, `>`, and `>=`).
///
/// The implementation requires [`PartialEq`] and [`PartialOrd`] impl.
///
/// <section class="warning">
/// PyO3 supports <code>#[pyclass(ord)]</code> since 0.22, it is recommended to use it.
/// </section>
///
/// The generated methods return `False` when [`PartialOrd::partial_cmp`] returns [`None`].
///
/// *Note that implementing `__richcmp__` will cause Python not to generate
/// a default `__hash__` implementation, so consider implementing `__hash__`
/// when implementing `__richcmp__`.*
///
/// # Expansion
///
/// This implements, for example;
///
/// ```
/// # use std::cmp::Ordering;
/// # use pyo3::prelude::*;
/// # use pyo3::pyclass::CompareOp;
/// # #[pyclass]
/// # #[derive(PartialOrd, PartialEq)]
/// # struct PyClass {}
/// #[pymethods]
/// impl PyClass {
///     pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
///         match op {
///             CompareOp::Eq => self.eq(other),
///             CompareOp::Ne => self.ne(other),
///             CompareOp::Lt => matches!(self.partial_cmp(other), Some(Ordering::Less)),
///             CompareOp::Le => matches!(self.partial_cmp(other), Some(Ordering::Less | Ordering::Equal)),
///             CompareOp::Gt => matches!(self.partial_cmp(other), Some(Ordering::Greater)),
///             CompareOp::Ge => matches!(self.partial_cmp(other), Some(Ordering::Greater | Ordering::Equal))
///         }
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
/// #[derive(PyRichCmp)]
/// #[pyclass]
/// #[derive(PartialOrd, PartialEq)]
/// struct PyClass {
///     field: f64,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass { field: 0.0 })?;
///     let b = Py::new(py, PyClass { field: 1.0 })?;
///     let c = Py::new(py, PyClass { field: f64::NAN })?;
///
///     py_run!(py, a b, "assert a == a");
///     py_run!(py, a b, "assert a != b");
///     py_run!(py, a b, "assert a < b");
///     py_run!(py, a b, "assert a <= b");
///     py_run!(py, a b, "assert not a > b");
///     py_run!(py, a b, "assert not a >= b");
///     py_run!(py, c, "assert not c < c");
///
///     let test = "
/// try:
///     a < 1
/// except TypeError:
///     pass
/// else:
///     raise AssertionError
/// ";
///     py_run!(py, a, test);
///
///     Ok(())
/// });
/// ```
pub use pyderive_macros::PyRichCmp;
/// Derive macro generating a [`__str__()`][__str__] fn/Python method.
///
/// It returns the string that contains `get` and `set` fields as default,
/// in the order of declaration.
///
/// If the filed is marked by `#[pyderive(str=true)]` attribute,
/// the field is included in the string that `__str__()` returns;
/// if `#[pyderive(str=false)]`, it isn't.
///
/// - It should place `#[derive(PyStr)]` before `#[pyclass]`.
/// - It requires [`IntoPyObject`][pyo3_IntoPyObject] trait for fields.
/// - recursively calls `str()` like a dataclass.
///
/// [pyo3_IntoPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.IntoPyObject.html
/// [pyo3_pyclass]: https://docs.rs/pyo3/latest/pyo3/attr.pyclass.html
/// [__str__]: https://docs.python.org/reference/datamodel.html#object.__str__
/// [str]: https://docs.python.org/library/functions.html#str
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
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
///     excluded: String,
/// }
///
/// Python::with_gil(|py| -> PyResult<()> {
///     let a = Py::new(py, PyClass {
///         string: "s".to_string(),
///         integer: 1,
///         float: 1.0,
///         tuple: ("s".to_string(), 1, 1.0),
///         option: None,
///         excluded: "excluded".to_string(),
///     })?;
///
///     py_run!(py, a, r#"assert str(a) == "PyClass(string='s', integer=1, float=1.0, tuple=('s', 1, 1.0), option=None)""#);
///
///     Ok(())
/// });
/// ```
pub use pyderive_macros::PyStr;
/// Derive macro generating an impl of the trait [`ToPyObject`][pyo3_ToPyObject] by trait [`IntoPy<PyObject>`][pyo3_IntoPy].
///
/// - It requires [`Clone`] trait.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
/// [pyo3_IntoPy]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.IntoPy.html
///
/// # Expansion
///
/// This implements, for example;
///
/// ```
/// # use pyo3::prelude::*;
/// # #[pyclass]
/// # #[derive(Clone)]
/// # struct PyClass {}
/// impl ToPyObject for PyClass {
///     fn to_object(&self, py: Python<'_>) -> PyObject {
///         self.clone().into_py(py)
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use pyo3::{prelude::*, py_run};
/// use pyderive::*;
///
/// #[derive(PyNew, PyRepr)]
/// #[pyclass(get_all)]
/// struct PyClass {
///     child: Py<Child>,
/// }
///
/// // PyRepr requires ToPyObject trait for child pyclass
/// #[derive(PyNew, PyRepr, ToPyObject)]
/// #[pyclass(get_all)]
/// #[derive(Clone)]
/// struct Child {
///     field: i64,
/// }
///
/// let test = r#"
/// a = PyClass(Child(10))
///
/// assert repr(a) == "PyClass(child=Child(field=10))"
/// "#;
///
/// Python::with_gil(|py| {
///     let PyClass = py.get_type::<PyClass>();
///     let Child = py.get_type::<Child>();
///
///     py_run!(py, PyClass Child, test)
/// });
/// ```
#[deprecated(
    since = "0.8.0",
    note = "`ToPyObject` is deprecated since PyO3 0.23.0, see PyO3 documentation"
)]
pub use pyderive_macros::ToPyObject;

/// Derive macro generating an impl of bitwise op methods/fns base on [std::ops] traits.
///
/// This derives;
///
/// | Python method                | Required Trait                    |
/// |------------------------------|-----------------------------------|
/// | [`__invert__()`][__invert__] | `Not for &Class`                  |
/// | [`__and__()`][__and__]       | `BitAnd<&Class> for &Class`       |
/// | [`__or__()`][__or__]         | `BitOr<&Class> for &Class`        |
/// | [`__xor__()`][__xor__]       | `BitXor<&Class> for &Class`       |
/// | [`__iand__()`][__iand__]     | `BitAndAssign<&Class> for &Class` |
/// | [`__ior__()`][__ior__]       | `BitOrAssign<&Class> for &Class`  |
/// | [`__ixor__()`][__ixor__]     | `BitXorAssign<&Class> for &Class` |
///
/// [__invert__]: https://docs.python.org/3/reference/datamodel.html#object.__invert__
/// [__and__]: https://docs.python.org/3/reference/datamodel.html#object.__and__
/// [__or__]: https://docs.python.org/3/reference/datamodel.html#object.__or__
/// [__xor__]: https://docs.python.org/3/reference/datamodel.html#object.__xor__
/// [__iand__]: https://docs.python.org/3/reference/datamodel.html#object.__iand__
/// [__ior__]: https://docs.python.org/3/reference/datamodel.html#object.__ior__
/// [__ixor__]: https://docs.python.org/3/reference/datamodel.html#object.__ixor__
pub use pyderive_macros::PyBitwise;
/// Derive macro generating an impl of numeric op methods/fns base on [std::ops] traits.
///
/// This derives;
///
/// | Python method                    | Required Trait                          |
/// |----------------------------------|-----------------------------------------|
/// | [`__pos__()`][__pos__]           | --                                      |
/// | [`__neg__()`][__neg__]           | `Neg<&Class> for &Class`                |
/// | [`__add__()`][__add__]           | `Add<&Class> for &Class`                |
/// | [`__sub__()`][__sub__]           | `Sub<&Class> for &Class`                |
/// | [`__mul__()`][__mul__]           | `Mul<&Class> for &Class`                |
/// | [`__truediv__()`][__truediv__]   | `Div<&Class> for &Class`                |
/// | [`__mod__()`][__mod__]           | `Rem<&Class> for &Class`                |
/// | [`__iadd__()`][__iadd__]         | `AddAssign<&Class> for &Class`          |
/// | [`__isub__()`][__isub__]         | `SubAssign<&Class> for &Class`          |
/// | [`__imul__()`][__imul__]         | `MulAssign<&Class> for &Class`          |
/// | [`__itruediv__()`][__itruediv__] | `DivAssign<&Class> for &Class`          |
/// | [`__imod__()`][__imod__]         | `RemAssign<&Class> for &Class`          |
/// | [`__divmod__()`][__divmod__]     | Same as `__truediv__()` and `__mod__()` |
///
/// [__pos__]: https://docs.python.org/3/reference/datamodel.html#object.__pos__
/// [__neg__]: https://docs.python.org/3/reference/datamodel.html#object.__neg__
/// [__add__]: https://docs.python.org/3/reference/datamodel.html#object.__add__
/// [__sub__]: https://docs.python.org/3/reference/datamodel.html#object.__sub__
/// [__mul__]: https://docs.python.org/3/reference/datamodel.html#object.__mul__
/// [__truediv__]: https://docs.python.org/3/reference/datamodel.html#object.__truediv__
/// [__mod__]: https://docs.python.org/3/reference/datamodel.html#object.__mod__
/// [__iadd__]: https://docs.python.org/3/reference/datamodel.html#object.__iadd__
/// [__isub__]: https://docs.python.org/3/reference/datamodel.html#object.__isub__
/// [__imul__]: https://docs.python.org/3/reference/datamodel.html#object.__imul__
/// [__itruediv__]: https://docs.python.org/3/reference/datamodel.html#object.__itruediv__
/// [__imod__]: https://docs.python.org/3/reference/datamodel.html#object.__imod__
/// [__divmod__]: https://docs.python.org/3/reference/datamodel.html#object.__divmod__
pub use pyderive_macros::PyNumeric;
