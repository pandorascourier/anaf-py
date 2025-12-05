//! PHP wrappers for ANAF API responses.
//!
//! This module provides PHP-accessible classes for all response types
//! from ANAF APIs, with comprehensive documentation and type hints.

use ext_php_rs::prelude::*;
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

#[php_class]
#[php(name = "Anaf\\VatPayerResponse")]
#[derive(Clone, Debug)]
pub struct PhpVatPayerResponse {
    pub status: u64,
    pub message: String,
    pub data: Vec<PhpTaxPayerEntity>,
    pub not_found: Vec<u64>,
}

#[php_impl]
impl PhpVatPayerResponse {
    #[php(getter)]
    pub fn get_status(&self) -> u64 { self.status }
    
    #[php(getter)]
    pub fn get_message(&self) -> String { self.message.clone() }
    
    #[php(getter)]
    pub fn get_data(&self) -> Vec<PhpTaxPayerEntity> { self.data.clone() }
    
    #[php(getter)]
    pub fn get_not_found(&self) -> Vec<u64> { self.not_found.clone() }

    pub fn count(&self) -> u64 { self.data.len() as u64 }

    pub fn first(&self) -> Option<PhpTaxPayerEntity> { self.data.first().cloned() }
}

impl From<VatPayerResponse> for PhpVatPayerResponse {
    fn from(resp: VatPayerResponse) -> Self {
        Self {
            status: resp.status as u64,
            message: resp.message,
            data: resp.data.into_iter().map(PhpTaxPayerEntity::from).collect(),
            not_found: resp.not_found.into_iter().map(|x| x as u64).collect(),
        }
    }
}

// =============================================================================
// Tax Payer Entity
// =============================================================================

#[php_class]
#[php(name = "Anaf\\TaxPayerEntity")]
#[derive(Clone, Debug)]
pub struct PhpTaxPayerEntity {
    pub general_data: PhpGeneralData,
    pub vat_registration: PhpVatRegistration,
    pub vat_collection: PhpVatCollection,
    pub inactivity_status: PhpInactivityStatus,
    pub split_vat: PhpSplitVat,
    pub headquarters_address: PhpHeadquartersAddress,
    pub fiscal_address: PhpFiscalAddress,
}

#[php_impl]
impl PhpTaxPayerEntity {
    #[php(getter)]
    pub fn get_general_data(&self) -> PhpGeneralData { self.general_data.clone() }
    
    #[php(getter)]
    pub fn get_vat_registration(&self) -> PhpVatRegistration { self.vat_registration.clone() }
    
    #[php(getter)]
    pub fn get_vat_collection(&self) -> PhpVatCollection { self.vat_collection.clone() }
    
    #[php(getter)]
    pub fn get_inactivity_status(&self) -> PhpInactivityStatus { self.inactivity_status.clone() }
    
    #[php(getter)]
    pub fn get_split_vat(&self) -> PhpSplitVat { self.split_vat.clone() }
    
    #[php(getter)]
    pub fn get_headquarters_address(&self) -> PhpHeadquartersAddress { self.headquarters_address.clone() }
    
    #[php(getter)]
    pub fn get_fiscal_address(&self) -> PhpFiscalAddress { self.fiscal_address.clone() }

    pub fn is_vat_payer(&self) -> bool { self.vat_registration.is_registered }
    pub fn is_inactive(&self) -> bool { self.inactivity_status.is_inactive }
    pub fn uses_split_vat(&self) -> bool { self.split_vat.is_applied }
    pub fn in_ro_efactura(&self) -> bool { self.general_data.ro_efactura_status }
}

impl From<TaxPayerEntity> for PhpTaxPayerEntity {
    fn from(entity: TaxPayerEntity) -> Self {
        Self {
            general_data: PhpGeneralData::from(entity.general_data),
            vat_registration: PhpVatRegistration::from(entity.vat_registration),
            vat_collection: PhpVatCollection::from(entity.vat_collection),
            inactivity_status: PhpInactivityStatus::from(entity.inactivity_status),
            split_vat: PhpSplitVat::from(entity.split_vat),
            headquarters_address: PhpHeadquartersAddress::from(entity.headquarters_address),
            fiscal_address: PhpFiscalAddress::from(entity.fiscal_address),
        }
    }
}

// =============================================================================
// General Data
// =============================================================================

