//! Node.js response conversion functions for ANAF API responses.
//!
//! This module provides functions to convert Rust response types into
//! JavaScript objects using Neon's API.

use neon::prelude::*;

use crate::vat_payer::{
    VatPayerResponse, TaxPayerEntity, GeneralData, VatRegistration, VatPeriod,
    VatCollection, InactivityStatus, SplitVat, HeadquartersAddress, FiscalAddress,
};

// =============================================================================
// VAT Payer Response Conversion
// =============================================================================

/// Convert a VatPayerResponse to a JavaScript object.
pub fn vat_payer_response_to_js<'a>(
    cx: &mut impl Context<'a>,
    response: VatPayerResponse,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let status = cx.number(response.status as f64);
    obj.set(cx, "status", status)?;

    let message = cx.string(&response.message);
    obj.set(cx, "message", message)?;

    // Convert data array
    let data_arr = JsArray::new(cx, response.data.len());
    for (i, entity) in response.data.into_iter().enumerate() {
        let entity_obj = tax_payer_entity_to_js(cx, entity)?;
        data_arr.set(cx, i as u32, entity_obj)?;
    }
    obj.set(cx, "data", data_arr)?;

    // Convert not_found array
    let not_found_arr = JsArray::new(cx, response.not_found.len());
    for (i, cui) in response.not_found.iter().enumerate() {
        let cui_val = cx.number(*cui as f64);
        not_found_arr.set(cx, i as u32, cui_val)?;
    }
    obj.set(cx, "notFound", not_found_arr)?;

    Ok(obj)
}

/// Convert a TaxPayerEntity to a JavaScript object.
fn tax_payer_entity_to_js<'a>(
    cx: &mut impl Context<'a>,
    entity: TaxPayerEntity,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let general_data = general_data_to_js(cx, entity.general_data)?;
    obj.set(cx, "generalData", general_data)?;

    let vat_registration = vat_registration_to_js(cx, entity.vat_registration)?;
    obj.set(cx, "vatRegistration", vat_registration)?;

    let vat_collection = vat_collection_to_js(cx, entity.vat_collection)?;
    obj.set(cx, "vatCollection", vat_collection)?;

    let inactivity_status = inactivity_status_to_js(cx, entity.inactivity_status)?;
    obj.set(cx, "inactivityStatus", inactivity_status)?;

    let split_vat = split_vat_to_js(cx, entity.split_vat)?;
    obj.set(cx, "splitVat", split_vat)?;

    let headquarters_address = headquarters_address_to_js(cx, entity.headquarters_address)?;
    obj.set(cx, "headquartersAddress", headquarters_address)?;

    let fiscal_address = fiscal_address_to_js(cx, entity.fiscal_address)?;
    obj.set(cx, "fiscalAddress", fiscal_address)?;

    Ok(obj)
}

/// Convert GeneralData to a JavaScript object.
fn general_data_to_js<'a>(
    cx: &mut impl Context<'a>,
    data: GeneralData,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let cui = cx.number(data.cui as f64);
    obj.set(cx, "cui", cui)?;

    let query_date = cx.string(&data.query_date);
    obj.set(cx, "queryDate", query_date)?;

    let name = cx.string(&data.name);
    obj.set(cx, "name", name)?;

    let address = cx.string(&data.address);
    obj.set(cx, "address", address)?;

    let trade_register_number = cx.string(&data.trade_register_number);
    obj.set(cx, "tradeRegisterNumber", trade_register_number)?;

    let phone = cx.string(&data.phone);
    obj.set(cx, "phone", phone)?;

    let fax = cx.string(&data.fax);
    obj.set(cx, "fax", fax)?;

    let postal_code = cx.string(&data.postal_code);
    obj.set(cx, "postalCode", postal_code)?;

    let authorization_act = cx.string(&data.authorization_act);
    obj.set(cx, "authorizationAct", authorization_act)?;

    let registration_status = cx.string(&data.registration_status);
    obj.set(cx, "registrationStatus", registration_status)?;

    let registration_date = cx.string(&data.registration_date);
    obj.set(cx, "registrationDate", registration_date)?;

    let caen_code = cx.string(&data.caen_code);
    obj.set(cx, "caenCode", caen_code)?;

    let iban = cx.string(&data.iban);
    obj.set(cx, "iban", iban)?;

    let ro_efactura_status = cx.boolean(data.ro_efactura_status);
    obj.set(cx, "roEfacturaStatus", ro_efactura_status)?;

    let ro_efactura_registration_date = cx.string(&data.ro_efactura_registration_date);
    obj.set(cx, "roEfacturaRegistrationDate", ro_efactura_registration_date)?;

    let fiscal_authority = cx.string(&data.fiscal_authority);
    obj.set(cx, "fiscalAuthority", fiscal_authority)?;

    let ownership_form = cx.string(&data.ownership_form);
    obj.set(cx, "ownershipForm", ownership_form)?;

    let organization_form = cx.string(&data.organization_form);
    obj.set(cx, "organizationForm", organization_form)?;

    let legal_form = cx.string(&data.legal_form);
    obj.set(cx, "legalForm", legal_form)?;

    Ok(obj)
}

