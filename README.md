# pyderive

```rust
// Enable `multiple-pymethods` feature of pyo3
use pyderive::*;

// Put #[derive(PyInit, ...)] before #[pyclass] to read its attr.
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

# Derives __init__ (technically __new__)
m = MyClass("a", 1, None)
# Derives __match_args__
match m:
    case MyClass(a, b, c):
        assert a == "a"
        assert b == 1
        assert c is None
    case _:
        raise AssertionError
# Derives __repr__
assert repr(m) == "MyClass(string='a', integer=1, option=None)"
# Derives __eq__ based on PartialEq/Eq trait
assert m == m
# Derives __hash__ based on Hash trait
assert hash(m) == 3289857268557676066
```

`pyderive` provides derive macros of
Python special methods and a class attribute.

It requires to enable `multiple-pymethods` feature of pyo3 because this derives multiple `#[pymethods]`.

This provides deriving following special methods and attribute;

1. `PyRepr`/`PyStr`: prints all `get` and `set` fileds.
2. `PyIter`: returns iterator of all `get` fields
3. `PyLen`: returns number of `get` fields (compile-time constant)
4. `PyInit`: a constructor with all fields (technically `__new__`)
5. `PyMatchArgs`: supports pattern matching by positional attr. with all `get` fields
6. `PyEq`: besed on `PartialEq`/`Eq` trait
7. `PyOrder` for `__lt__`, `__le__`, `__gt__` and `__ge__`: besed on `PartialOrd`/`Ord` trait
8. `PyHash`: based on `Hash` trait
   - *Note that implementing any of `__eq__`, `__lt__`, `__le__`, `__gt__` and `__ge__` methods will cause Python not to generate a default `__hash__` implementation, so consider also implementing `__hash__`.*

For example,

```rust
use pyderive::*;

#[derive(PyInit, PyMatchArgs, PyRepr)]
#[pyclass(name="RenamedClass", name="camelCase")]
#[derive]
struct MyClass {
    #[pyo3(get, name="renamed_field")]
    str_field: String,
    #[pyo3(set)]
    int_field: i64,
    opt_field: Option<String>
}
```
```python
from rust_module import RenamedClass

# Renames arg names
m = RenamedClass(renamed_field="a", intField=1, opt_field=None)
# Uses get field only
match m:
    case RenamedClass(a):
        assert a == "a"
    # RenamedClass(a, b) and RenamedClass(a, b, c) throw
    # TypeError: RenamedClass() accepts 1 positional sub-patterns (2 (or 3) given)
    case _:
        raise AssertionError
# Prints get/set field only
assert repr(m) == "RenamedClass(renamed_field='a', intField=1)"
```

## License

MIT or Apache-2.0