#[php_class]
#[php(name = "Anaf\\GeneralData")]
#[derive(Clone, Debug)]
pub struct PhpGeneralData {
    pub cui: u64,
    pub query_date: String,
    pub name: String,
    pub address: String,
    pub trade_register_number: String,
    pub phone: String,
    pub fax: String,
    pub postal_code: String,
    pub authorization_act: String,
    pub registration_status: String,
    pub registration_date: String,
    pub caen_code: String,
    pub iban: String,
    pub ro_efactura_status: bool,
    pub ro_efactura_registration_date: String,
    pub fiscal_authority: String,
    pub ownership_form: String,
    pub organization_form: String,
    pub legal_form: String,
}

#[php_impl]
impl PhpGeneralData {
    #[php(getter)]
    pub fn get_cui(&self) -> u64 { self.cui }
    #[php(getter)]
    pub fn get_query_date(&self) -> String { self.query_date.clone() }
    #[php(getter)]
    pub fn get_name(&self) -> String { self.name.clone() }
    #[php(getter)]
    pub fn get_address(&self) -> String { self.address.clone() }
    #[php(getter)]
    pub fn get_trade_register_number(&self) -> String { self.trade_register_number.clone() }
    #[php(getter)]
    pub fn get_phone(&self) -> String { self.phone.clone() }
    #[php(getter)]
    pub fn get_fax(&self) -> String { self.fax.clone() }
    #[php(getter)]
    pub fn get_postal_code(&self) -> String { self.postal_code.clone() }
    #[php(getter)]
    pub fn get_authorization_act(&self) -> String { self.authorization_act.clone() }
    #[php(getter)]
    pub fn get_registration_status(&self) -> String { self.registration_status.clone() }
    #[php(getter)]
    pub fn get_registration_date(&self) -> String { self.registration_date.clone() }
    #[php(getter)]
    pub fn get_caen_code(&self) -> String { self.caen_code.clone() }
    #[php(getter)]
    pub fn get_iban(&self) -> String { self.iban.clone() }
    #[php(getter)]
    pub fn get_ro_efactura_status(&self) -> bool { self.ro_efactura_status }
    #[php(getter)]
    pub fn get_ro_efactura_registration_date(&self) -> String { self.ro_efactura_registration_date.clone() }
    #[php(getter)]
    pub fn get_fiscal_authority(&self) -> String { self.fiscal_authority.clone() }
    #[php(getter)]
    pub fn get_ownership_form(&self) -> String { self.ownership_form.clone() }
    #[php(getter)]
    pub fn get_organization_form(&self) -> String { self.organization_form.clone() }
    #[php(getter)]
    pub fn get_legal_form(&self) -> String { self.legal_form.clone() }
}

impl From<GeneralData> for PhpGeneralData {
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

#[php_class]
#[php(name = "Anaf\\VatRegistration")]
#[derive(Clone, Debug)]
pub struct PhpVatRegistration {
    pub is_registered: bool,
    pub periods: Vec<PhpVatPeriod>,
}

#[php_impl]
impl PhpVatRegistration {
    #[php(getter)]
    pub fn get_is_registered(&self) -> bool { self.is_registered }
    #[php(getter)]
    pub fn get_periods(&self) -> Vec<PhpVatPeriod> { self.periods.clone() }

    pub fn current_period(&self) -> Option<PhpVatPeriod> { self.periods.first().cloned() }
}

impl From<VatRegistration> for PhpVatRegistration {
    fn from(reg: VatRegistration) -> Self {
        Self {
            is_registered: reg.is_registered,
            periods: reg.periods.into_iter().map(PhpVatPeriod::from).collect(),
        }
    }
}

// =============================================================================
// VAT Period
// =============================================================================

#[php_class]
#[php(name = "Anaf\\VatPeriod")]
#[derive(Clone, Debug)]
pub struct PhpVatPeriod {
    pub start_date: String,
    pub end_date: String,
    pub annulment_date: String,
    pub annulment_message: String,
}

#[php_impl]
impl PhpVatPeriod {
    #[php(getter)]
    pub fn get_start_date(&self) -> String { self.start_date.clone() }
    #[php(getter)]
    pub fn get_end_date(&self) -> String { self.end_date.clone() }
    #[php(getter)]
    pub fn get_annulment_date(&self) -> String { self.annulment_date.clone() }
    #[php(getter)]
    pub fn get_annulment_message(&self) -> String { self.annulment_message.clone() }

