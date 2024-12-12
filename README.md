# pyderive

[![Crates.io Version](https://img.shields.io/crates/v/pyderive?logo=rust)](https://crates.io/crates/pyderive)
[![Crates.io MSRV](https://img.shields.io/crates/msrv/pyderive?logo=rust)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/paqira/pyderive/ci.yaml?logo=GitHub)
[![docs.rs](https://img.shields.io/docsrs/pyderive?logo=rust)](https://docs.rs/pyderive/)
![Crates.io License](https://img.shields.io/crates/l/pyderive)

`pyderive` provides derive macros for Python spacial methods and a class attributes for [PyO3].

[PyO3]: https://github.com/PyO3/pyo3

```rust
// Enable `multiple-pymethods` feature of PyO3
use pyo3::prelude::*;
use pyderive::*;

// Place #[derive(PyNew, ...)] before #[pyclass]
#[derive(PyNew, PyMatchArgs, PyRepr, PyEq)]
#[pyclass(get_all)]
#[derive(PartialEq, Hash)]
struct MyClass {
    string: String,
    integer: i64,
    option: Option<String>
}
```

```python
from rust_module import MyClass

# Derives __new__()
m = MyClass("a", 1, None)

# Derives __match_args__ (supports Pattern Matching by positional arguments)
match m:
    case MyClass(a, b, c):
        assert a == "a"
        assert b == 1
        assert c is None
    case _:
        raise AssertionError

# Derives __repr__(), calls Python repr() recursively
assert str(m) == "MyClass(string='a', integer=1, option=None)"
assert repr(m) == "MyClass(string='a', integer=1, option=None)"

# Derives __eq__() that depends on PartialEq trait
assert m == MyClass("a", 1, None)
```

This provides deriving following special methods and attributes;

| Derive Macro        | Python Method/Attribute                                |
|---------------------|--------------------------------------------------------|
| `PyNew`             | `__new__()`                                            |
| `PyMatchArgs`       | `__match_args__`                                       |
| `PyRepr`            | `__repr__()` (recursively calls `repr()`)              |
| `PyStr`             | `__str__()`  (recursively calls `str()`)               |
| `PyEq`              | `__eq__()` and `__ne__()`                              |
| `PyOrd`             | `__lt__()`, `__le__()`, `__gt__()` and `__ge__()`      |
| `PyRichCmp`         | `==`, `!=`, `>`, `>=`, `<` and `<=` by `__richcmp__()` |
| `PyIter`            | `__iter__()`                                           |
| `PyReversed`        | `__reversed__()`                                       |
| `PyLen`             | `__len__()`                                            |
| `PyDataclassFields` | `__dataclass_fields__`                                 |
| `PyNumeric`         | Numeric op methods (`__add__()` etc.)                  |
| `PyBitwise`         | Bitwise op methods (`__and__()` etc.)                  |

The field attributes `#[pyderive(..)]` is used to customize the implementation,
like [`dataclasses.field()`][dataclasses-field] of Python.

[dataclasses-field]: https://docs.python.org/3/library/dataclasses.html#dataclasses.field

Module `pyderive::ops` and `pyderive::convert` provides
derive macros that implement individual method that enumerating numeric type (`__add__()` etc.) and
called by builtin functions (`__int__()` etc.).

It requires to enable `multiple-pymethods` feature of PyO3 because this may produce multiple `#[pymethods]`.

## License

MIT or Apache-2.0
