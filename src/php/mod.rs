//! PHP bindings for the ANAF API client.
//!
//! This module provides PHP extension bindings using ext-php-rs.

use ext_php_rs::prelude::*;

mod client;
mod responses;

pub use client::PhpAnafClient;
pub use responses::*;

/// Entry point for the PHP module.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    let module = module
        // Core classes
        .class::<PhpAnafClient>()
        // VAT Payer response types
        .class::<PhpVatPayerResponse>()
        .class::<PhpTaxPayerEntity>()
        .class::<PhpGeneralData>()
        .class::<PhpVatRegistration>()
        .class::<PhpVatPeriod>()
        .class::<PhpVatCollection>()
        .class::<PhpInactivityStatus>()
        .class::<PhpSplitVat>()
        .class::<PhpHeadquartersAddress>()
        .class::<PhpFiscalAddress>();
    
    // Register cult response types
    #[cfg(feature = "cults_api")]
    let module = module
        .class::<PhpCultResponse>()
        .class::<PhpCultResponseItem>();
    
    // Register farmer response types
    #[cfg(feature = "farmers_api")]
    let module = module
        .class::<PhpFarmerResponse>()
        .class::<PhpFarmerResponseItem>();
    
    // Register balance response types
    #[cfg(feature = "balance_api")]
    let module = module
        .class::<PhpBalanceResponse>()
        .class::<PhpRawBalance>();
    
    module
}
