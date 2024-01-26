# derive_pysp

```rust
// Enable `multiple-pymethods` feature of pyo3
use derive_pysp::*;

// Put #[derive(__init__, ...)] before #[pyclass] to read its attr.
#[derive(__init__, __match_args__, __repr__, __eq__, __hash__)]
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

`derive-pysp` provides derive macros of
Python special methods and a class attribute.

It requires to enable `multiple-pymethods` feature of pyo3 because this derives multiple `#[pymethods]`.

This provides deriving following special methods and attribute;

1. `__repr__`/`__str__`: prints all `get` and `set` fileds.
2. `__iter__`: returns iterator of all `get` fields
3. `__len__`: returns number of `get` fields (compile-time constant)
4. `__init__`: a constructor with all fields (technically `__new__`)
5. `__match_args__`: supports pattern matching by positional attr. with all `get` fields
6. `__eq__`: besed on `PartialEq`/`Eq` trait
7. `__ord__` for `__lt__`, `__le__`, `__gt__` and `__ge__`: besed on `PartialOrd`/`Ord` trait
8. `__hash__`: based on `Hash` trait
   - *Note that implementing any of `__eq__`, `__lt__`, `__le__`, `__gt__` and `__ge__` methods will cause Python not to generate a default `__hash__` implementation, so consider also implementing `__hash__`.*

For example,

```rust
use derive_pysp::*;

#[derive(__init__, __match_args__, __repr__)]
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