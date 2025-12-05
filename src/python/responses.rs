//! Python wrappers for ANAF API responses.
//!
//! This module provides Python-accessible classes for all response types
//! from ANAF APIs, with comprehensive documentation and type hints.

use pyo3::prelude::*;
use std::collections::HashMap;

use crate::vat_payer::{
    FiscalAddress, GeneralData, HeadquartersAddress, InactivityStatus,
    SplitVat, TaxPayerEntity, VatCollection, VatPeriod, VatRegistration,
    VatPayerResponse,
};

#[cfg(feature = "cults_api")]
use crate::cults::{CultResponse, CultResponseItem};

#[cfg(feature = "farmers_api")]
use crate::farmers::{FarmerResponse, FarmerResponseItem};

#[cfg(feature = "balance_api")]
use crate::balance::{BalanceResponse, RawBalance};

// =============================================================================
// VAT Payer Response (Main Response Container)
// =============================================================================

/// Response from the ANAF VAT Payer (PlatitorTvaRest) API.
///
/// Contains a list of found tax entities and a list of CUIs that were not found.
///
/// Attributes:
///     status: HTTP status code (0 for V9, 200 for older versions).
///     message: Response message (empty for V9).
///     data: List of found TaxPayerEntity objects.
///     not_found: List of CUIs (registration codes) that were not found.
#[pyclass(name = "VatPayerResponse")]
#[derive(Clone, Debug)]
pub struct PyVatPayerResponse {
    #[pyo3(get)]
    pub status: usize,
    #[pyo3(get)]
    pub message: String,
    #[pyo3(get)]
    pub data: Vec<PyTaxPayerEntity>,
    #[pyo3(get)]
    pub not_found: Vec<usize>,
}

#[pymethods]
impl PyVatPayerResponse {
    fn __repr__(&self) -> String {
        format!(
            "VatPayerResponse(found={}, not_found={})",
            self.data.len(),
            self.not_found.len()
        )
    }

    fn __len__(&self) -> usize {
        self.data.len()
    }

    /// Get the first found entity, if any.
    pub fn first(&self) -> Option<PyTaxPayerEntity> {
        self.data.first().cloned()
    }
}

impl From<VatPayerResponse> for PyVatPayerResponse {
    fn from(resp: VatPayerResponse) -> Self {
        Self {
            status: resp.status,
            message: resp.message,
            data: resp.data.into_iter().map(PyTaxPayerEntity::from).collect(),
            not_found: resp.not_found,
        }
    }
}

// =============================================================================
// Tax Payer Entity (Root Entity Structure)
// =============================================================================

/// Romanian tax payee entity from ANAF registry.
///
/// Represents a company, PFA (authorized natural person), or other fiscal entity
/// registered with ANAF (Romanian National Agency for Fiscal Administration).
///
/// This is the main data structure returned for each found CUI in the response.
///
/// Attributes:
///     general_data: Core identification info (CUI, name, registration).
///     vat_registration: VAT (TVA) registration status and history.
///     vat_collection: Cash-based VAT (TVA la încasare) details.
///     inactivity_status: Inactive/reactivation status.
///     split_vat: Split VAT payment system details.
///     headquarters_address: Registered office (sediu social) address.
///     fiscal_address: Fiscal domicile address.
#[pyclass(name = "TaxPayerEntity")]
#[derive(Clone, Debug)]
pub struct PyTaxPayerEntity {
    #[pyo3(get)]
    pub general_data: PyGeneralData,
    #[pyo3(get)]
    pub vat_registration: PyVatRegistration,
    #[pyo3(get)]
    pub vat_collection: PyVatCollection,
    #[pyo3(get)]
    pub inactivity_status: PyInactivityStatus,
    #[pyo3(get)]
    pub split_vat: PySplitVat,
    #[pyo3(get)]
    pub headquarters_address: PyHeadquartersAddress,
    #[pyo3(get)]
    pub fiscal_address: PyFiscalAddress,
}

#[pymethods]
impl PyTaxPayerEntity {
    fn __repr__(&self) -> String {
        format!(
            "TaxPayerEntity(cui={}, name='{}')",
            self.general_data.cui, self.general_data.name
        )
    }

