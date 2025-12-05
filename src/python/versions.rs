//! Python wrappers for API version enums.

use pyo3::prelude::*;

use crate::vat_payer::VatPayerApiVersion;

#[cfg(feature = "balance_api")]
use crate::balance::BalanceApiVersion;
#[cfg(feature = "cults_api")]
use crate::cults::CultApiVersion;
#[cfg(feature = "farmers_api")]
use crate::farmers::FarmerApiVersion;

/// VAT Payer API version selector.
///
/// Available versions:
/// - V9 (default, latest) - max 100 CUIs per request
/// - V8 - max 500 CUIs per request
/// - V7 - max 500 CUIs per request
#[pyclass(name = "VatPayerApiVersion")]
#[derive(Clone, Debug)]
pub struct PyVatPayerApiVersion {
    pub(crate) inner: VatPayerApiVersion,
}

#[pymethods]
impl PyVatPayerApiVersion {
    /// Create version V9 (latest).
    #[staticmethod]
    pub fn v9() -> Self {
        Self {
            inner: VatPayerApiVersion::V9,
        }
    }

    /// Create version V8.
    #[staticmethod]
    pub fn v8() -> Self {
        Self {
            inner: VatPayerApiVersion::V8,
        }
    }

    /// Create version V7.
    #[staticmethod]
    pub fn v7() -> Self {
        Self {
            inner: VatPayerApiVersion::V7,
        }
    }

    /// Get the latest version.
    #[staticmethod]
    pub fn latest() -> Self {
        Self {
            inner: VatPayerApiVersion::latest(),
        }
    }

    /// Get the maximum number of CUIs allowed per request.
    pub fn max_cuis(&self) -> usize {
        self.inner.max_cuis()
    }

    fn __repr__(&self) -> String {
        format!("VatPayerApiVersion({})", self.inner)
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}

impl Default for PyVatPayerApiVersion {
    fn default() -> Self {
        Self {
            inner: VatPayerApiVersion::default(),
        }
    }
}

/// Balance API version selector.
///
/// Available versions:
/// - V1 (default, latest)
#[cfg(feature = "balance_api")]
#[pyclass(name = "BalanceApiVersion")]
#[derive(Clone, Debug)]
pub struct PyBalanceApiVersion {
    pub(crate) inner: BalanceApiVersion,
}

#[cfg(feature = "balance_api")]
#[pymethods]
impl PyBalanceApiVersion {
    /// Create version V1 (latest).
    #[staticmethod]
    pub fn v1() -> Self {
        Self {
            inner: BalanceApiVersion::V1,
        }
    }

    /// Get the latest version.
    #[staticmethod]
    pub fn latest() -> Self {
        Self {
            inner: BalanceApiVersion::latest(),
        }
    }

    fn __repr__(&self) -> String {
        format!("BalanceApiVersion({})", self.inner)
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}

#[cfg(feature = "balance_api")]
impl Default for PyBalanceApiVersion {
    fn default() -> Self {
        Self {
            inner: BalanceApiVersion::default(),
        }
    }
}

/// Cult API version selector.
///
/// Available versions:
/// - V2 (default, latest)
#[cfg(feature = "cults_api")]
#[pyclass(name = "CultApiVersion")]
#[derive(Clone, Debug)]
pub struct PyCultApiVersion {
    pub(crate) inner: CultApiVersion,
}

#[cfg(feature = "cults_api")]
#[pymethods]
impl PyCultApiVersion {
    /// Create version V2 (latest).
    #[staticmethod]
    pub fn v2() -> Self {
        Self {
            inner: CultApiVersion::V2,
        }
    }

    /// Get the latest version.
    #[staticmethod]
    pub fn latest() -> Self {
        Self {
            inner: CultApiVersion::latest(),
        }
    }

    fn __repr__(&self) -> String {
        format!("CultApiVersion({})", self.inner)
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}

#[cfg(feature = "cults_api")]
impl Default for PyCultApiVersion {
    fn default() -> Self {
        Self {
            inner: CultApiVersion::default(),
        }
    }
}

/// Farmer API version selector.
///
/// Available versions:
/// - V2 (default, latest)
#[cfg(feature = "farmers_api")]
#[pyclass(name = "FarmerApiVersion")]
#[derive(Clone, Debug)]
pub struct PyFarmerApiVersion {
    pub(crate) inner: FarmerApiVersion,
}

#[cfg(feature = "farmers_api")]
#[pymethods]
impl PyFarmerApiVersion {
    /// Create version V2 (latest).
    #[staticmethod]
    pub fn v2() -> Self {
        Self {
            inner: FarmerApiVersion::V2,
        }
    }

    /// Get the latest version.
    #[staticmethod]
    pub fn latest() -> Self {
        Self {
            inner: FarmerApiVersion::latest(),
        }
    }

    fn __repr__(&self) -> String {
        format!("FarmerApiVersion({})", self.inner)
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}

#[cfg(feature = "farmers_api")]
impl Default for PyFarmerApiVersion {
    fn default() -> Self {
        Self {
            inner: FarmerApiVersion::default(),
        }
    }
}