    pub fn is_active(&self) -> bool { self.end_date.is_empty() }
}

impl From<VatPeriod> for PhpVatPeriod {
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
// VAT Collection
// =============================================================================

#[php_class]
#[php(name = "Anaf\\VatCollection")]
#[derive(Clone, Debug)]
pub struct PhpVatCollection {
    pub is_applied: bool,
    pub start_date: String,
    pub end_date: String,
    pub update_type: String,
    pub update_date: String,
    pub publication_date: String,
}

#[php_impl]
impl PhpVatCollection {
    #[php(getter)]
    pub fn get_is_applied(&self) -> bool { self.is_applied }
    #[php(getter)]
    pub fn get_start_date(&self) -> String { self.start_date.clone() }
    #[php(getter)]
    pub fn get_end_date(&self) -> String { self.end_date.clone() }
    #[php(getter)]
    pub fn get_update_type(&self) -> String { self.update_type.clone() }
    #[php(getter)]
    pub fn get_update_date(&self) -> String { self.update_date.clone() }
    #[php(getter)]
    pub fn get_publication_date(&self) -> String { self.publication_date.clone() }
}

impl From<VatCollection> for PhpVatCollection {
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

#[php_class]
#[php(name = "Anaf\\InactivityStatus")]
#[derive(Clone, Debug)]
pub struct PhpInactivityStatus {
    pub is_inactive: bool,
    pub inactivation_date: String,
    pub reactivation_date: String,
    pub publication_date: String,
    pub deregistration_date: String,
}

#[php_impl]
impl PhpInactivityStatus {
    #[php(getter)]
    pub fn get_is_inactive(&self) -> bool { self.is_inactive }
    #[php(getter)]
    pub fn get_inactivation_date(&self) -> String { self.inactivation_date.clone() }
    #[php(getter)]
    pub fn get_reactivation_date(&self) -> String { self.reactivation_date.clone() }
    #[php(getter)]
    pub fn get_publication_date(&self) -> String { self.publication_date.clone() }
    #[php(getter)]
    pub fn get_deregistration_date(&self) -> String { self.deregistration_date.clone() }

