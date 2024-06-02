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
//! #[derive(PyNew, PyMatchArgs, PyRepr, PyEq, PyHash)]
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
//! # Derives __repr__()
//! assert str(m) == "builtins.MyClass(string='a', integer=1, option=None)"
//! assert repr(m) == "builtins.MyClass(string='a', integer=1, option=None)"
//!
//! # Derives __eq__() based on PartialEq/Eq trait
//! assert m == MyClass("a", 1, None)
//!
//! # Derives __hash__() based on Hash trait
//! assert hash(m) == 3289857268557676066
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
//! Module [`pyderive::ops`](mod@ops) and [`pyderive::convert`](mod@convert) provides
//! derive macros that implement method that enumerating numeric type (`__add__` etc.) and
//! called by builtin functions (`__int__` etc.).
//!
//! In addition, this provides a helper derive macro that generates an impl of [`ToPyObject`][pyo3_ToPyObject] trait
//! that required by [`PyRepr`], [`PyStr`], [`PyIter`] and [`PyDataclassFields`] derive macros.
//!
//! | Derive Macro   | Impl                                                                                                           |
//! | -------------- | -------------------------------------------------------------------------------------------------------------- |
//! | [`ToPyObject`] | [`ToPyObject`][pyo3_ToPyObject] trait by [`IntoPy<PyObject>`][pyo3_IntoPy] trait for [`pyclass`][pyo3_pyclass] |
//!
//! [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
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
/// - It requires [`ToPyObject`][pyo3_ToPyObject] trait
///   for child [`pyclass`][pyo3_pyclass]es.
///
/// This does not generate other fn/method,
/// use [`PyNew`] etc. to implement `__new__()` etc.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
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
/// The implementation is based on [`PartialEq`]/[`Eq`] trait.
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
/// - It requires [`ToPyObject`][pyo3_ToPyObject] trait
///   for child [`pyclass`][pyo3_pyclass]es.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
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
///         let PyClass = py.get_type_bound::<PyClass>();
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
///     let PyClass = py.get_type_bound::<PyClass>();
///
///     py_run!(py, PyClass, test)
/// });
/// ```
pub use pyderive_macros::PyNew;
/// Derive macro generating [`__lt__()`][__lt__], [`__le__()`][__le__], [`__gt__()`][__gt__] and [`__ge__()`][__ge__] fn/Python methods.
///
/// The implementation is based on [`PartialOrd`]/[`Ord`] trait.
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
///     pub fn __lt__(&self, other: &Self) -> PyResult<bool> {
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
/// - It requires [`ToPyObject`][pyo3_ToPyObject] trait
///   for child [`pyclass`][pyo3_pyclass]es.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
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
///     py_run!(py, a, r#"assert repr(a) == "builtins.PyClass(string='s', integer=1, float=1.0, tuple=('s', 1, 1.0), option=None)""#);
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
/// - It requires [`ToPyObject`][pyo3_ToPyObject] trait
///   for child [`pyclass`][pyo3_pyclass]es.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
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
/// - It requires [`ToPyObject`][pyo3_ToPyObject] trait
///   for child [`pyclass`][pyo3_pyclass]es.
///
/// [pyo3_ToPyObject]: https://docs.rs/pyo3/latest/pyo3/conversion/trait.ToPyObject.html
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
///     py_run!(py, a, r#"assert str(a) == "builtins.PyClass(string='s', integer=1, float=1.0, tuple=('s', 1, 1.0), option=None)""#);
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
///     child: Child,
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
/// assert repr(a) == "builtins.PyClass(child=builtins.Child(field=10))"
/// "#;
///
/// Python::with_gil(|py| {
///     let PyClass = py.get_type_bound::<PyClass>();
///     let Child = py.get_type_bound::<Child>();
///
///     py_run!(py, PyClass Child, test)
/// });
/// ```
pub use pyderive_macros::ToPyObject;