    /// Check if entity is a VAT payer.
    #[getter]
    pub fn is_vat_payer(&self) -> bool {
        self.vat_registration.is_registered
    }

    /// Check if entity is inactive.
    #[getter]
    pub fn is_inactive(&self) -> bool {
        self.inactivity_status.is_inactive
    }

    /// Check if entity uses split VAT.
    #[getter]
    pub fn uses_split_vat(&self) -> bool {
        self.split_vat.is_applied
    }

    /// Check if entity is in RO e-Factura registry.
    #[getter]
    pub fn in_ro_efactura(&self) -> bool {
        self.general_data.ro_efactura_status
    }
}

impl From<TaxPayerEntity> for PyTaxPayerEntity {
    fn from(entity: TaxPayerEntity) -> Self {
        Self {
            general_data: PyGeneralData::from(entity.general_data),
            vat_registration: PyVatRegistration::from(entity.vat_registration),
            vat_collection: PyVatCollection::from(entity.vat_collection),
            inactivity_status: PyInactivityStatus::from(entity.inactivity_status),
            split_vat: PySplitVat::from(entity.split_vat),
            headquarters_address: PyHeadquartersAddress::from(entity.headquarters_address),
            fiscal_address: PyFiscalAddress::from(entity.fiscal_address),
        }
    }
}

// =============================================================================
// General Data
// =============================================================================

/// General entity details from ANAF registry.
///
/// Contains core identification information including CUI (fiscal code),
/// legal name, registration details, CAEN code, and fiscal authority.
///
/// Attributes:
///     cui: Fiscal identification code (Cod Unic de Înregistrare).
///     query_date: Date for which the query was made (YYYY-MM-DD).
///     name: Legal entity name.
///     address: Full address string.
///     trade_register_number: Trade register number (e.g., "F40/8875/2023").
///     phone: Phone number (may be empty).
///     fax: Fax number (may be empty).
///     postal_code: Postal code.
///     authorization_act: Authorization act reference.
///     registration_status: Registration status text.
///     registration_date: Entity registration date (YYYY-MM-DD).
///     caen_code: CAEN activity code (e.g., "6201").
///     iban: IBAN for split VAT payments.
///     ro_efactura_status: True if in RO e-Factura registry.
///     ro_efactura_registration_date: Date of e-Factura registration.
///     fiscal_authority: Competent fiscal authority name.
///     ownership_form: Ownership form.
///     organization_form: Organization form.
///     legal_form: Legal/juridical form.
#[pyclass(name = "GeneralData")]
#[derive(Clone, Debug)]
pub struct PyGeneralData {
    #[pyo3(get)]
    pub cui: u64,
    #[pyo3(get)]
    pub query_date: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub address: String,
    #[pyo3(get)]
    pub trade_register_number: String,
    #[pyo3(get)]
    pub phone: String,
    #[pyo3(get)]
    pub fax: String,
    #[pyo3(get)]
    pub postal_code: String,
    #[pyo3(get)]
    pub authorization_act: String,
    #[pyo3(get)]
    pub registration_status: String,
    #[pyo3(get)]
    pub registration_date: String,
    #[pyo3(get)]
    pub caen_code: String,
    #[pyo3(get)]
    pub iban: String,
    #[pyo3(get)]
    pub ro_efactura_status: bool,
    #[pyo3(get)]
    pub ro_efactura_registration_date: String,
    #[pyo3(get)]
    pub fiscal_authority: String,
    #[pyo3(get)]
    pub ownership_form: String,
    #[pyo3(get)]
    pub organization_form: String,
    #[pyo3(get)]
    pub legal_form: String,
}

#[pymethods]
impl PyGeneralData {
    fn __repr__(&self) -> String {
        format!("GeneralData(cui={}, name='{}')", self.cui, self.name)
    }
}

impl From<GeneralData> for PyGeneralData {
    fn from(data: GeneralData) -> Self {
        Self {
            cui: data.cui,
            query_date: data.query_date,
            name: data.name,
            address: data.address,
            trade_register_number: data.trade_register_number,
            phone: data.phone,
            fax: data.fax,
            postal_code: data.postal_code,
            authorization_act: data.authorization_act,
            registration_status: data.registration_status,
            registration_date: data.registration_date,
            caen_code: data.caen_code,
            iban: data.iban,
            ro_efactura_status: data.ro_efactura_status,
            ro_efactura_registration_date: data.ro_efactura_registration_date,
            fiscal_authority: data.fiscal_authority,
            ownership_form: data.ownership_form,
            organization_form: data.organization_form,
            legal_form: data.legal_form,
        }
    }
}