    pub fn is_deregistered(&self) -> bool { !self.deregistration_date.is_empty() }
}

impl From<InactivityStatus> for PhpInactivityStatus {
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

#[php_class]
#[php(name = "Anaf\\SplitVat")]
#[derive(Clone, Debug)]
pub struct PhpSplitVat {
    pub is_applied: bool,
    pub start_date: String,
    pub cancellation_date: String,
}

#[php_impl]
impl PhpSplitVat {
    #[php(getter)]
    pub fn get_is_applied(&self) -> bool { self.is_applied }
    #[php(getter)]
    pub fn get_start_date(&self) -> String { self.start_date.clone() }
    #[php(getter)]
    pub fn get_cancellation_date(&self) -> String { self.cancellation_date.clone() }
}

impl From<SplitVat> for PhpSplitVat {
    fn from(sv: SplitVat) -> Self {
        Self {
            is_applied: sv.is_applied,
            start_date: sv.start_date,
            cancellation_date: sv.cancellation_date,
        }
    }
}

// =============================================================================
// Headquarters Address
// =============================================================================

#[php_class]
#[php(name = "Anaf\\HeadquartersAddress")]
#[derive(Clone, Debug)]
pub struct PhpHeadquartersAddress {
    pub country: String,
    pub county: String,
    pub county_code: String,
    pub county_auto_code: String,
    pub locality: String,
    pub locality_code: String,
    pub street: String,
    pub street_number: String,
    pub details: String,
    pub postal_code: String,
}

#[php_impl]
impl PhpHeadquartersAddress {
    #[php(getter)]
    pub fn get_country(&self) -> String { self.country.clone() }
    #[php(getter)]
    pub fn get_county(&self) -> String { self.county.clone() }
    #[php(getter)]
    pub fn get_county_code(&self) -> String { self.county_code.clone() }
    #[php(getter)]
    pub fn get_county_auto_code(&self) -> String { self.county_auto_code.clone() }
    #[php(getter)]
    pub fn get_locality(&self) -> String { self.locality.clone() }
    #[php(getter)]
    pub fn get_locality_code(&self) -> String { self.locality_code.clone() }
    #[php(getter)]
    pub fn get_street(&self) -> String { self.street.clone() }
    #[php(getter)]
    pub fn get_street_number(&self) -> String { self.street_number.clone() }
    #[php(getter)]
    pub fn get_details(&self) -> String { self.details.clone() }
    #[php(getter)]
    pub fn get_postal_code(&self) -> String { self.postal_code.clone() }

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
        if !self.details.is_empty() { parts.push(self.details.clone()); }
        if !self.locality.is_empty() { parts.push(self.locality.clone()); }
        if !self.county.is_empty() && self.county != self.locality { parts.push(self.county.clone()); }
        if !self.postal_code.is_empty() { parts.push(self.postal_code.clone()); }
        if !self.country.is_empty() { parts.push(self.country.clone()); }
        parts.join(", ")
    }
}

impl From<HeadquartersAddress> for PhpHeadquartersAddress {
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

// =============================================================================
// Fiscal Address
// =============================================================================

#[php_class]
#[php(name = "Anaf\\FiscalAddress")]
#[derive(Clone, Debug)]
pub struct PhpFiscalAddress {
    pub country: String,
    pub county: String,
    pub county_code: String,
    pub county_auto_code: String,
    pub locality: String,
    pub locality_code: String,
    pub street: String,
    pub street_number: String,
    pub details: String,
    pub postal_code: String,
}

#[php_impl]
impl PhpFiscalAddress {
    #[php(getter)]
    pub fn get_country(&self) -> String { self.country.clone() }
    #[php(getter)]
    pub fn get_county(&self) -> String { self.county.clone() }
    #[php(getter)]
    pub fn get_county_code(&self) -> String { self.county_code.clone() }
    #[php(getter)]
    pub fn get_county_auto_code(&self) -> String { self.county_auto_code.clone() }
    #[php(getter)]
    pub fn get_locality(&self) -> String { self.locality.clone() }
    #[php(getter)]
    pub fn get_locality_code(&self) -> String { self.locality_code.clone() }
    #[php(getter)]
    pub fn get_street(&self) -> String { self.street.clone() }
    #[php(getter)]
    pub fn get_street_number(&self) -> String { self.street_number.clone() }
    #[php(getter)]
    pub fn get_details(&self) -> String { self.details.clone() }
    #[php(getter)]
    pub fn get_postal_code(&self) -> String { self.postal_code.clone() }

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
        if !self.details.is_empty() { parts.push(self.details.clone()); }
        if !self.locality.is_empty() { parts.push(self.locality.clone()); }
        if !self.county.is_empty() && self.county != self.locality { parts.push(self.county.clone()); }
        if !self.postal_code.is_empty() { parts.push(self.postal_code.clone()); }
        if !self.country.is_empty() { parts.push(self.country.clone()); }
        parts.join(", ")
    }
}

impl From<FiscalAddress> for PhpFiscalAddress {
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

#[cfg(feature = "cults_api")]
#[php_class]
#[php(name = "Anaf\\CultResponse")]
#[derive(Clone, Debug)]
pub struct PhpCultResponse {
    pub status: u64,
    pub message: String,
    pub data: Vec<PhpCultResponseItem>,
    pub not_found: Vec<u64>,
}

#[cfg(feature = "cults_api")]
#[php_impl]
impl PhpCultResponse {
    #[php(getter)]
    pub fn get_status(&self) -> u64 { self.status }
    #[php(getter)]
    pub fn get_message(&self) -> String { self.message.clone() }
    #[php(getter)]
    pub fn get_data(&self) -> Vec<PhpCultResponseItem> { self.data.clone() }
    #[php(getter)]
    pub fn get_not_found(&self) -> Vec<u64> { self.not_found.clone() }

