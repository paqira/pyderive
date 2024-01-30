# pyderive

`pyderive` provides derive macros for Python spacial methods and a class attribute for PyO3.

```rust
// Enable `multiple-pymethods` feature of PyO3
use pyderive::*;

// Place #[derive(PyInit, ...)] before #[pyclass]
#[derive(PyInit, PyMatchArgs, PyRepr, PyEq, PyHash)]
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

# Derives __init__() (technically __new__())
m = MyClass("a", 1, None)

# Derives __match_args__ (supports Pattern Matching by positional arg)
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
assert m == m

# Derives __hash__() based on Hash trait
assert hash(m) == 3289857268557676066
```

This provides deriving following special methods and attribute;

| Derive Macro  | Python Method/Attribute                   |
| ------------- | ----------------------------------------- |
| `PyInit`      | `__init__()` (`__new__()` precisely)        |
| `PyMatchArgs` | `__match_args__`                          |
| `PyRepr`      | `__repr__()`                                |
| `PyStr`       | `__str__()`                                 |
| `PyIter`      | `__iter__()`                                |
| `PyLen`       | `__len__()`                                 |
| `PyEq`        | `__eq__()`                                  |
| `PyOrd`     | `__lt__()`, `__le__()`, `__gt__()` and `__ge__()` |
| `PyHash`      | `__hash__()`                                |

The field attributes `#[pyderive(..)]` is used to customize the implementation,
like `dataclasses.field()` of Python.

It requires to enable `multiple-pymethods` feature of PyO3 because this may produce multiple `#[pymethods]`.

## License

MIT or Apache-2.0