// =============================================================================
// VAT Registration
// =============================================================================

/// VAT (TVA) registration status and history.
///
/// Tracks whether an entity is registered for VAT purposes under
/// art. 316 of the Romanian Fiscal Code, including historical periods.
///
/// Attributes:
///     is_registered: True if currently registered for VAT purposes.
///     periods: List of VAT registration periods (historical data).
#[pyclass(name = "VatRegistration")]
#[derive(Clone, Debug)]
pub struct PyVatRegistration {
    #[pyo3(get)]
    pub is_registered: bool,
    #[pyo3(get)]
    pub periods: Vec<PyVatPeriod>,
}

#[pymethods]
impl PyVatRegistration {
    fn __repr__(&self) -> String {
        format!(
            "VatRegistration(is_registered={}, periods={})",
            self.is_registered,
            self.periods.len()
        )
    }

    /// Get the current/most recent VAT period if any.
    pub fn current_period(&self) -> Option<PyVatPeriod> {
        self.periods.first().cloned()
    }
}

impl From<VatRegistration> for PyVatRegistration {
    fn from(reg: VatRegistration) -> Self {
        Self {
            is_registered: reg.is_registered,
            periods: reg.periods.into_iter().map(PyVatPeriod::from).collect(),
        }
    }
}

/// Single VAT registration period.
///
/// Represents a time interval during which the entity was registered
/// for VAT purposes, including any annulment information.
///
/// Attributes:
///     start_date: Start date of VAT registration (YYYY-MM-DD).
///     end_date: End date of VAT registration (empty if still active).
///     annulment_date: Date when VAT was annulled.
///     annulment_message: Legal basis for annulment.
#[pyclass(name = "VatPeriod")]
#[derive(Clone, Debug)]
pub struct PyVatPeriod {
    #[pyo3(get)]
    pub start_date: String,
    #[pyo3(get)]
    pub end_date: String,
    #[pyo3(get)]
    pub annulment_date: String,
    #[pyo3(get)]
    pub annulment_message: String,
}

#[pymethods]
impl PyVatPeriod {
    fn __repr__(&self) -> String {
        if self.end_date.is_empty() {
            format!("VatPeriod(from='{}')", self.start_date)
        } else {
            format!(
                "VatPeriod(from='{}', to='{}')",
                self.start_date, self.end_date
            )
        }
    }

    /// Check if this period is currently active (no end date).
    #[getter]
    pub fn is_active(&self) -> bool {
        self.end_date.is_empty()
    }
}

impl From<VatPeriod> for PyVatPeriod {
    fn from(period: VatPeriod) -> Self {
        Self {
            start_date: period.start_date,
            end_date: period.end_date,
            annulment_date: period.annulment_date,
            annulment_message: period.annulment_message,
        }
    }
}

// =============================================================================
// VAT Collection (Cash-based VAT)
// =============================================================================

/// VAT at collection (TVA la încasare) details.
///
/// This system allows certain entities to pay VAT only when
/// payment is received, rather than when the invoice is issued.
///
/// Attributes:
///     is_applied: True if using cash-based VAT system.
///     start_date: Start date for VAT at collection.
///     end_date: End date for VAT at collection.
///     update_type: Type of last update.
///     update_date: Date of last update.
///     publication_date: Date when status was published.
#[pyclass(name = "VatCollection")]
#[derive(Clone, Debug)]
pub struct PyVatCollection {
    #[pyo3(get)]
    pub is_applied: bool,
    #[pyo3(get)]
    pub start_date: String,
    #[pyo3(get)]
    pub end_date: String,
    #[pyo3(get)]
    pub update_type: String,
    #[pyo3(get)]
    pub update_date: String,
    #[pyo3(get)]
    pub publication_date: String,
}

#[pymethods]
impl PyVatCollection {
    fn __repr__(&self) -> String {
        format!("VatCollection(is_applied={})", self.is_applied)
    }
}