    pub fn count(&self) -> u64 { self.data.len() as u64 }
}

#[cfg(feature = "cults_api")]
impl From<CultResponse> for PhpCultResponse {
    fn from(resp: CultResponse) -> Self {
        Self {
            status: resp.status as u64,
            message: resp.message,
            data: resp.data.into_iter().map(PhpCultResponseItem::from).collect(),
            not_found: resp.not_found.into_iter().map(|x| x as u64).collect(),
        }
    }
}

#[cfg(feature = "cults_api")]
#[php_class]
#[php(name = "Anaf\\CultResponseItem")]
#[derive(Clone, Debug)]
pub struct PhpCultResponseItem {
    pub cui: u64,
    pub query_date: String,
    pub name: String,
    pub address: String,
    pub trade_register_number: String,
    pub phone: String,
    pub fax: String,
    pub postal_code: String,
    pub authorization_act: String,
    pub registration_status: String,
    pub cult_start_date: String,
    pub cult_end_date: Option<String>,
    pub is_active: bool,
}

#[cfg(feature = "cults_api")]
#[php_impl]
impl PhpCultResponseItem {
    #[php(getter)]
    pub fn get_cui(&self) -> u64 { self.cui }
    #[php(getter)]
    pub fn get_query_date(&self) -> String { self.query_date.clone() }
    #[php(getter)]
    pub fn get_name(&self) -> String { self.name.clone() }
    #[php(getter)]
    pub fn get_address(&self) -> String { self.address.clone() }
    #[php(getter)]
    pub fn get_trade_register_number(&self) -> String { self.trade_register_number.clone() }
    #[php(getter)]
    pub fn get_phone(&self) -> String { self.phone.clone() }
    #[php(getter)]
    pub fn get_fax(&self) -> String { self.fax.clone() }
    #[php(getter)]
    pub fn get_postal_code(&self) -> String { self.postal_code.clone() }
    #[php(getter)]
    pub fn get_authorization_act(&self) -> String { self.authorization_act.clone() }
    #[php(getter)]
    pub fn get_registration_status(&self) -> String { self.registration_status.clone() }
    #[php(getter)]
    pub fn get_cult_start_date(&self) -> String { self.cult_start_date.clone() }
    #[php(getter)]
    pub fn get_cult_end_date(&self) -> Option<String> { self.cult_end_date.clone() }
    #[php(getter)]
    pub fn get_is_active(&self) -> bool { self.is_active }
}

#[cfg(feature = "cults_api")]
impl From<CultResponseItem> for PhpCultResponseItem {
    fn from(item: CultResponseItem) -> Self {
        Self {
            cui: item.unique_registration_code as u64,
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

#[cfg(feature = "farmers_api")]
#[php_class]
#[php(name = "Anaf\\FarmerResponse")]
#[derive(Clone, Debug)]
pub struct PhpFarmerResponse {
    pub status: u64,
    pub message: String,
    pub data: Vec<PhpFarmerResponseItem>,
    pub not_found: Vec<u64>,
}

#[cfg(feature = "farmers_api")]
#[php_impl]
impl PhpFarmerResponse {
    #[php(getter)]
    pub fn get_status(&self) -> u64 { self.status }
    #[php(getter)]
    pub fn get_message(&self) -> String { self.message.clone() }
    #[php(getter)]
    pub fn get_data(&self) -> Vec<PhpFarmerResponseItem> { self.data.clone() }
    #[php(getter)]
    pub fn get_not_found(&self) -> Vec<u64> { self.not_found.clone() }

    pub fn count(&self) -> u64 { self.data.len() as u64 }
}

#[cfg(feature = "farmers_api")]
impl From<FarmerResponse> for PhpFarmerResponse {
    fn from(resp: FarmerResponse) -> Self {
        Self {
            status: resp.status as u64,
            message: resp.message,
            data: resp.data.into_iter().map(PhpFarmerResponseItem::from).collect(),
            not_found: resp.not_found.into_iter().map(|x| x as u64).collect(),
        }
    }
}

#[cfg(feature = "farmers_api")]
#[php_class]
#[php(name = "Anaf\\FarmerResponseItem")]
#[derive(Clone, Debug)]
pub struct PhpFarmerResponseItem {
    pub cui: u64,
    pub query_date: String,
    pub name: String,
    pub address: String,
    pub trade_register_number: String,
    pub phone: String,
    pub fax: String,
    pub postal_code: String,
    pub authorization_act: String,
    pub registration_status: String,
    pub farmer_start_date: String,
    pub farmer_end_date: Option<String>,
    pub is_active: bool,
}

#[cfg(feature = "farmers_api")]
#[php_impl]
impl PhpFarmerResponseItem {
    #[php(getter)]
    pub fn get_cui(&self) -> u64 { self.cui }
    #[php(getter)]
    pub fn get_query_date(&self) -> String { self.query_date.clone() }
    #[php(getter)]
    pub fn get_name(&self) -> String { self.name.clone() }
    #[php(getter)]
    pub fn get_address(&self) -> String { self.address.clone() }
    #[php(getter)]
    pub fn get_trade_register_number(&self) -> String { self.trade_register_number.clone() }
    #[php(getter)]
    pub fn get_phone(&self) -> String { self.phone.clone() }
    #[php(getter)]
    pub fn get_fax(&self) -> String { self.fax.clone() }
    #[php(getter)]
    pub fn get_postal_code(&self) -> String { self.postal_code.clone() }
    #[php(getter)]
    pub fn get_authorization_act(&self) -> String { self.authorization_act.clone() }
    #[php(getter)]
    pub fn get_registration_status(&self) -> String { self.registration_status.clone() }
    #[php(getter)]
    pub fn get_farmer_start_date(&self) -> String { self.farmer_start_date.clone() }
    #[php(getter)]
    pub fn get_farmer_end_date(&self) -> Option<String> { self.farmer_end_date.clone() }
    #[php(getter)]
    pub fn get_is_active(&self) -> bool { self.is_active }
}

#[cfg(feature = "farmers_api")]
impl From<FarmerResponseItem> for PhpFarmerResponseItem {
    fn from(item: FarmerResponseItem) -> Self {
        Self {
            cui: item.unique_registration_code as u64,
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

#[cfg(feature = "balance_api")]
#[php_class]
#[php(name = "Anaf\\BalanceResponse")]
#[derive(Clone, Debug)]
pub struct PhpBalanceResponse {
    pub kind: String,
    pub year: u64,
    pub cui: u64,
    pub name: String,
    pub caen_code: u64,
    pub activity_name: String,
    pub balance: HashMap<String, PhpRawBalance>,
}

#[cfg(feature = "balance_api")]
#[php_impl]
impl PhpBalanceResponse {
    #[php(getter)]
    pub fn get_kind(&self) -> String { self.kind.clone() }
    #[php(getter)]
    pub fn get_year(&self) -> u64 { self.year }
    #[php(getter)]
    pub fn get_cui(&self) -> u64 { self.cui }
    #[php(getter)]
    pub fn get_name(&self) -> String { self.name.clone() }
    #[php(getter)]
    pub fn get_caen_code(&self) -> u64 { self.caen_code }
    #[php(getter)]
    pub fn get_activity_name(&self) -> String { self.activity_name.clone() }
    #[php(getter)]
    pub fn get_balance(&self) -> HashMap<String, PhpRawBalance> { self.balance.clone() }

    pub fn is_company(&self) -> bool { self.kind == "Company" }
    pub fn is_ngo(&self) -> bool { self.kind == "Ngo" }
}

#[cfg(feature = "balance_api")]
impl From<BalanceResponse> for PhpBalanceResponse {
    fn from(resp: BalanceResponse) -> Self {
        let kind = match resp.kind {
            crate::balance::EntityKind::Company => "Company",
            crate::balance::EntityKind::Ngo => "Ngo",
        };

        let balance: HashMap<String, PhpRawBalance> = resp
            .balance
            .into_inner()
            .into_iter()
            .map(|(k, v)| (k.to_string(), PhpRawBalance::from(v)))
            .collect();

        Self {
            kind: kind.to_string(),
            year: resp.year as u64,
            cui: resp.unique_registration_code as u64,
            name: resp.name,
            caen_code: resp.activity_code as u64,
            activity_name: resp.activity_name,
            balance,
        }
    }
}

#[cfg(feature = "balance_api")]
#[php_class]
#[php(name = "Anaf\\RawBalance")]
#[derive(Clone, Debug)]
pub struct PhpRawBalance {
    pub code: String,
    pub name: String,
    pub value: i64,
}

#[cfg(feature = "balance_api")]
#[php_impl]
impl PhpRawBalance {
    #[php(getter)]
    pub fn get_code(&self) -> String { self.code.clone() }
    #[php(getter)]
    pub fn get_name(&self) -> String { self.name.clone() }
    #[php(getter)]
    pub fn get_value(&self) -> i64 { self.value }
}

#[cfg(feature = "balance_api")]
impl From<RawBalance> for PhpRawBalance {
    fn from(balance: RawBalance) -> Self {
        Self {
            code: balance.code,
            name: balance.name,
            value: balance.value as i64,
        }
    }
}
