//! Response structures for ANAF VAT Payer (PlatitorTvaRest) API V9.
//!
//! These structures model the response from ANAF's fiscal entity registry,
//! including VAT registration, inactivity status, split VAT, and address details.
//!
//! Reference: https://static.anaf.ro/static/10/Anaf/Informatii_R/Servicii_web/doc_WS_V9.txt

use serde::{Deserialize, Serialize};

use crate::ApiResponse;

#[cfg(feature = "vat_payer_async_api")]
pub type VatPayerAsyncResponse = crate::AsyncApiResponse<VatPayerAsyncToken>;

#[cfg(feature = "vat_payer_async_api")]
#[derive(Debug, Deserialize)]
pub struct VatPayerAsyncToken {
    #[serde(alias = "correlationId")]
    pub id: String,
}

#[cfg(feature = "vat_payer_async_api")]
impl std::fmt::Display for crate::vat_payer::VatPayerAsyncToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// Response type for VAT Payer API queries.
pub type VatPayerResponse = ApiResponse<TaxPayerEntity>;

// =============================================================================
// Root Entity Structure
// =============================================================================

/// Root structure for a single found entity in ANAF PlatitorTvaRest response.
/// 
/// Represents a Romanian tax payee (e.g., company, PFA, or other fiscal entity).
/// This structure contains all fiscal and administrative details about an entity
/// registered with ANAF (Romanian National Agency for Fiscal Administration).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaxPayerEntity {
    /// General data about the entity (name, CUI, registration info).
    #[serde(alias = "date_generale")]
    pub general_data: GeneralData,

    /// VAT (TVA) registration details and history.
    #[serde(alias = "inregistrare_scop_Tva")]
    pub vat_registration: VatRegistration,

    /// VAT at collection (RTVAI) details - cash-based VAT system.
    #[serde(alias = "inregistrare_RTVAI")]
    pub vat_collection: VatCollection,

    /// Inactivity/reactivation status.
    #[serde(alias = "stare_inactiv")]
    pub inactivity_status: InactivityStatus,

    /// Split VAT payment system details.
    #[serde(alias = "inregistrare_SplitTVA")]
    pub split_vat: SplitVat,

    /// Social headquarters (sediu social) address.
    #[serde(alias = "adresa_sediu_social")]
    pub headquarters_address: HeadquartersAddress,

    /// Fiscal domicile (domiciliu fiscal) address.
    #[serde(alias = "adresa_domiciliu_fiscal")]
    pub fiscal_address: FiscalAddress,
}

// =============================================================================
// General Data
// =============================================================================

/// General entity details from ANAF registry.
/// 
/// Contains core identification information including CUI (fiscal code),
/// name, registration details, and fiscal authority assignment.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralData {
    /// Fiscal identification code (Cod Unic de Înregistrare).
    #[serde(alias = "cui")]
    pub cui: u64,

    /// Query date (YYYY-MM-DD format).
    #[serde(alias = "data")]
    pub query_date: String,

    /// Entity legal name.
    #[serde(alias = "denumire")]
    pub name: String,

    /// Full address string.
    #[serde(alias = "adresa")]
    pub address: String,

    /// Trade register number (e.g., "F40/8875/2023").
    #[serde(alias = "nrRegCom", default)]
    pub trade_register_number: String,

    /// Phone number (may be empty).
    #[serde(alias = "telefon", default)]
    pub phone: String,

    /// Fax number (may be empty).
    #[serde(alias = "fax", default)]
    pub fax: String,

    /// Postal code.
    #[serde(alias = "codPostal", default)]
    pub postal_code: String,

    /// Authorization act (may be empty).
    #[serde(alias = "act", default)]
    pub authorization_act: String,

    /// Registration status (e.g., "INREGISTRAT din data 28.11.2023").
    #[serde(alias = "stare_inregistrare", default)]
    pub registration_status: String,

    /// Registration date (YYYY-MM-DD format).
    #[serde(alias = "data_inregistrare", default)]
    pub registration_date: String,

    /// CAEN activity code (e.g., "6201" for software development).
    #[serde(alias = "cod_CAEN", default)]
    pub caen_code: String,

    /// IBAN account for split TVA payments (may be empty).
    #[serde(alias = "iban", default)]
    pub iban: String,

    /// RO e-Factura registry status.
    #[serde(alias = "statusRO_e_Factura", default)]
    pub ro_efactura_status: bool,

    /// RO e-Factura registration date (may be empty).
    #[serde(alias = "data_inreg_Reg_RO_e_Factura", default)]
    pub ro_efactura_registration_date: String,

    /// Competent fiscal authority (e.g., "Administrația Județeană a Finanțelor Publice Timiș").
    #[serde(alias = "organFiscalCompetent", default)]
    pub fiscal_authority: String,

    /// Ownership form (may be empty).
    #[serde(alias = "forma_de_proprietate", default)]
    pub ownership_form: String,

    /// Organization form (may be empty).
    #[serde(alias = "forma_organizare", default)]
    pub organization_form: String,

    /// Legal/juridical form (may be empty).
    #[serde(alias = "forma_juridica", default)]
    pub legal_form: String,
}