impl From<VatCollection> for PyVatCollection {
    fn from(vc: VatCollection) -> Self {
        Self {
            is_applied: vc.is_applied,
            start_date: vc.start_date,
            end_date: vc.end_date,
            update_type: vc.update_type,
            update_date: vc.update_date,
            publication_date: vc.publication_date,
        }
    }
}

// =============================================================================
// Inactivity Status
// =============================================================================

/// Inactivity and reactivation status.
///
/// Tracks whether an entity has been marked as inactive by ANAF,
/// including dates for inactivation, reactivation, and deregistration.
///
/// Attributes:
///     is_inactive: True if entity is currently inactive.
///     inactivation_date: Date when marked inactive.
///     reactivation_date: Date when reactivated.
///     publication_date: Date when status was published.
///     deregistration_date: Date of deregistration/erasure.
#[pyclass(name = "InactivityStatus")]
#[derive(Clone, Debug)]
pub struct PyInactivityStatus {
    #[pyo3(get)]
    pub is_inactive: bool,
    #[pyo3(get)]
    pub inactivation_date: String,
    #[pyo3(get)]
    pub reactivation_date: String,
    #[pyo3(get)]
    pub publication_date: String,
    #[pyo3(get)]
    pub deregistration_date: String,
}

#[pymethods]
impl PyInactivityStatus {
    fn __repr__(&self) -> String {
        format!("InactivityStatus(is_inactive={})", self.is_inactive)
    }

    /// Check if entity has been deregistered.
    #[getter]
    pub fn is_deregistered(&self) -> bool {
        !self.deregistration_date.is_empty()
    }
}

impl From<InactivityStatus> for PyInactivityStatus {
    fn from(status: InactivityStatus) -> Self {
        Self {
            is_inactive: status.is_inactive,
            inactivation_date: status.inactivation_date,
            reactivation_date: status.reactivation_date,
            publication_date: status.publication_date,
            deregistration_date: status.deregistration_date,
        }
    }
}

// =============================================================================
// Split VAT
// =============================================================================

/// Split VAT payment system details.
///
/// The split VAT mechanism requires separate payment of VAT
/// directly to a dedicated tax account.
///
/// Attributes:
///     is_applied: True if using split VAT payment system.
///     start_date: Start date for split VAT.
///     cancellation_date: Date when split VAT was cancelled.
#[pyclass(name = "SplitVat")]
#[derive(Clone, Debug)]
pub struct PySplitVat {
    #[pyo3(get)]
    pub is_applied: bool,
    #[pyo3(get)]
    pub start_date: String,
    #[pyo3(get)]
    pub cancellation_date: String,
}

#[pymethods]
impl PySplitVat {
    fn __repr__(&self) -> String {
        format!("SplitVat(is_applied={})", self.is_applied)
    }
}

impl From<SplitVat> for PySplitVat {
    fn from(sv: SplitVat) -> Self {
        Self {
            is_applied: sv.is_applied,
            start_date: sv.start_date,
            cancellation_date: sv.cancellation_date,
        }
    }
}

// =============================================================================
// Addresses
// =============================================================================

/// Headquarters (sediu social) address.
///
/// The registered office address of the entity.
///
/// Attributes:
///     country: Country name.
///     county: County/district name.
///     county_code: County code (e.g., "40").
///     county_auto_code: County auto code (e.g., "B" for București).
///     locality: City/locality name.
///     locality_code: Locality code.
///     street: Street name.
///     street_number: Street number.
///     details: Additional details (floor, apartment, etc.).
///     postal_code: Postal code.
#[pyclass(name = "HeadquartersAddress")]
#[derive(Clone, Debug)]
pub struct PyHeadquartersAddress {
    #[pyo3(get)]
    pub country: String,
    #[pyo3(get)]
    pub county: String,
    #[pyo3(get)]
    pub county_code: String,
    #[pyo3(get)]
    pub county_auto_code: String,
    #[pyo3(get)]
    pub locality: String,
    #[pyo3(get)]
    pub locality_code: String,
    #[pyo3(get)]
    pub street: String,
    #[pyo3(get)]
    pub street_number: String,
    #[pyo3(get)]
    pub details: String,
    #[pyo3(get)]
    pub postal_code: String,
}