/// Convert VatRegistration to a JavaScript object.
fn vat_registration_to_js<'a>(
    cx: &mut impl Context<'a>,
    data: VatRegistration,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let is_registered = cx.boolean(data.is_registered);
    obj.set(cx, "isRegistered", is_registered)?;

    let periods_arr = JsArray::new(cx, data.periods.len());
    for (i, period) in data.periods.into_iter().enumerate() {
        let period_obj = vat_period_to_js(cx, period)?;
        periods_arr.set(cx, i as u32, period_obj)?;
    }
    obj.set(cx, "periods", periods_arr)?;

    Ok(obj)
}

/// Convert VatPeriod to a JavaScript object.
fn vat_period_to_js<'a>(
    cx: &mut impl Context<'a>,
    data: VatPeriod,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let start_date = cx.string(&data.start_date);
    obj.set(cx, "startDate", start_date)?;

    let end_date = cx.string(&data.end_date);
    obj.set(cx, "endDate", end_date)?;

    let annulment_date = cx.string(&data.annulment_date);
    obj.set(cx, "annulmentDate", annulment_date)?;

    let annulment_message = cx.string(&data.annulment_message);
    obj.set(cx, "annulmentMessage", annulment_message)?;

    Ok(obj)
}

/// Convert VatCollection to a JavaScript object.
fn vat_collection_to_js<'a>(
    cx: &mut impl Context<'a>,
    data: VatCollection,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let is_applied = cx.boolean(data.is_applied);
    obj.set(cx, "isApplied", is_applied)?;

    let start_date = cx.string(&data.start_date);
    obj.set(cx, "startDate", start_date)?;

    let end_date = cx.string(&data.end_date);
    obj.set(cx, "endDate", end_date)?;

    let update_type = cx.string(&data.update_type);
    obj.set(cx, "updateType", update_type)?;

    let update_date = cx.string(&data.update_date);
    obj.set(cx, "updateDate", update_date)?;

    let publication_date = cx.string(&data.publication_date);
    obj.set(cx, "publicationDate", publication_date)?;

    Ok(obj)
}

/// Convert InactivityStatus to a JavaScript object.
fn inactivity_status_to_js<'a>(
    cx: &mut impl Context<'a>,
    data: InactivityStatus,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let is_inactive = cx.boolean(data.is_inactive);
    obj.set(cx, "isInactive", is_inactive)?;

    let inactivation_date = cx.string(&data.inactivation_date);
    obj.set(cx, "inactivationDate", inactivation_date)?;

    let reactivation_date = cx.string(&data.reactivation_date);
    obj.set(cx, "reactivationDate", reactivation_date)?;

    let publication_date = cx.string(&data.publication_date);
    obj.set(cx, "publicationDate", publication_date)?;

    let deregistration_date = cx.string(&data.deregistration_date);
    obj.set(cx, "deregistrationDate", deregistration_date)?;

    Ok(obj)
}

/// Convert SplitVat to a JavaScript object.
fn split_vat_to_js<'a>(
    cx: &mut impl Context<'a>,
    data: SplitVat,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let is_applied = cx.boolean(data.is_applied);
    obj.set(cx, "isApplied", is_applied)?;

    let start_date = cx.string(&data.start_date);
    obj.set(cx, "startDate", start_date)?;

    let cancellation_date = cx.string(&data.cancellation_date);
    obj.set(cx, "cancellationDate", cancellation_date)?;

    Ok(obj)
}

