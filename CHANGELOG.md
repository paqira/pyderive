# Changelog

## v0.8.0 - 2014-12-13

- Migrate to PyO3 0.23.0
- Deprecates `PyHash`, PyO3 officially provides same functionality by `#[pyclass(hash)]`
- Deprecates `ToPyObject` because it is deprecated since PyO3 0.23.0 