// =============================================================================
// VAT Registration (TVA Purposes)
// =============================================================================

/// VAT (TVA) registration for fiscal purposes.
/// 
/// Tracks whether an entity is registered for VAT purposes under
/// art. 316 of the Romanian Fiscal Code, including historical periods.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VatRegistration {
    /// True if currently registered for VAT purposes.
    #[serde(alias = "scpTVA")]
    pub is_registered: bool,

    /// List of VAT registration periods (can include historical data).
    #[serde(alias = "perioade_TVA", default)]
    pub periods: Vec<VatPeriod>,
}

/// Single VAT registration period.
/// 
/// Represents a time interval during which the entity was registered
/// for VAT purposes, including any annulment information.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct VatPeriod {
    /// Start date of VAT registration (YYYY-MM-DD).
    #[serde(alias = "data_inceput_ScpTVA", default)]
    pub start_date: String,

    /// End date of VAT registration (YYYY-MM-DD), empty if still active.
    #[serde(alias = "data_sfarsit_ScpTVA", default)]
    pub end_date: String,

    /// Date when VAT annulment was processed (YYYY-MM-DD).
    #[serde(alias = "data_anul_imp_ScpTVA", default)]
    pub annulment_date: String,

    /// Legal basis/message for VAT registration annulment.
    #[serde(alias = "mesaj_ScpTVA", default)]
    pub annulment_message: String,
}

// =============================================================================
// VAT at Collection (RTVAI) - Cash-based VAT
// =============================================================================

/// VAT at collection (TVA la încasare) details.
/// 
/// This system allows certain entities to pay VAT only when
/// payment is received, rather than when the invoice is issued.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VatCollection {
    /// True if applying VAT at collection system.
    #[serde(alias = "statusTvaIncasare")]
    pub is_applied: bool,

    /// Start date for VAT at collection (YYYY-MM-DD).
    #[serde(alias = "dataInceputTvaInc", default)]
    pub start_date: String,

    /// End date for VAT at collection (YYYY-MM-DD).
    #[serde(alias = "dataSfarsitTvaInc", default)]
    pub end_date: String,

    /// Type of update/action.
    #[serde(alias = "tipActTvaInc", default)]
    pub update_type: String,

    /// Last update date (YYYY-MM-DD).
    #[serde(alias = "dataActualizareTvaInc", default)]
    pub update_date: String,

    /// Publication date (YYYY-MM-DD).
    #[serde(alias = "dataPublicareTvaInc", default)]
    pub publication_date: String,
}

// =============================================================================
// Inactivity Status
// =============================================================================