/// Provides derive macros that implements enumeration of numeric type.
pub mod ops {
    /// Derive macro generating an impl of [`__add__`][py] method by [`Add`][std::ops::Add] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Add;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Add for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn add(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __add__(&self, other: &Self) -> Self {
    ///         Add::add(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Add;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyAdd;
    ///
    /// #[derive(PyNew, PyAdd)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Add for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn add(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field + rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(1) + PyClass(2)
    /// assert actual.field == 3
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__add__
    pub use pyderive_macros::PyAdd;
    /// Derive macro generating an impl of [`__iadd__`][py] method by [`AddAssign`][std::ops::AddAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::AddAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl AddAssign<&Self> for PyClass {
    /// #    fn add_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __iadd__(&mut self, other: &Self) {
    ///         AddAssign::add_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::AddAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyAddAssign;
    ///
    /// #[derive(PyNew, PyAddAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl AddAssign<&Self> for PyClass {
    ///     fn add_assign(&mut self, rhs: &Self) {
    ///         self.field += rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(1)
    /// actual += PyClass(2)
    /// assert actual.field == 3
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__iadd__
    pub use pyderive_macros::PyAddAssign;
    /// Derive macro generating an impl of [`__and__`][py] method by [`BitAnd`][std::ops::BitAnd] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::BitAnd;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl BitAnd for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn bitand(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __and__(&self, other: &Self) -> Self {
    ///         BitAnd::bitand(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::BitAnd;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyAnd;
    ///
    /// #[derive(PyNew, PyAnd)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl BitAnd for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn bitand(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field & rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(9) & PyClass(5)
    /// assert actual.field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__and__
    pub use pyderive_macros::PyAnd;
    /// Derive macro generating an impl of [`__iand__`][py] method by [`BitAndAssign`][std::ops::BitAndAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::BitAndAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl BitAndAssign<&Self> for PyClass {
    /// #    fn bitand_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __iand__(&mut self, other: &Self) {
    ///         BitAndAssign::bitand_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::BitAndAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyAndAssign;
    ///
    /// #[derive(PyNew, PyAndAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl BitAndAssign<&Self> for PyClass {
    ///     fn bitand_assign(&mut self, rhs: &Self) {
    ///         self.field &= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(9)
    /// actual &= PyClass(5)
    /// assert actual.field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__iand__
    pub use pyderive_macros::PyAndAssign;
    /// Derive macro generating an impl of [`__divmod__`][py] method by [`Div`][std::ops::Div] and [`Rem`][std::ops::Rem] traits.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::{Div, Rem};
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Div for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn div(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// # impl Rem for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn rem(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __divmod__(&self, other: &Self) -> (Self, Self) {
    ///         (Div::div(self, other), Rem::rem(self, other))
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::{Div, Rem};
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyDivMod;
    ///
    /// #[derive(PyNew, PyDivMod)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Div for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn div(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field / rhs.field }
    ///     }
    /// }
    ///
    /// impl Rem for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn rem(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field % rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = divmod(PyClass(7), PyClass(2))
    /// assert actual[0].field == 3
    /// assert actual[1].field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__divmod__
    pub use pyderive_macros::PyDivMod;
    /// Derive macro generating an impl of [`__floordiv__`][py] method by [`Div`][std::ops::Div] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Div;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Div for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn div(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __floordiv__(&self, other: &Self) -> Self {
    ///         Div::div(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Div;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyFloorDiv;
    ///
    /// #[derive(PyNew, PyFloorDiv)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Div for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn div(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field / rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7) // PyClass(2)
    /// assert actual.field == 3
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__floordiv__
    pub use pyderive_macros::PyFloorDiv;
    /// Derive macro generating an impl of [`__ifloordiv__`][py] method by [`DivAssign`][std::ops::DivAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::DivAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl DivAssign<&Self> for PyClass {
    /// #    fn div_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __ifloordiv__(&mut self, other: &Self) {
    ///         DivAssign::div_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::DivAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyFloorDivAssign;
    ///
    /// #[derive(PyNew, PyFloorDivAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl DivAssign<&Self> for PyClass {
    ///     fn div_assign(&mut self, rhs: &Self) {
    ///         self.field /= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7)
    /// actual //= PyClass(2)
    /// assert actual.field == 3
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__ifloordiv__
    pub use pyderive_macros::PyFloorDivAssign;
    /// Derive macro generating an impl of [`__invert__`][py] method by [`Not`][std::ops::Not] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Not;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Not for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn not(self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __invert__(&self) -> Self {
    ///         Not::not(self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Not;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyInvert;
    ///
    /// #[derive(PyNew, PyInvert)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Not for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn not(self) -> Self::Output {
    ///         PyClass { field: !self.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = ~PyClass(1)
    /// assert actual.field == -2
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__invert__
    pub use pyderive_macros::PyInvert;
    /// Derive macro generating an impl of [`__lshift__`][py] method by [`Shl`][std::ops::Shl] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Shl;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Shl for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn shl(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __lshift__(&self, other: &Self) -> Self {
    ///         Shl::shl(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Shl;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyLeftShift;
    ///
    /// #[derive(PyNew, PyLeftShift)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Shl for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn shl(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field << rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(1) << PyClass(2)
    /// assert actual.field == 4
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__lshift__
    pub use pyderive_macros::PyLeftShift;
    /// Derive macro generating an impl of [`__ilshift__`][py] method by [`ShlAssign`][std::ops::ShlAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::ShlAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl ShlAssign<&Self> for PyClass {
    /// #    fn shl_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __ilshift__(&mut self, other: &Self) {
    ///         ShlAssign::shl_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::ShlAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyLeftShiftAssign;
    ///
    /// #[derive(PyNew, PyLeftShiftAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl ShlAssign<&Self> for PyClass {
    ///     fn shl_assign(&mut self, rhs: &Self) {
    ///         self.field <<= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(1)
    /// actual <<= PyClass(2)
    /// assert actual.field == 4
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__ilshift__
    pub use pyderive_macros::PyLeftShiftAssign;
    /// Derive macro generating an impl of [`__matmul__`][py] method by [`Mul`][std::ops::Mul] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Mul;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Mul for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn mul(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __matmul__(&self, other: &Self) -> Self {
    ///         Mul::mul(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Mul;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyMatMul;
    ///
    /// #[derive(PyNew, PyMatMul)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Mul for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn mul(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field * rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(2) @ PyClass(3)
    /// assert actual.field == 6
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__matmul__
    pub use pyderive_macros::PyMatMul;
    /// Derive macro generating an impl of [`__imatmul__`][py] method by [`MulAssign`][std::ops::MulAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::MulAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl MulAssign<&Self> for PyClass {
    /// #    fn mul_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __imatmul__(&mut self, other: &Self) {
    ///         MulAssign::mul_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::MulAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyMatMulAssign;
    ///
    /// #[derive(PyNew, PyMatMulAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl MulAssign<&Self> for PyClass {
    ///     fn mul_assign(&mut self, rhs: &Self) {
    ///         self.field *= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(2)
    /// actual @= PyClass(3)
    /// assert actual.field == 6
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__imatmul__
    pub use pyderive_macros::PyMatMulAssign;
    /// Derive macro generating an impl of [`__mod__`][py] method by [`Rem`][std::ops::Rem] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Rem;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Rem for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn rem(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __mod__(&self, other: &Self) -> Self {
    ///         Rem::rem(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Rem;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyMod;
    ///
    /// #[derive(PyNew, PyMod)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Rem for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn rem(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field % rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7) % PyClass(2)
    /// assert actual.field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__mod__
    pub use pyderive_macros::PyMod;
    /// Derive macro generating an impl of [`__imod__`][py] method by [`RemAssign`][std::ops::RemAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::RemAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl RemAssign<&Self> for PyClass {
    /// #    fn rem_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __imod__(&mut self, other: &Self) {
    ///         RemAssign::rem_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::RemAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyModAssign;
    ///
    /// #[derive(PyNew, PyModAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl RemAssign<&Self> for PyClass {
    ///     fn rem_assign(&mut self, rhs: &Self) {
    ///         self.field %= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7)
    /// actual %= PyClass(2)
    /// assert actual.field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__imod__
    pub use pyderive_macros::PyModAssign;
    /// Derive macro generating an impl of [`__mul__`][py] method by [`Mul`][std::ops::Mul] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Mul;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Mul for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn mul(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __mul__(&self, other: &Self) -> Self {
    ///         Mul::mul(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Mul;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyMul;
    ///
    /// #[derive(PyNew, PyMul)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Mul for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn mul(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field * rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(2) * PyClass(3)
    /// assert actual.field == 6
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__mul__
    pub use pyderive_macros::PyMul;
    /// Derive macro generating an impl of [`__imul__`][py] method by [`MulAssign`][std::ops::MulAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::MulAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl MulAssign<&Self> for PyClass {
    /// #    fn mul_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __imul__(&mut self, other: &Self) {
    ///         MulAssign::mul_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::MulAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyMulAssign;
    ///
    /// #[derive(PyNew, PyMulAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl MulAssign<&Self> for PyClass {
    ///     fn mul_assign(&mut self, rhs: &Self) {
    ///         self.field *= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(2)
    /// actual *= PyClass(3)
    /// assert actual.field == 6
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__imul__
    pub use pyderive_macros::PyMulAssign;
    /// Derive macro generating an impl of [`__neg__`][py] method by [`Neg`][std::ops::Neg] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Neg;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Neg for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn neg(self) -> Self::Output { PyClass {} }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __neg__(&self) -> Self {
    ///         Neg::neg(self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Neg;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyNeg;
    ///
    /// #[derive(PyNew, PyNeg, Clone)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Neg for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn neg(self) -> Self::Output {
    ///         PyClass { field: Neg::neg(self.field) }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = -PyClass(1)
    /// assert actual.field == -1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     if py.version_info() >= (3, 10) {
    ///         let PyClass = py.get_type_bound::<PyClass>();
    ///
    ///         py_run!(py, PyClass, test)
    ///     }
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__neg__
    pub use pyderive_macros::PyNeg;
    /// Derive macro generating an impl of [`__or__`][py] method by [`BitOr`][std::ops::BitOr] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::BitOr;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl BitOr for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn bitor(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __or__(&self, other: &Self) -> Self {
    ///         BitOr::bitor(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::BitOr;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyOr;
    ///
    /// #[derive(PyNew, PyOr)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl BitOr for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn bitor(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field | rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(5) | PyClass(3)
    /// assert actual.field == 7
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__or__
    pub use pyderive_macros::PyOr;
    /// Derive macro generating an impl of [`__ior__`][py] method by [`BitOrAssign`][std::ops::BitOrAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::BitOrAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl BitOrAssign<&Self> for PyClass {
    /// #    fn bitor_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __ior__(&mut self, other: &Self) {
    ///         BitOrAssign::bitor_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::BitOrAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyOrAssign;
    ///
    /// #[derive(PyNew, PyOrAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl BitOrAssign<&Self> for PyClass {
    ///     fn bitor_assign(&mut self, rhs: &Self) {
    ///         self.field |= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(5)
    /// actual |= PyClass(3)
    /// assert actual.field == 7
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__ior__
    pub use pyderive_macros::PyOrAssign;
    /// Derive macro generating an impl of [`__pos__`][py] method (an identity method).
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use pyo3::prelude::*;
    /// # #[derive(Clone)]
    /// # #[pyclass]
    /// # struct PyClass {}
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __pos__(self_: PyRef<'_, Self>) -> PyRef<'_, Self> {
    ///         self_
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
    /// use pyderive::ops::PyPos;
    ///
    /// #[derive(PyNew, PyPos)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// let test = "
    /// actual = +PyClass(1)
    /// assert actual.field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__pos__
    pub use pyderive_macros::PyPos;
    /// Derive macro generating an impl of [`__radd__`][py] method by [`Add`][std::ops::Add] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Add;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Add for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn add(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __radd__(&self, other: &Self) -> Self {
    ///         Add::add(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Add;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedAdd;
    ///
    /// #[derive(PyNew, PyReflectedAdd)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Add for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn add(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field + rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(5) + PyClass(3)
    /// assert actual.field == 8
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__radd__
    pub use pyderive_macros::PyReflectedAdd;
    /// Derive macro generating an impl of [`__rand__`][py] method by [`BitAnd`][std::ops::BitAnd] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::BitAnd;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl BitAnd for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn bitand(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rand__(&self, other: &Self) -> Self {
    ///         BitAnd::bitand(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::BitAnd;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedAnd;
    ///
    /// #[derive(PyNew, PyReflectedAnd)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl BitAnd for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn bitand(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field & rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7) & PyClass(3)
    /// assert actual.field == 3
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rand__
    pub use pyderive_macros::PyReflectedAnd;
    /// Derive macro generating an impl of [`__rdivmod__`][py] method by [`Div`][std::ops::Div] and [`Rem`][std::ops::Rem] traits.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::{Div, Rem};
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Div for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn div(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// # impl Rem for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn rem(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rdivmod__(&self, other: &Self) -> (Self, Self) {
    ///         (Div::div(other, self), Rem::rem(other, self))
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::{Div, Rem};
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedDivMod;
    ///
    /// #[derive(PyNew, PyReflectedDivMod)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Div for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn div(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field / rhs.field }
    ///     }
    /// }
    ///
    /// impl Rem for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn rem(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field % rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = divmod(PyClass(7), PyClass(2))
    /// assert actual[0].field == 3
    /// assert actual[1].field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rdivmod__
    pub use pyderive_macros::PyReflectedDivMod;
    /// Derive macro generating an impl of [`__rfloordiv__`][py] method by [`Div`][std::ops::Div] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Div;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Div for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn div(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rfloordiv__(&self, other: &Self) -> Self {
    ///         Div::div(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Div;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedFloorDiv;
    ///
    /// #[derive(PyNew, PyReflectedFloorDiv)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Div for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn div(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field / rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7) // PyClass(3)
    /// assert actual.field == 2
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rfloordiv__
    pub use pyderive_macros::PyReflectedFloorDiv;
    /// Derive macro generating an impl of [`__rlshift__`][py] method by [`Shl`][std::ops::Shl] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Shl;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Shl for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn shl(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rlshift__(&self, other: &Self) -> Self {
    ///         Shl::shl(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Shl;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedLeftShift;
    ///
    /// #[derive(PyNew, PyReflectedLeftShift)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Shl for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn shl(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field << rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(1) << PyClass(2)
    /// assert actual.field == 4
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rlshift__
    pub use pyderive_macros::PyReflectedLeftShift;
    /// Derive macro generating an impl of [`__rmatmul__`][py] method by [`Mul`][std::ops::Mul] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Mul;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Mul for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn mul(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rmatmul__(&self, other: &Self) -> Self {
    ///         Mul::mul(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Mul;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedMatMul;
    ///
    /// #[derive(PyNew, PyReflectedMatMul)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Mul for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn mul(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field * rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(2) @ PyClass(3)
    /// assert actual.field == 6
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rmatmul__
    pub use pyderive_macros::PyReflectedMatMul;
    /// Derive macro generating an impl of [`__rmod__`][py] method by [`Rem`][std::ops::Rem] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Rem;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Rem for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn rem(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rmod__(&self, other: &Self) -> Self {
    ///         Rem::rem(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Rem;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedMod;
    ///
    /// #[derive(PyNew, PyReflectedMod)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Rem for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn rem(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field % rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7) % PyClass(2)
    /// assert actual.field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rmod__
    pub use pyderive_macros::PyReflectedMod;
    /// Derive macro generating an impl of [`__rmul__`][py] method by [`Mul`][std::ops::Mul] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Mul;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Mul for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn mul(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rmul__(&self, other: &Self) -> Self {
    ///         Mul::mul(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Mul;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedMul;
    ///
    /// #[derive(PyNew, PyReflectedMul)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Mul for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn mul(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field * rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(2) * PyClass(3)
    /// assert actual.field == 6
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rmul__
    pub use pyderive_macros::PyReflectedMul;
    /// Derive macro generating an impl of [`__ror__`][py] method by [`BitOr`][std::ops::BitOr] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::BitOr;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl BitOr for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn bitor(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __ror__(&self, other: &Self) -> Self {
    ///         BitOr::bitor(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::BitOr;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedOr;
    ///
    /// #[derive(PyNew, PyReflectedOr)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl BitOr for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn bitor(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field | rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7) | PyClass(3)
    /// assert actual.field == 7
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__ror__
    pub use pyderive_macros::PyReflectedOr;
    /// Derive macro generating an impl of [`__rrshift__`][py] method by [`Shr`][std::ops::Shr] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Shr;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Shr for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn shr(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rrshift__(&self, other: &Self) -> Self {
    ///         Shr::shr(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Shr;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedRightShift;
    ///
    /// #[derive(PyNew, PyReflectedRightShift)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Shr for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn shr(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field >> rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(8) >> PyClass(2)
    /// assert actual.field == 2
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rrshift__
    pub use pyderive_macros::PyReflectedRightShift;
    /// Derive macro generating an impl of [`__rsub__`][py] method by [`Sub`][std::ops::Sub] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Sub;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Sub for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn sub(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rsub__(&self, other: &Self) -> Self {
    ///         Sub::sub(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Sub;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedSub;
    ///
    /// #[derive(PyNew, PyReflectedSub)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Sub for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn sub(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field - rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(2) - PyClass(1)
    /// assert actual.field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rsub__
    pub use pyderive_macros::PyReflectedSub;
    /// Derive macro generating an impl of [`__rtruediv__`][py] method by [`Div`][std::ops::Div] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Div;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Div for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn div(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rtruediv__(&self, other: &Self) -> Self {
    ///         Div::div(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Div;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyReflectedTrueDiv;
    ///
    /// #[derive(PyNew, PyReflectedTrueDiv)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Div for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn div(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field / rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7) / PyClass(3)
    /// assert actual.field == 2
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rtruediv__
    pub use pyderive_macros::PyReflectedTrueDiv;
    /// Derive macro generating an impl of [`__rxor__`][py] method by [`BitXor`][std::ops::BitXor] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::BitXor;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl BitXor for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn bitxor(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rxor__(&self, other: &Self) -> Self {
    ///         BitXor::bitxor(other, self)
    ///     }
    /// }
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rxor__
    pub use pyderive_macros::PyReflectedXor;
    /// Derive macro generating an impl of [`__rshift__`][py] method by [`Shr`][std::ops::Shr] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Shr;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Shr for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn shr(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __rshift__(&self, other: &Self) -> Self {
    ///         Shr::shr(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Shr;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyRightShift;
    ///
    /// #[derive(PyNew, PyRightShift)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Shr for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn shr(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field >> rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(8) >> PyClass(2)
    /// assert actual.field == 2
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__rshift__
    pub use pyderive_macros::PyRightShift;
    /// Derive macro generating an impl of [`__irshift__`][py] method by [`ShrAssign`][std::ops::ShrAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::ShrAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl ShrAssign<&Self> for PyClass {
    /// #    fn shr_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __irshift__(&mut self, other: &Self) {
    ///         ShrAssign::shr_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::ShrAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyRightShiftAssign;
    ///
    /// #[derive(PyNew, PyRightShiftAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl ShrAssign<&Self> for PyClass {
    ///     fn shr_assign(&mut self, rhs: &Self) {
    ///         self.field >>= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(8)
    /// actual >>= PyClass(2)
    /// assert actual.field == 2
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__irshift__
    pub use pyderive_macros::PyRightShiftAssign;
    /// Derive macro generating an impl of [`__sub__`][py] method by [`Sub`][std::ops::Sub] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Sub;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Sub for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn sub(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __sub__(&self, other: &Self) -> Self {
    ///         Sub::sub(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Sub;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PySub;
    ///
    /// #[derive(PyNew, PySub)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Sub for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn sub(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field - rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(2) - PyClass(1)
    /// assert actual.field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__sub__
    pub use pyderive_macros::PySub;
    /// Derive macro generating an impl of [`__isub__`][py] method by [`SubAssign`][std::ops::SubAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::SubAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl SubAssign<&Self> for PyClass {
    /// #    fn sub_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __isub__(&mut self, other: &Self) {
    ///         SubAssign::sub_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::SubAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PySubAssign;
    ///
    /// #[derive(PyNew, PySubAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl SubAssign<&Self> for PyClass {
    ///     fn sub_assign(&mut self, rhs: &Self) {
    ///         self.field -= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(2)
    /// actual -= PyClass(1)
    /// assert actual.field == 1
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__isub__
    pub use pyderive_macros::PySubAssign;
    /// Derive macro generating an impl of [`__truediv__`][py] method by [`Div`][std::ops::Div] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::Div;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl Div for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn div(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __truediv__(&self, other: &Self) -> Self {
    ///         Div::div(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Div;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyTrueDiv;
    ///
    /// #[derive(PyNew, PyTrueDiv)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl Div for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn div(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field / rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7) / PyClass(2)
    /// assert actual.field == 3
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__truediv__
    pub use pyderive_macros::PyTrueDiv;
    /// Derive macro generating an impl of [`__itruediv__`][py] method by [`Div`][std::ops::Div] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::DivAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl DivAssign<&Self> for PyClass {
    /// #    fn div_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __itruediv__(&mut self, other: &Self) {
    ///         DivAssign::div_assign(self, other);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::DivAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyTrueDivAssign;
    ///
    /// #[derive(PyNew, PyTrueDivAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl DivAssign<&Self> for PyClass {
    ///     fn div_assign(&mut self, rhs: &Self) {
    ///         self.field /= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7)
    /// actual /= PyClass(2)
    /// assert actual.field == 3
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__itruediv__
    pub use pyderive_macros::PyTrueDivAssign;
    /// Derive macro generating an impl of [`__xor__`][py] method by [`BitXor`][std::ops::BitXor] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::BitXor;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl BitXor for &PyClass {
    /// #    type Output = PyClass;
    /// #    fn bitxor(self, rhs: Self) -> Self::Output { self.clone() }
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __xor__(&self, other: &Self) -> Self {
    ///         BitXor::bitxor(self, other)
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::BitXor;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyXor;
    ///
    /// #[derive(PyNew, PyXor)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl BitXor for &PyClass {
    ///     type Output = PyClass;
    ///
    ///     fn bitxor(self, rhs: Self) -> Self::Output {
    ///         PyClass { field: self.field ^ rhs.field }
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7) ^ PyClass(3)
    /// assert actual.field == 4
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__xor__
    pub use pyderive_macros::PyXor;
    /// Derive macro generating an impl of [`__ixor__`][py] method by [`BitXorAssign`][std::ops::BitXorAssign] trait.
    ///
    /// # Expansion
    ///
    /// This implements:
    ///
    /// ```
    /// # use std::ops::BitXorAssign;
    /// # use pyo3::prelude::*;
    /// # #[pyclass]
    /// # #[derive(Clone)]
    /// # struct PyClass {}
    /// # impl BitXorAssign<&Self> for PyClass {
    /// #    fn bitxor_assign(&mut self, rhs: &Self) {}
    /// # }
    /// #[pymethods]
    /// impl PyClass {
    ///     fn __ixor__(&mut self, other: &Self) {
    ///         BitXorAssign::bitxor_assign(self, other);
    ///     }
    /// }
    /// ```
    /// # Example
    ///
    /// ```
    /// use std::ops::BitXorAssign;
    ///
    /// use pyo3::{prelude::*, py_run};
    ///
    /// use pyderive::PyNew;
    /// use pyderive::ops::PyXorAssign;
    ///
    /// #[derive(PyNew, PyXorAssign)]
    /// #[pyclass(get_all)]
    /// struct PyClass {
    ///     field: i64
    /// }
    ///
    /// impl BitXorAssign<&Self> for PyClass {
    ///     fn bitxor_assign(&mut self, rhs: &Self) {
    ///         self.field ^= rhs.field;
    ///     }
    /// }
    ///
    /// let test = "
    /// actual = PyClass(7)
    /// actual ^= PyClass(3)
    /// assert actual.field == 4
    /// ";
    ///
    /// Python::with_gil(|py| {
    ///     let PyClass = py.get_type_bound::<PyClass>();
    ///     py_run!(py, PyClass, test)
    /// });
    /// ```
    ///
    /// [py]: https://docs.python.org/3/reference/datamodel.html#object.__ixor__
    pub use pyderive_macros::PyXorAssign;
}

/// Provides derive macros that implements conversion by built-in functions.
pub mod convert {
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
    ///         Into::<f64>::into(self)
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
}