#[pymethods]
impl PyHeadquartersAddress {
    fn __repr__(&self) -> String {
        format!(
            "HeadquartersAddress(locality='{}', county='{}')",
            self.locality, self.county
        )
    }

    /// Get formatted full address string.
    pub fn full_address(&self) -> String {
        let mut parts = Vec::new();
        if !self.street.is_empty() {
            let street_part = if !self.street_number.is_empty() {
                format!("{} {}", self.street, self.street_number)
            } else {
                self.street.clone()
            };
            parts.push(street_part);
        }
        if !self.details.is_empty() {
            parts.push(self.details.clone());
        }
        if !self.locality.is_empty() {
            parts.push(self.locality.clone());
        }
        if !self.county.is_empty() && self.county != self.locality {
            parts.push(self.county.clone());
        }
        if !self.postal_code.is_empty() {
            parts.push(self.postal_code.clone());
        }
        if !self.country.is_empty() {
            parts.push(self.country.clone());
        }
        parts.join(", ")
    }
}

impl From<HeadquartersAddress> for PyHeadquartersAddress {
    fn from(addr: HeadquartersAddress) -> Self {
        Self {
            country: addr.country,
            county: addr.county,
            county_code: addr.county_code,
            county_auto_code: addr.county_auto_code,
            locality: addr.locality,
            locality_code: addr.locality_code,
            street: addr.street,
            street_number: addr.street_number,
            details: addr.details,
            postal_code: addr.postal_code,
        }
    }
}

/// Fiscal domicile address.
///
/// The address where the entity conducts its main fiscal activities.
/// May differ from headquarters address.
///
/// Attributes:
///     country: Country name.
///     county: County/district name.
///     county_code: County code.
///     county_auto_code: County auto code.
///     locality: City/locality name.
///     locality_code: Locality code.
///     street: Street name.
///     street_number: Street number.
///     details: Additional details.
///     postal_code: Postal code.
#[pyclass(name = "FiscalAddress")]
#[derive(Clone, Debug)]
pub struct PyFiscalAddress {
    #[pyo3(get)]
    pub country: String,
    #[pyo3(get)]
    pub county: String,
    #[pyo3(get)]
    pub county_code: String,
    #[pyo3(get)]
    pub county_auto_code: String,
    #[pyo3(get)]
    pub locality: String,
    #[pyo3(get)]
    pub locality_code: String,
    #[pyo3(get)]
    pub street: String,
    #[pyo3(get)]
    pub street_number: String,
    #[pyo3(get)]
    pub details: String,
    #[pyo3(get)]
    pub postal_code: String,
}

#[pymethods]
impl PyFiscalAddress {
    fn __repr__(&self) -> String {
        format!(
            "FiscalAddress(locality='{}', county='{}')",
            self.locality, self.county
        )
    }

    /// Get formatted full address string.
    pub fn full_address(&self) -> String {
        let mut parts = Vec::new();
        if !self.street.is_empty() {
            let street_part = if !self.street_number.is_empty() {
                format!("{} {}", self.street, self.street_number)
            } else {
                self.street.clone()
            };
            parts.push(street_part);
        }
        if !self.details.is_empty() {
            parts.push(self.details.clone());
        }
        if !self.locality.is_empty() {
            parts.push(self.locality.clone());
        }
        if !self.county.is_empty() && self.county != self.locality {
            parts.push(self.county.clone());
        }
        if !self.postal_code.is_empty() {
            parts.push(self.postal_code.clone());
        }
        if !self.country.is_empty() {
            parts.push(self.country.clone());
        }
        parts.join(", ")
    }
}

impl From<FiscalAddress> for PyFiscalAddress {
    fn from(addr: FiscalAddress) -> Self {
        Self {
            country: addr.country,
            county: addr.county,
            county_code: addr.county_code,
            county_auto_code: addr.county_auto_code,
            locality: addr.locality,
            locality_code: addr.locality_code,
            street: addr.street,
            street_number: addr.street_number,
            details: addr.details,
            postal_code: addr.postal_code,
        }
    }
}

// =============================================================================
// Cult Response Types
// =============================================================================