/// Convert HeadquartersAddress to a JavaScript object.
fn headquarters_address_to_js<'a>(
    cx: &mut impl Context<'a>,
    data: HeadquartersAddress,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let country = cx.string(&data.country);
    obj.set(cx, "country", country)?;

    let county = cx.string(&data.county);
    obj.set(cx, "county", county)?;

    let county_code = cx.string(&data.county_code);
    obj.set(cx, "countyCode", county_code)?;

    let county_auto_code = cx.string(&data.county_auto_code);
    obj.set(cx, "countyAutoCode", county_auto_code)?;

    let locality = cx.string(&data.locality);
    obj.set(cx, "locality", locality)?;

    let locality_code = cx.string(&data.locality_code);
    obj.set(cx, "localityCode", locality_code)?;

    let street = cx.string(&data.street);
    obj.set(cx, "street", street)?;

    let street_number = cx.string(&data.street_number);
    obj.set(cx, "streetNumber", street_number)?;

    let details = cx.string(&data.details);
    obj.set(cx, "details", details)?;

    let postal_code = cx.string(&data.postal_code);
    obj.set(cx, "postalCode", postal_code)?;

    let full_address = cx.string(&data.full_address());
    obj.set(cx, "fullAddress", full_address)?;

    Ok(obj)
}

/// Convert FiscalAddress to a JavaScript object.
fn fiscal_address_to_js<'a>(
    cx: &mut impl Context<'a>,
    data: FiscalAddress,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let country = cx.string(&data.country);
    obj.set(cx, "country", country)?;

    let county = cx.string(&data.county);
    obj.set(cx, "county", county)?;

    let county_code = cx.string(&data.county_code);
    obj.set(cx, "countyCode", county_code)?;

    let county_auto_code = cx.string(&data.county_auto_code);
    obj.set(cx, "countyAutoCode", county_auto_code)?;

    let locality = cx.string(&data.locality);
    obj.set(cx, "locality", locality)?;

    let locality_code = cx.string(&data.locality_code);
    obj.set(cx, "localityCode", locality_code)?;

    let street = cx.string(&data.street);
    obj.set(cx, "street", street)?;

    let street_number = cx.string(&data.street_number);
    obj.set(cx, "streetNumber", street_number)?;

    let details = cx.string(&data.details);
    obj.set(cx, "details", details)?;

    let postal_code = cx.string(&data.postal_code);
    obj.set(cx, "postalCode", postal_code)?;

    let full_address = cx.string(&data.full_address());
    obj.set(cx, "fullAddress", full_address)?;

    Ok(obj)
}

// =============================================================================
// Balance Response Conversion
// =============================================================================

#[cfg(feature = "balance_api")]
pub fn balance_response_to_js<'a>(
    cx: &mut impl Context<'a>,
    response: crate::balance::BalanceResponse,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let kind = cx.string(format!("{:?}", response.kind));
    obj.set(cx, "kind", kind)?;

    let year = cx.number(response.year as f64);
    obj.set(cx, "year", year)?;

    let cui = cx.number(response.unique_registration_code as f64);
    obj.set(cx, "uniqueRegistrationCode", cui)?;

    let name = cx.string(&response.name);
    obj.set(cx, "name", name)?;

    let activity_code = cx.number(response.activity_code as f64);
    obj.set(cx, "activityCode", activity_code)?;

    let activity_name = cx.string(&response.activity_name);
    obj.set(cx, "activityName", activity_name)?;

    // Convert balance to an object representation
    let balance_obj = cx.empty_object();
    // Balance is a complex type - we'll add the raw indicators
    obj.set(cx, "balance", balance_obj)?;

    Ok(obj)
}

// =============================================================================
// Cult Response Conversion
// =============================================================================

