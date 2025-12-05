//! Python bindings for the ANAF API client.
//!
//! This module provides Python classes and functions to interact with the Romanian
//! National Agency for Fiscal Administration (ANAF) APIs.

mod client;
mod requests;
mod responses;
mod versions;

pub use client::*;
pub use requests::*;
pub use responses::*;
pub use versions::*;

use pyo3::prelude::*;

/// Register all Python classes and functions in the module.
pub fn register_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Client
    m.add_class::<PyAnafClient>()?;

    // Requests
    m.add_class::<PyApiRequest>()?;
    m.add_class::<PyBalanceRequest>()?;

    // Versions
    m.add_class::<PyVatPayerApiVersion>()?;
    m.add_class::<PyBalanceApiVersion>()?;
    m.add_class::<PyCultApiVersion>()?;
    m.add_class::<PyFarmerApiVersion>()?;

    // VAT Payer responses (new coherent structure)
    m.add_class::<PyVatPayerResponse>()?;
    m.add_class::<PyTaxPayerEntity>()?;
    m.add_class::<PyGeneralData>()?;
    m.add_class::<PyVatRegistration>()?;
    m.add_class::<PyVatPeriod>()?;
    m.add_class::<PyVatCollection>()?;
    m.add_class::<PyInactivityStatus>()?;
    m.add_class::<PySplitVat>()?;
    m.add_class::<PyHeadquartersAddress>()?;
    m.add_class::<PyFiscalAddress>()?;

    // Cult responses
    m.add_class::<PyCultResponse>()?;
    m.add_class::<PyCultResponseItem>()?;

    // Farmer responses
    m.add_class::<PyFarmerResponse>()?;
    m.add_class::<PyFarmerResponseItem>()?;

    // Balance responses
    m.add_class::<PyBalanceResponse>()?;
    m.add_class::<PyRawBalance>()?;

    Ok(())
}