/// Response from the Cult API.
#[cfg(feature = "cults_api")]
#[pyclass(name = "CultResponse")]
#[derive(Clone, Debug)]
pub struct PyCultResponse {
    #[pyo3(get)]
    pub status: usize,
    #[pyo3(get)]
    pub message: String,
    #[pyo3(get)]
    pub data: Vec<PyCultResponseItem>,
    #[pyo3(get)]
    pub not_found: Vec<usize>,
}

#[cfg(feature = "cults_api")]
#[pymethods]
impl PyCultResponse {
    fn __repr__(&self) -> String {
        format!(
            "CultResponse(found={}, not_found={})",
            self.data.len(),
            self.not_found.len()
        )
    }

    fn __len__(&self) -> usize {
        self.data.len()
    }
}

#[cfg(feature = "cults_api")]
impl From<CultResponse> for PyCultResponse {
    fn from(resp: CultResponse) -> Self {
        Self {
            status: resp.status,
            message: resp.message,
            data: resp.data.into_iter().map(PyCultResponseItem::from).collect(),
            not_found: resp.not_found,
        }
    }
}

/// Information about a registered cult/religious organization.
#[cfg(feature = "cults_api")]
#[pyclass(name = "CultResponseItem")]
#[derive(Clone, Debug)]
pub struct PyCultResponseItem {
    #[pyo3(get)]
    pub cui: usize,
    #[pyo3(get)]
    pub query_date: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub address: String,
    #[pyo3(get)]
    pub trade_register_number: String,
    #[pyo3(get)]
    pub phone: String,
    #[pyo3(get)]
    pub fax: String,
    #[pyo3(get)]
    pub postal_code: String,
    #[pyo3(get)]
    pub authorization_act: String,
    #[pyo3(get)]
    pub registration_status: String,
    #[pyo3(get)]
    pub cult_start_date: String,
    #[pyo3(get)]
    pub cult_end_date: Option<String>,
    #[pyo3(get)]
    pub is_active: bool,
}

#[cfg(feature = "cults_api")]
#[pymethods]
impl PyCultResponseItem {
    fn __repr__(&self) -> String {
        format!(
            "CultResponseItem(cui={}, name='{}', is_active={})",
            self.cui, self.name, self.is_active
        )
    }
}

#[cfg(feature = "cults_api")]
impl From<CultResponseItem> for PyCultResponseItem {
    fn from(item: CultResponseItem) -> Self {
        Self {
            cui: item.unique_registration_code,
            query_date: item.when,
            name: item.name,
            address: item.address,
            trade_register_number: item.commerce_registry_number,
            phone: item.phone,
            fax: item.fax,
            postal_code: item.postal_code,
            authorization_act: item.act,
            registration_status: item.registration_status,
            cult_start_date: item.cult_since,
            cult_end_date: item.cult_until,
            is_active: item.is_active,
        }
    }
}

// =============================================================================
// Farmer Response Types
// =============================================================================

/// Response from the Farmer API.
#[cfg(feature = "farmers_api")]
#[pyclass(name = "FarmerResponse")]
#[derive(Clone, Debug)]
pub struct PyFarmerResponse {
    #[pyo3(get)]
    pub status: usize,
    #[pyo3(get)]
    pub message: String,
    #[pyo3(get)]
    pub data: Vec<PyFarmerResponseItem>,
    #[pyo3(get)]
    pub not_found: Vec<usize>,
}

#[cfg(feature = "farmers_api")]
#[pymethods]
impl PyFarmerResponse {
    fn __repr__(&self) -> String {
        format!(
            "FarmerResponse(found={}, not_found={})",
            self.data.len(),
            self.not_found.len()
        )
    }

    fn __len__(&self) -> usize {
        self.data.len()
    }
}

#[cfg(feature = "farmers_api")]
impl From<FarmerResponse> for PyFarmerResponse {
    fn from(resp: FarmerResponse) -> Self {
        Self {
            status: resp.status,
            message: resp.message,
            data: resp.data.into_iter().map(PyFarmerResponseItem::from).collect(),
            not_found: resp.not_found,
        }
    }
}

