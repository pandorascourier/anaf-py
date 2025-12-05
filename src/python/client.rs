//! Python wrapper for the ANAF API client.

use pyo3::prelude::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::vat_payer::VatPayerApiVersion;
use crate::AnafClient;

#[cfg(feature = "balance_api")]
use crate::balance::BalanceApiVersion;
#[cfg(feature = "cults_api")]
use crate::cults::CultApiVersion;
#[cfg(feature = "farmers_api")]
use crate::farmers::FarmerApiVersion;

use super::{
    PyApiRequest, PyVatPayerApiVersion, PyVatPayerResponse,
};

#[cfg(feature = "balance_api")]
use super::{PyBalanceApiVersion, PyBalanceRequest, PyBalanceResponse};
#[cfg(feature = "cults_api")]
use super::{PyCultApiVersion, PyCultResponse};
#[cfg(feature = "farmers_api")]
use super::{PyFarmerApiVersion, PyFarmerResponse};

/// ANAF API Client for Python.
///
/// Provides access to Romanian National Agency for Fiscal Administration APIs.
///
/// Example:
///     >>> client = AnafClient()
///     >>> request = ApiRequest.today(12345678)
///     >>> response = client.get_vat_payer([request])
///     >>> for company in response.data:
///     ...     print(company.company_info.name)
#[pyclass(name = "AnafClient")]
pub struct PyAnafClient {
    runtime: Arc<Runtime>,
}

#[pymethods]
impl PyAnafClient {
    /// Create a new ANAF API client.
    #[new]
    pub fn new() -> PyResult<Self> {
        let runtime = Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        Ok(Self {
            runtime: Arc::new(runtime),
        })
    }

    /// Query VAT payer information for companies.
    ///
    /// Args:
    ///     requests: List of ApiRequest objects (max 500).
    ///     version: API version to use (default: latest).
    ///
    /// Returns:
    ///     VatPayerResponse with company information.
    ///
    /// Example:
    ///     >>> client = AnafClient()
    ///     >>> request = ApiRequest.today(12345678)
    ///     >>> response = client.get_vat_payer([request])
    #[pyo3(signature = (requests, version=None))]
    pub fn get_vat_payer(
        &self,
        requests: Vec<PyApiRequest>,
        version: Option<PyVatPayerApiVersion>,
    ) -> PyResult<PyVatPayerResponse> {
        let version = version.map(|v| v.inner).unwrap_or(VatPayerApiVersion::default());
        let api_requests: Vec<_> = requests.into_iter().map(|r| r.inner).collect();

        let client = AnafClient::new();
        let api = client.vat_payer(version);

        self.runtime
            .block_on(async { api.send(api_requests).await })
            .map(PyVatPayerResponse::from)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Query VAT payer information asynchronously (bulk API).
    ///
    /// This is the async variant that supports more requests.
    ///
    /// Args:
    ///     requests: List of ApiRequest objects (max 500).
    ///     version: API version to use (default: latest).
    ///
    /// Returns:
    ///     VatPayerResponse with company information.
    #[cfg(feature = "vat_payer_async_api")]
    #[pyo3(signature = (requests, version=None))]
    pub fn get_vat_payer_async(
        &self,
        requests: Vec<PyApiRequest>,
        version: Option<PyVatPayerApiVersion>,
    ) -> PyResult<PyVatPayerResponse> {
        let version = version.map(|v| v.inner).unwrap_or(VatPayerApiVersion::default());
        let api_requests: Vec<_> = requests.into_iter().map(|r| r.inner).collect();

        let client = AnafClient::new();
        let api = client.async_vat_payer(version);

        self.runtime
            .block_on(async { api.send(api_requests).await })
            .map(PyVatPayerResponse::from)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Query cult registry information.
    ///
    /// Args:
    ///     requests: List of ApiRequest objects (max 500).
    ///     version: API version to use (default: latest).
    ///
    /// Returns:
    ///     CultResponse with cult information.
    ///
    /// Example:
    ///     >>> client = AnafClient()
    ///     >>> request = ApiRequest.today(12345678)
    ///     >>> response = client.get_cult([request])
    #[cfg(feature = "cults_api")]
    #[pyo3(signature = (requests, version=None))]
    pub fn get_cult(
        &self,
        requests: Vec<PyApiRequest>,
        version: Option<PyCultApiVersion>,
    ) -> PyResult<PyCultResponse> {
        let version = version.map(|v| v.inner).unwrap_or(CultApiVersion::default());
        let api_requests: Vec<_> = requests.into_iter().map(|r| r.inner).collect();

        let client = AnafClient::new();
        let api = client.cult(version);

        self.runtime
            .block_on(async { api.send(api_requests).await })
            .map(PyCultResponse::from)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Query farmer registry information.
    ///
    /// Args:
    ///     requests: List of ApiRequest objects (max 500).
    ///     version: API version to use (default: latest).
    ///
    /// Returns:
    ///     FarmerResponse with farmer information.
    ///
    /// Example:
    ///     >>> client = AnafClient()
    ///     >>> request = ApiRequest.today(12345678)
    ///     >>> response = client.get_farmer([request])
    #[cfg(feature = "farmers_api")]
    #[pyo3(signature = (requests, version=None))]
    pub fn get_farmer(
        &self,
        requests: Vec<PyApiRequest>,
        version: Option<PyFarmerApiVersion>,
    ) -> PyResult<PyFarmerResponse> {
        let version = version.map(|v| v.inner).unwrap_or(FarmerApiVersion::default());
        let api_requests: Vec<_> = requests.into_iter().map(|r| r.inner).collect();

        let client = AnafClient::new();
        let api = client.farmer(version);

        self.runtime
            .block_on(async { api.send(api_requests).await })
            .map(PyFarmerResponse::from)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    /// Query company balance information.
    ///
    /// Args:
    ///     request: BalanceRequest object.
    ///     version: API version to use (default: latest).
    ///
    /// Returns:
    ///     BalanceResponse with balance information.
    ///
    /// Example:
    ///     >>> client = AnafClient()
    ///     >>> request = BalanceRequest(12345678, 2023)
    ///     >>> response = client.get_balance(request)
    #[cfg(feature = "balance_api")]
    #[pyo3(signature = (request, version=None))]
    pub fn get_balance(
        &self,
        request: PyBalanceRequest,
        version: Option<PyBalanceApiVersion>,
    ) -> PyResult<PyBalanceResponse> {
        let version = version.map(|v| v.inner).unwrap_or(BalanceApiVersion::default());

        let client = AnafClient::new();
        let api = client.balance(version);

        self.runtime
            .block_on(async { api.send(request.inner).await })
            .map(PyBalanceResponse::from)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn __repr__(&self) -> &'static str {
        "AnafClient()"
    }
}

impl Default for PyAnafClient {
    fn default() -> Self {
        Self::new().expect("Failed to create AnafClient")
    }
}
