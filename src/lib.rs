mod apis;
mod client;
mod common;

#[cfg(feature = "python")]
mod python;

#[cfg(feature = "php")]
mod php;

pub use client::AnafClient;

pub use apis::*;
pub use common::*;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// ANAF API - Python bindings for Romanian fiscal administration APIs.
///
/// This module provides access to various ANAF (National Agency for Fiscal Administration)
/// APIs including:
///
/// - **VAT Payer API**: Query company VAT registration status
/// - **Cult API**: Query cult/religious organization registry
/// - **Farmer API**: Query agricultural producer registry
/// - **Balance API**: Query company financial balance sheets
///
/// Example:
///     >>> from anaf_api import AnafClient, ApiRequest
///     >>> client = AnafClient()
///     >>> request = ApiRequest.today(12345678)  # Query by CUI
///     >>> response = client.get_vat_payer([request])
///     >>> for company in response.data:
///     ...     print(f"{company.company_info.name}: VAT payer = {company.vat_scope.is_payer}")
#[cfg(feature = "python")]
#[pymodule]
fn anaf_api(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python::register_module(m)?;
    Ok(())
}

