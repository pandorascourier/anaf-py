use serde::{Deserialize, Serialize};

/// Root structure for a single found entity in ANAF PlatitorTvaRest response.
/// Represents a Romanian tax payee (e.g., company, PFA, or other fiscal entity).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxPayeeEntity {
    /// General data about the entity.
    pub date_generale: DateGenerale,
    /// TVA (VAT) registration details.
    pub inregistrare_scop_tva: InregistrareScopTva,
    /// TVA at collection (RTVAI) details.
    pub inregistrare_rtvai: InregistrareRtvai,
    /// Inactivity status.
    pub stare_inactiv: StareInactiv,
    /// Split TVA (payment splitting) details.
    pub inregistrare_split_tva: InregistrareSplitTva,
    /// Social headquarters address.
    pub adresa_sediu_social: AdresaSediuSocial,
    /// Fiscal domicile address.
    pub adresa_domiciliu_fiscal: AdresaDomiciliuFiscal,
}

/// General entity details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateGenerale {
    /// Query date (YYYY-MM-DD).
    pub data: String,
    /// Fiscal identification code (CUI).
    pub cui: u64,
    /// Entity name.
    pub denumire: String,
    /// Full address string.
    pub adresa: String,
    /// Phone number (may be empty).
    pub telefon: String,
    /// Fax number (may be empty).
    pub fax: String,
    /// Postal code.
    pub cod_postal: String,
    /// Authorization act (may be empty).
    pub act: String,
    /// Registration status (e.g., "INREGISTRAT din data 28.11.2023").
    pub stare_inregistrare: String,
    /// RO e-Factura registration date (may be empty).
    pub data_inreg_reg_ro_e_factura: String,
    /// Competent fiscal authority.
    pub organ_fiscal_competent: String,
    /// Ownership form (may be empty).
    pub forma_de_proprietate: String,
    /// Organization form (may be empty).
    pub forma_organizare: String,
    /// Legal form (may be empty).
    pub forma_juridica: String,
    /// Trade register number.
    pub nr_reg_com: String,
    /// CAEN activity code.
    pub cod_caen: String,
    /// IBAN account (for split TVA; may be empty).
    pub iban: String,
    /// RO e-Factura status.
    pub status_ro_e_factura: bool,
    /// Registration date (YYYY-MM-DD).
    pub data_inregistrare: String,
}

/// TVA (VAT) registration for purposes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InregistrareScopTva {
    /// True if registered for TVA purposes.
    pub scp_tva: bool,
    /// List of TVA periods (can be multiple historical periods).
    pub perioade_tva: Vec<PerioadaTva>,
}

/// Single TVA period details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerioadaTva {
    /// Start date of TVA registration (YYYY-MM-DD).
    #[serde(default)]
    pub data_inceput_scp_tva: String,
    /// End date of TVA registration (YYYY-MM-DD).
    #[serde(default)]
    pub data_sfarsit_scp_tva: String,
    /// Date of TVA annulment operation (YYYY-MM-DD).
    #[serde(default)]
    pub data_anul_imp_scp_tva: String,
    /// Legal basis for annulment.
    #[serde(default)]
    pub mesaj_scp_tva: String,
}

/// TVA at collection (RTVAI) details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InregistrareRtvai {
    /// Start date for TVA at collection (YYYY-MM-DD).
    pub data_inceput_tva_inc: String,
    /// End date for TVA at collection (YYYY-MM-DD).
    pub data_sfarsit_tva_inc: String,
    /// Update type.
    pub tip_act_tva_inc: String,
    /// True if applying TVA at collection.
    pub status_tva_incasare: bool,
    /// Update date (YYYY-MM-DD).
    pub data_actualizare_tva_inc: String,
    /// Publication date (YYYY-MM-DD).
    pub data_publicare_tva_inc: String,
}

/// Inactivity status.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StareInactiv {
    /// Inactivation date (YYYY-MM-DD).
    pub data_inactivare: String,
    /// Reactivation date (YYYY-MM-DD).
    pub data_reactivare: String,
    /// Publication date (YYYY-MM-DD).
    pub data_publicare: String,
    /// Deregistration date (YYYY-MM-DD).
    pub data_radiere: String,
    /// True if inactive.
    pub status_inactivi: bool,
}

/// Split TVA details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InregistrareSplitTva {
    /// Start date for split TVA (YYYY-MM-DD).
    pub data_inceput_split_tva: String,
    /// Annulment date for split TVA (YYYY-MM-DD).
    pub data_anulare_split_tva: String,
    /// True if applying split TVA.
    pub status_split_tva: bool,
}

/// Social headquarters address.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdresaSediuSocial {
    /// Country.
    pub stara: String,
    /// Locality name.
    pub sdenumire_localitate: String,
    /// Street name.
    pub sdenumire_strada: String,
    /// Street number.
    pub snumar_strada: String,
    /// Locality code.
    pub scod_localitate: String,
    /// County name.
    pub sdenumire_judet: String,
    /// County code.
    pub scod_judet: String,
    /// County auto code (e.g., "B" for București).
    pub scod_judet_auto: String,
    /// Address details (e.g., "PARTER").
    pub sdetalii_adresa: String,
    /// Postal code.
    pub scod_postal: String,
}

/// Fiscal domicile address.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdresaDomiciliuFiscal {
    /// Country.
    pub dtara: String,
    /// Locality code.
    pub dcod_localitate: String,
    /// Street name.
    pub ddenumire_strada: String,
    /// Street number.
    pub dnumar_strada: String,
    /// Locality name.
    pub ddenumire_localitate: String,
    /// County name.
    pub ddenumire_judet: String,
    /// County code.
    pub dcod_judet: String,
    /// County auto code (e.g., "B" for București).
    pub dcod_judet_auto: String,
    /// Address details (e.g., "PARTER").
    pub ddetalii_adresa: String,
    /// Postal code.
    pub dcod_postal: String,
}

/// Example usage for deserializing the "found" item from the payload:
/// let entity: TaxPayeeEntity = serde_json::from_str(payload_found_item)?;