//! Provides derive macros that implements enumeration of numeric type.

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
///     fn __add__(&self, other: &Self) -> <&Self as Add<&Self>>::Output {
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
///     fn __and__(&self, other: &Self) -> <&Self as BitAnd<&Self>>::Output {
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
///     fn __divmod__(&self, other: &Self) -> (
///         <&Self as Div<&Self>>::Output,
///         <&Self as Rem<&Self>>::Output
///     ) {
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
///     fn __floordiv__(&self, other: &Self) -> <&Self as Div<&Self>>::Output {
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
///     fn __invert__(&self) -> <&Self as Not>::Output {
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
///     fn __lshift__(&self, other: &Self) -> <&Self as Shl<&Self>>::Output {
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
///     fn __matmul__(&self, other: &Self) -> <&Self as Mul<&Self>>::Output {
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
///     fn __mod__(&self, other: &Self) -> <&Self as Rem<&Self>>::Output {
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
///     fn __mul__(&self, other: &Self) -> <&Self as Mul<&Self>>::Output {
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
///     fn __neg__(&self) -> <&Self as Neg>::Output {
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
///     fn __or__(&self, other: &Self) -> <&Self as BitOr<&Self>>::Output {
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
///     fn __radd__(&self, other: &Self) -> <&Self as Add<&Self>>::Output {
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
///     fn __rand__(&self, other: &Self) -> <&Self as BitAnd<&Self>>::Output {
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
///     fn __rdivmod__(&self, other: &Self) -> (
///         <&Self as Div<&Self>>::Output,
///         <&Self as Rem<&Self>>::Output
///     ) {
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
///     fn __rfloordiv__(&self, other: &Self) -> <&Self as Div<&Self>>::Output {
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
///     fn __rlshift__(&self, other: &Self) -> <&Self as Shl<&Self>>::Output {
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
///     fn __rmatmul__(&self, other: &Self) -> <&Self as Mul<&Self>>::Output {
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
///     fn __rmod__(&self, other: &Self) -> <&Self as Rem<&Self>>::Output {
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
///     fn __rmul__(&self, other: &Self) -> <&Self as Mul<&Self>>::Output {
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
///     fn __ror__(&self, other: &Self) -> <&Self as BitOr<&Self>>::Output {
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
///     fn __rrshift__(&self, other: &Self) -> <&Self as Shr<&Self>>::Output {
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
///     fn __rsub__(&self, other: &Self) -> <&Self as Sub<&Self>>::Output {
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
///     fn __rtruediv__(&self, other: &Self) -> <&Self as Div<&Self>>::Output {
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
///     fn __rxor__(&self, other: &Self) -> <&Self as BitXor<&Self>>::Output {
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
///     fn __rshift__(&self, other: &Self) -> <&Self as Shr<&Self>>::Output {
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
///     fn __sub__(&self, other: &Self) -> <&Self as Sub<&Self>>::Output {
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
///     fn __truediv__(&self, other: &Self) -> <&Self as Div<&Self>>::Output {
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
///     fn __xor__(&self, other: &Self) -> <&Self as BitXor<&Self>>::Output {
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
