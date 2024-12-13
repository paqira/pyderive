# Changelog

## v0.8.2 - 20xx-xx-xx

- Remove module declaration from iterator impls of `PyIter` and `PyReversed` 
- Remove `ToPyObject` method from `PyIter` and `PyReversed` (migration to PyO3 0.23.0 is complete, maybe)

## v0.8.1 - 2024-12-13

- Fix `PyEq` document


## v0.8.0 - 2024-12-13

- Migrate to PyO3 0.23.0
- Deprecates `PyHash`, PyO3 officially provides same functionality by `#[pyclass(hash)]`
- Deprecates `ToPyObject` because it is deprecated since PyO3 0.23.0 
- Bump MSRV to 1.63 from 1.62