/// Information about a registered farmer/agricultural producer.
#[cfg(feature = "farmers_api")]
#[pyclass(name = "FarmerResponseItem")]
#[derive(Clone, Debug)]
pub struct PyFarmerResponseItem {
    #[pyo3(get)]
    pub cui: usize,
    #[pyo3(get)]
    pub query_date: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub address: String,
    #[pyo3(get)]
    pub trade_register_number: String,
    #[pyo3(get)]
    pub phone: String,
    #[pyo3(get)]
    pub fax: String,
    #[pyo3(get)]
    pub postal_code: String,
    #[pyo3(get)]
    pub authorization_act: String,
    #[pyo3(get)]
    pub registration_status: String,
    #[pyo3(get)]
    pub farmer_start_date: String,
    #[pyo3(get)]
    pub farmer_end_date: Option<String>,
    #[pyo3(get)]
    pub is_active: bool,
}

#[cfg(feature = "farmers_api")]
#[pymethods]
impl PyFarmerResponseItem {
    fn __repr__(&self) -> String {
        format!(
            "FarmerResponseItem(cui={}, name='{}', is_active={})",
            self.cui, self.name, self.is_active
        )
    }
}

#[cfg(feature = "farmers_api")]
impl From<FarmerResponseItem> for PyFarmerResponseItem {
    fn from(item: FarmerResponseItem) -> Self {
        Self {
            cui: item.unique_registration_code,
            query_date: item.when,
            name: item.name,
            address: item.address,
            trade_register_number: item.commerce_registry_number,
            phone: item.phone,
            fax: item.fax,
            postal_code: item.postal_code,
            authorization_act: item.act,
            registration_status: item.registration_status,
            farmer_start_date: item.farmer_since,
            farmer_end_date: item.farmer_until,
            is_active: item.is_active,
        }
    }
}

// =============================================================================
// Balance Response Types
// =============================================================================

/// Response from the Balance API.
#[cfg(feature = "balance_api")]
#[pyclass(name = "BalanceResponse")]
#[derive(Clone, Debug)]
pub struct PyBalanceResponse {
    #[pyo3(get)]
    pub kind: String,
    #[pyo3(get)]
    pub year: usize,
    #[pyo3(get)]
    pub cui: usize,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub caen_code: usize,
    #[pyo3(get)]
    pub activity_name: String,
    #[pyo3(get)]
    pub balance: HashMap<String, PyRawBalance>,
}

#[cfg(feature = "balance_api")]
#[pymethods]
impl PyBalanceResponse {
    fn __repr__(&self) -> String {
        format!(
            "BalanceResponse(cui={}, name='{}', year={}, kind='{}')",
            self.cui, self.name, self.year, self.kind
        )
    }

    /// Check if this is a company balance.
    pub fn is_company(&self) -> bool {
        self.kind == "Company"
    }

    /// Check if this is an NGO balance.
    pub fn is_ngo(&self) -> bool {
        self.kind == "Ngo"
    }
}

#[cfg(feature = "balance_api")]
impl From<BalanceResponse> for PyBalanceResponse {
    fn from(resp: BalanceResponse) -> Self {
        let kind = match resp.kind {
            crate::balance::EntityKind::Company => "Company",
            crate::balance::EntityKind::Ngo => "Ngo",
        };

        let balance: HashMap<String, PyRawBalance> = resp
            .balance
            .into_inner()
            .into_iter()
            .map(|(k, v)| (k.to_string(), PyRawBalance::from(v)))
            .collect();

        Self {
            kind: kind.to_string(),
            year: resp.year,
            cui: resp.unique_registration_code,
            name: resp.name,
            caen_code: resp.activity_code,
            activity_name: resp.activity_name,
            balance,
        }
    }
}

/// Raw balance indicator data.
#[cfg(feature = "balance_api")]
#[pyclass(name = "RawBalance")]
#[derive(Clone, Debug)]
pub struct PyRawBalance {
    #[pyo3(get)]
    pub code: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub value: isize,
}

#[cfg(feature = "balance_api")]
#[pymethods]
impl PyRawBalance {
    fn __repr__(&self) -> String {
        format!(
            "RawBalance(code='{}', value={})",
            self.code, self.value
        )
    }
}

#[cfg(feature = "balance_api")]
impl From<RawBalance> for PyRawBalance {
    fn from(balance: RawBalance) -> Self {
        Self {
            code: balance.code,
            name: balance.name,
            value: balance.value,
        }
    }
}