/// Inactivity and reactivation status.
/// 
/// Tracks whether an entity has been marked as inactive by ANAF,
/// including dates for inactivation, reactivation, and deregistration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InactivityStatus {
    /// True if entity is currently inactive.
    #[serde(alias = "statusInactivi")]
    pub is_inactive: bool,

    /// Date when entity was marked inactive (YYYY-MM-DD).
    #[serde(alias = "dataInactivare", default)]
    pub inactivation_date: String,

    /// Date when entity was reactivated (YYYY-MM-DD).
    #[serde(alias = "dataReactivare", default)]
    pub reactivation_date: String,

    /// Date when status was published (YYYY-MM-DD).
    #[serde(alias = "dataPublicare", default)]
    pub publication_date: String,

    /// Date of deregistration/erasure (YYYY-MM-DD).
    #[serde(alias = "dataRadiere", default)]
    pub deregistration_date: String,
}

// =============================================================================
// Split VAT
// =============================================================================

/// Split VAT payment system details.
/// 
/// The split VAT mechanism requires separate payment of VAT
/// directly to a dedicated tax account.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SplitVat {
    /// True if applying split VAT payment system.
    #[serde(alias = "statusSplitTVA")]
    pub is_applied: bool,

    /// Start date for split VAT application (YYYY-MM-DD).
    #[serde(alias = "dataInceputSplitTVA", default)]
    pub start_date: String,

    /// Date when split VAT was cancelled (YYYY-MM-DD).
    #[serde(alias = "dataAnulareSplitTVA", default)]
    pub cancellation_date: String,
}

// =============================================================================
// Addresses
// =============================================================================

/// Social headquarters (sediu social) address.
/// 
/// The registered office address of the entity.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct HeadquartersAddress {
    /// Country name.
    #[serde(alias = "stara", default)]
    pub country: String,

    /// County/district name (e.g., "MUNICIPIUL BUCUREȘTI").
    #[serde(alias = "sdenumire_Judet", default)]
    pub county: String,

    /// County code (e.g., "40").
    #[serde(alias = "scod_Judet", default)]
    pub county_code: String,

    /// County auto code (e.g., "B" for București).
    #[serde(alias = "scod_JudetAuto", default)]
    pub county_auto_code: String,

    /// Locality/city name (e.g., "Sector 1 Mun. București").
    #[serde(alias = "sdenumire_Localitate", default)]
    pub locality: String,

    /// Locality code.
    #[serde(alias = "scod_Localitate", default)]
    pub locality_code: String,

    /// Street name.
    #[serde(alias = "sdenumire_Strada", default)]
    pub street: String,

    /// Street number.
    #[serde(alias = "snumar_Strada", default)]
    pub street_number: String,

    /// Additional address details (e.g., "PARTER", "AP.5").
    #[serde(alias = "sdetalii_Adresa", default)]
    pub details: String,

    /// Postal code.
    #[serde(alias = "scod_Postal", default)]
    pub postal_code: String,
}

impl HeadquartersAddress {
    /// Returns a formatted full address string.
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

/// Fiscal domicile (domiciliu fiscal) address.
/// 
/// The address where the entity conducts its main fiscal activities.
/// May differ from headquarters address.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct FiscalAddress {
    /// Country name.
    #[serde(alias = "dtara", default)]
    pub country: String,

    /// County/district name.
    #[serde(alias = "ddenumire_Judet", default)]
    pub county: String,

    /// County code.
    #[serde(alias = "dcod_Judet", default)]
    pub county_code: String,

    /// County auto code (e.g., "B" for București).
    #[serde(alias = "dcod_JudetAuto", default)]
    pub county_auto_code: String,

    /// Locality/city name.
    #[serde(alias = "ddenumire_Localitate", default)]
    pub locality: String,

    /// Locality code.
    #[serde(alias = "dcod_Localitate", default)]
    pub locality_code: String,

    /// Street name.
    #[serde(alias = "ddenumire_Strada", default)]
    pub street: String,

    /// Street number.
    #[serde(alias = "dnumar_Strada", default)]
    pub street_number: String,

    /// Additional address details.
    #[serde(alias = "ddetalii_Adresa", default)]
    pub details: String,

    /// Postal code.
    #[serde(alias = "dcod_Postal", default)]
    pub postal_code: String,
}

impl FiscalAddress {
    /// Returns a formatted full address string.
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