#[cfg(feature = "cults_api")]
pub fn cult_response_to_js<'a>(
    cx: &mut impl Context<'a>,
    response: crate::cults::CultResponse,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let status = cx.number(response.status as f64);
    obj.set(cx, "status", status)?;

    let message = cx.string(&response.message);
    obj.set(cx, "message", message)?;

    // Convert data array
    let data_arr = JsArray::new(cx, response.data.len());
    for (i, entity) in response.data.iter().enumerate() {
        let entity_obj = cx.empty_object();

        let cui = cx.number(entity.unique_registration_code as f64);
        entity_obj.set(cx, "uniqueRegistrationCode", cui)?;

        let when = cx.string(&entity.when);
        entity_obj.set(cx, "when", when)?;

        let name = cx.string(&entity.name);
        entity_obj.set(cx, "name", name)?;

        let address = cx.string(&entity.address);
        entity_obj.set(cx, "address", address)?;

        let commerce_registry_number = cx.string(&entity.commerce_registry_number);
        entity_obj.set(cx, "commerceRegistryNumber", commerce_registry_number)?;

        let phone = cx.string(&entity.phone);
        entity_obj.set(cx, "phone", phone)?;

        let fax = cx.string(&entity.fax);
        entity_obj.set(cx, "fax", fax)?;

        let postal_code = cx.string(&entity.postal_code);
        entity_obj.set(cx, "postalCode", postal_code)?;

        let act = cx.string(&entity.act);
        entity_obj.set(cx, "act", act)?;

        let registration_status = cx.string(&entity.registration_status);
        entity_obj.set(cx, "registrationStatus", registration_status)?;

        let cult_since = cx.string(&entity.cult_since);
        entity_obj.set(cx, "cultSince", cult_since)?;

        if let Some(ref cult_until) = entity.cult_until {
            let cult_until_val = cx.string(cult_until);
            entity_obj.set(cx, "cultUntil", cult_until_val)?;
        } else {
            let null = cx.null();
            entity_obj.set(cx, "cultUntil", null)?;
        }

        let is_active = cx.boolean(entity.is_active);
        entity_obj.set(cx, "isActive", is_active)?;

        data_arr.set(cx, i as u32, entity_obj)?;
    }
    obj.set(cx, "data", data_arr)?;

    // Convert not_found array
    let not_found_arr = JsArray::new(cx, response.not_found.len());
    for (i, cui) in response.not_found.iter().enumerate() {
        let cui_val = cx.number(*cui as f64);
        not_found_arr.set(cx, i as u32, cui_val)?;
    }
    obj.set(cx, "notFound", not_found_arr)?;

    Ok(obj)
}

// =============================================================================
// Farmer Response Conversion
// =============================================================================

#[cfg(feature = "farmers_api")]
pub fn farmer_response_to_js<'a>(
    cx: &mut impl Context<'a>,
    response: crate::farmers::FarmerResponse,
) -> JsResult<'a, JsObject> {
    let obj = cx.empty_object();

    let status = cx.number(response.status as f64);
    obj.set(cx, "status", status)?;

    let message = cx.string(&response.message);
    obj.set(cx, "message", message)?;

    // Convert data array
    let data_arr = JsArray::new(cx, response.data.len());
    for (i, entity) in response.data.iter().enumerate() {
        let entity_obj = cx.empty_object();

        let cui = cx.number(entity.unique_registration_code as f64);
        entity_obj.set(cx, "uniqueRegistrationCode", cui)?;

        let when = cx.string(&entity.when);
        entity_obj.set(cx, "when", when)?;

        let name = cx.string(&entity.name);
        entity_obj.set(cx, "name", name)?;

        let address = cx.string(&entity.address);
        entity_obj.set(cx, "address", address)?;

        let commerce_registry_number = cx.string(&entity.commerce_registry_number);
        entity_obj.set(cx, "commerceRegistryNumber", commerce_registry_number)?;

        let phone = cx.string(&entity.phone);
        entity_obj.set(cx, "phone", phone)?;

        let fax = cx.string(&entity.fax);
        entity_obj.set(cx, "fax", fax)?;

        let postal_code = cx.string(&entity.postal_code);
        entity_obj.set(cx, "postalCode", postal_code)?;

        let act = cx.string(&entity.act);
        entity_obj.set(cx, "act", act)?;

        let registration_status = cx.string(&entity.registration_status);
        entity_obj.set(cx, "registrationStatus", registration_status)?;

        let farmer_since = cx.string(&entity.farmer_since);
        entity_obj.set(cx, "farmerSince", farmer_since)?;

        if let Some(ref farmer_until) = entity.farmer_until {
            let farmer_until_val = cx.string(farmer_until);
            entity_obj.set(cx, "farmerUntil", farmer_until_val)?;
        } else {
            let null = cx.null();
            entity_obj.set(cx, "farmerUntil", null)?;
        }

        let is_active = cx.boolean(entity.is_active);
        entity_obj.set(cx, "isActive", is_active)?;

        data_arr.set(cx, i as u32, entity_obj)?;
    }
    obj.set(cx, "data", data_arr)?;

    // Convert not_found array
    let not_found_arr = JsArray::new(cx, response.not_found.len());
    for (i, cui) in response.not_found.iter().enumerate() {
        let cui_val = cx.number(*cui as f64);
        not_found_arr.set(cx, i as u32, cui_val)?;
    }
    obj.set(cx, "notFound", not_found_arr)?;

    Ok(obj)
}

