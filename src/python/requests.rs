//! Python wrappers for API requests.

use chrono::NaiveDate;
use pyo3::prelude::*;

use crate::ApiRequest;

#[cfg(feature = "balance_api")]
use crate::balance::BalanceRequest;

/// API request for VAT Payer, Cult, and Farmer APIs.
///
/// Args:
///     registration_code: The company's unique registration code (CUI).
///     year: The year for the query.
///     month: The month for the query.
///     day: The day for the query.
///
/// Example:
///     >>> request = ApiRequest(12345678, 2024, 12, 5)
#[pyclass(name = "ApiRequest")]
#[derive(Clone, Debug)]
pub struct PyApiRequest {
    pub(crate) inner: ApiRequest,
}

#[pymethods]
impl PyApiRequest {
    /// Create a new API request.
    ///
    /// Args:
    ///     registration_code: The company's unique registration code (CUI).
    ///     year: The year for the query.
    ///     month: The month for the query.
    ///     day: The day for the query.
    #[new]
    pub fn new(registration_code: usize, year: i32, month: u32, day: u32) -> PyResult<Self> {
        let date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(format!(
                "Invalid date: {}-{}-{}",
                year, month, day
            ))
        })?;
        Ok(Self {
            inner: ApiRequest::new(registration_code, date),
        })
    }

    /// Create a request for today's date.
    ///
    /// Args:
    ///     registration_code: The company's unique registration code (CUI).
    #[staticmethod]
    pub fn today(registration_code: usize) -> Self {
        let today = chrono::Local::now().date_naive();
        Self {
            inner: ApiRequest::new(registration_code, today),
        }
    }

    /// The company's unique registration code (CUI).
    #[getter]
    pub fn registration_code(&self) -> usize {
        self.inner.registration_code
    }

    /// The date for the query.
    #[getter]
    pub fn date(&self) -> String {
        self.inner.when.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "ApiRequest(registration_code={}, date='{}')",
            self.inner.registration_code, self.inner.when
        )
    }
}

/// Balance API request.
///
/// Args:
///     registration_code: The company's unique registration code (CUI).
///     year: The year to get balance for.
///
/// Example:
///     >>> request = BalanceRequest(12345678, 2023)
#[cfg(feature = "balance_api")]
#[pyclass(name = "BalanceRequest")]
#[derive(Clone, Debug)]
pub struct PyBalanceRequest {
    pub(crate) inner: BalanceRequest,
}

#[cfg(feature = "balance_api")]
#[pymethods]
impl PyBalanceRequest {
    /// Create a new Balance API request.
    ///
    /// Args:
    ///     registration_code: The company's unique registration code (CUI).
    ///     year: The year to get balance for.
    #[new]
    pub fn new(registration_code: usize, year: usize) -> Self {
        Self {
            inner: BalanceRequest::new(registration_code, year),
        }
    }

    /// The company's unique registration code (CUI).
    #[getter]
    pub fn registration_code(&self) -> usize {
        self.inner.registration_code
    }

    /// The year for the balance query.
    #[getter]
    pub fn year(&self) -> usize {
        self.inner.year
    }

    fn __repr__(&self) -> String {
        format!(
            "BalanceRequest(registration_code={}, year={})",
            self.inner.registration_code, self.inner.year
        )
    }
}
