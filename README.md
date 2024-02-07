# pyderive

`pyderive` provides derive macros for Python spacial methods and a class attribute for [PyO3].

[PyO3]: https://github.com/PyO3/pyo3

```rust
// Enable `multiple-pymethods` feature of PyO3
use pyo3::prelude::*;
use pyderive::*;

// Place #[derive(PyNew, ...)] before #[pyclass]
#[derive(PyNew, PyMatchArgs, PyRepr, PyEq, PyHash)]
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

# Derives __repr__()
assert str(m) == "MyClass(string='a', integer=1, option=None)"
assert repr(m) == "MyClass(string='a', integer=1, option=None)"

# Derives __eq__() based on PartialEq/Eq trait
assert m == MyClass("a", 1, None)

# Derives __hash__() based on Hash trait
assert hash(m) == 3289857268557676066
```

This provides deriving following special methods and attributes;

| Derive Macro        | Python Method/Attribute                           |
| ------------------- | ------------------------------------------------- |
| `PyNew`             | `__new__()`                                       |
| `PyMatchArgs`       | `__match_args__`                                  |
| `PyRepr`            | `__repr__()`                                      |
| `PyStr`             | `__str__()`                                       |
| `PyEq`              | `__eq__()` and `__ne__()`                         |
| `PyOrd`             | `__lt__()`, `__le__()`, `__gt__()` and `__ge__()` |
| `PyHash`            | `__hash__()`                                      |
| `PyIter`            | `__iter__()`                                      |
| `PyReversed`        | `__reversed__()`                                  |
| `PyLen`             | `__len__()`                                       |
| `PyDataclassFields` | `__dataclass_fields__`                            |
| `PyAnnotations`     | `__annotations__`                                 |

In addition, this provides a helper derive macro:

| Derive Macro | Impl                                                         |
| ------------ | ------------------------------------------------------------ |
| `ToPyObject` | `ToPyObject` trait by `IntoPy<PyObject>` trait for `pyclass` |

The field attributes `#[pyderive(..)]` is used to customize the implementation,
like [`dataclasses.field()`][dataclasses-field] of Python.

[dataclasses-field]: https://docs.python.org/3/library/dataclasses.html#dataclasses.field

It requires to enable `multiple-pymethods` feature of PyO3 because this may produce multiple `#[pymethods]`.

## License

MIT or Apache-2.0