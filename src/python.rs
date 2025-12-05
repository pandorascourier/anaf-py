use chrono::NaiveDate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::sync::OnceLock;

use crate::{AnafClient, ApiRequest, Result};

use crate::vat_payer::VatPayerApiVersion;

#[cfg(feature = "balance_api")]
use crate::balance::{BalanceApiVersion, BalanceRequest};

#[cfg(feature = "farmers_api")]
use crate::farmers::FarmerApiVersion;

#[cfg(feature = "cults_api")]
use crate::cults::CultApiVersion;

// Global shared runtime for all client instances
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn get_runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime")
    })
}

/// Helper function to parse date string and create API requests
fn parse_requests(requests: Vec<(usize, String)>) -> PyResult<Vec<ApiRequest>> {
    requests
        .into_iter()
        .map(|(code, date_str)| {
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|e| PyException::new_err(format!("Invalid date format: {}", e)))?;
            Ok(ApiRequest::new(code, date))
        })
        .collect()
}

/// Helper function to handle API response and convert to JSON string
fn handle_response<T: serde::Serialize>(result: Result<T>) -> PyResult<String> {
    match result {
        Ok(response) => {
            serde_json::to_string(&response)
                .map_err(|e| PyException::new_err(format!("Serialization error: {}", e)))
        }
        Err(e) => Err(PyException::new_err(format!("API error: {}", e))),
    }
}

// Python wrapper for AnafClient
#[pyclass]
pub struct PyAnafClient {
    client: AnafClient,
}

#[pymethods]
impl PyAnafClient {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Self {
            client: AnafClient::new(),
        })
    }

    /// Query VAT Payer information
    /// 
    /// Args:
    ///     requests: List of tuples (registration_code: int, date: str in YYYY-MM-DD format)
    ///     version: API version (7 or 8, default is 8)
    ///     async_mode: Whether to use async API (default False)
    /// 
    /// Returns:
    ///     Dictionary with the API response
    #[pyo3(signature = (requests, version=8, async_mode=false))]
    pub fn vat_payer(
        &self,
        requests: Vec<(usize, String)>,
        version: u8,
        async_mode: bool,
    ) -> PyResult<String> {
        let api_version = match version {
            7 => VatPayerApiVersion::V7,
            8 => VatPayerApiVersion::V8,
            _ => return Err(PyException::new_err("Invalid API version. Use 7 or 8")),
        };

        let api_requests = parse_requests(requests)?;
        let client = self.client.clone();
        
        let result = if async_mode {
            #[cfg(feature = "vat_payer_async_api")]
            {
                get_runtime().block_on(async move {
                    client.async_vat_payer(api_version).send(api_requests).await
                })
            }
            #[cfg(not(feature = "vat_payer_async_api"))]
            {
                return Err(PyException::new_err(
                    "Async VAT Payer API not enabled in this build",
                ));
            }
        } else {
            get_runtime().block_on(async move {
                client.vat_payer(api_version).send(api_requests).await
            })
        };

        handle_response(result)
    }

    /// Query Balance information
    /// 
    /// Args:
    ///     registration_code: Company registration code (CUI)
    ///     year: Year for the balance query
    ///     version: API version (default is 1)
    /// 
    /// Returns:
    ///     Dictionary with the API response
    #[cfg(feature = "balance_api")]
    #[pyo3(signature = (registration_code, year, version=1))]
    pub fn balance(
        &self,
        registration_code: usize,
        year: i32,
        version: u8,
    ) -> PyResult<String> {
        let api_version = match version {
            1 => BalanceApiVersion::V1,
            _ => return Err(PyException::new_err("Invalid API version. Use 1")),
        };

        if year < 0 {
            return Err(PyException::new_err("Year must be positive"));
        }
        let year_usize = year as usize;
        let request = BalanceRequest::new(registration_code, year_usize);
        let client = self.client.clone();

        let result = get_runtime().block_on(async move {
            client.balance(api_version).send(request).await
        });

        handle_response(result)
    }

    /// Query Farmers Registry information
    /// 
    /// Args:
    ///     requests: List of tuples (registration_code: int, date: str in YYYY-MM-DD format)
    ///     version: API version (default is 2)
    /// 
    /// Returns:
    ///     Dictionary with the API response
    #[cfg(feature = "farmers_api")]
    #[pyo3(signature = (requests, version=2))]
    pub fn farmer(
        &self,
        requests: Vec<(usize, String)>,
        version: u8,
    ) -> PyResult<String> {
        let api_version = match version {
            2 => FarmerApiVersion::V2,
            _ => return Err(PyException::new_err("Invalid API version. Use 2")),
        };

        let api_requests = parse_requests(requests)?;
        let client = self.client.clone();

        let result = get_runtime().block_on(async move {
            client.farmer(api_version).send(api_requests).await
        });

        handle_response(result)
    }

    /// Query Cults Registry information
    /// 
    /// Args:
    ///     requests: List of tuples (registration_code: int, date: str in YYYY-MM-DD format)
    ///     version: API version (default is 2)
    /// 
    /// Returns:
    ///     Dictionary with the API response
    #[cfg(feature = "cults_api")]
    #[pyo3(signature = (requests, version=2))]
    pub fn cult(
        &self,
        requests: Vec<(usize, String)>,
        version: u8,
    ) -> PyResult<String> {
        let api_version = match version {
            2 => CultApiVersion::V2,
            _ => return Err(PyException::new_err("Invalid API version. Use 2")),
        };

        let api_requests = parse_requests(requests)?;
        let client = self.client.clone();

        let result = get_runtime().block_on(async move {
            client.cult(api_version).send(api_requests).await
        });

        handle_response(result)
    }
}

/// Python module for ANAF API client
#[pymodule]
pub fn _anaf_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAnafClient>()?;
    Ok(())
}
