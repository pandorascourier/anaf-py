//! PHP wrapper for the ANAF API client.

use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use std::sync::Arc;
use tokio::runtime::Runtime;
use chrono::NaiveDate;

use crate::common::ApiRequest;
use crate::vat_payer::VatPayerApiVersion;
use crate::AnafClient;

#[cfg(feature = "balance_api")]
use crate::balance::{BalanceApiVersion, BalanceRequest};
#[cfg(feature = "cults_api")]
use crate::cults::CultApiVersion;
#[cfg(feature = "farmers_api")]
use crate::farmers::FarmerApiVersion;

use super::PhpVatPayerResponse;

#[cfg(feature = "balance_api")]
use super::PhpBalanceResponse;
#[cfg(feature = "cults_api")]
use super::PhpCultResponse;
#[cfg(feature = "farmers_api")]
use super::PhpFarmerResponse;

/// Parse API requests from a PHP array of [cui, date] arrays.
fn parse_requests(requests: &ZendHashTable) -> Result<Vec<ApiRequest>, PhpException> {
    let mut result = Vec::new();
    for (_key, val) in requests.iter() {
        if let Some(arr) = val.array() {
            let cui = arr.get(0)
                .and_then(|v| v.long())
                .ok_or_else(|| PhpException::default("Each request must have a CUI as first element".to_string()))?;
            let date_str = arr.get(1)
                .and_then(|v| v.string())
                .ok_or_else(|| PhpException::default("Each request must have a date string as second element".to_string()))?;
            
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|e| PhpException::default(format!("Invalid date format: {}. Use YYYY-MM-DD", e)))?;
            
            let req = ApiRequest::new(cui as usize, date);
            result.push(req);
        } else {
            return Err(PhpException::default("Each request must be an array [cui, date]".to_string()));
        }
    }
    Ok(result)
}

/// ANAF API Client for PHP.
///
/// Provides access to Romanian National Agency for Fiscal Administration APIs.
///
/// Example:
/// ```php
/// $client = new Anaf\AnafClient();
/// // Pass requests as [[cui, date], [cui, date], ...]
/// $response = $client->getVatPayer([[12345678, '2024-01-15']]);
/// foreach ($response->data as $company) {
///     echo $company->generalData->name . "\n";
/// }
/// ```
#[php_class]
#[php(name = "Anaf\\AnafClient")]
#[derive(Clone)]
pub struct PhpAnafClient {
    runtime: Arc<Runtime>,
}

#[php_impl]
impl PhpAnafClient {
    /// Create a new ANAF API client.
    pub fn __construct() -> PhpResult<Self> {
        let runtime = Runtime::new().map_err(|e| {
            PhpException::default(format!("Failed to create runtime: {}", e))
        })?;
        Ok(Self {
            runtime: Arc::new(runtime),
        })
    }

    /// Query VAT payer information for companies.
    ///
    /// @param array $requests List of [cui, date] arrays (max 100 for V9, 500 for V8/V7).
    ///                        Use today's date in YYYY-MM-DD format.
    /// @param ?string $version API version to use ("V9" = default, "V8", "V7").
    /// @return VatPayerResponse Response with company information.
    pub fn get_vat_payer(
        &self,
        requests: &ZendHashTable,
        version: Option<String>,
    ) -> PhpResult<PhpVatPayerResponse> {
        let version = match version.as_deref() {
            Some("V8") => VatPayerApiVersion::V8,
            Some("V7") => VatPayerApiVersion::V7,
            Some("V9") | None => VatPayerApiVersion::V9,
            _ => VatPayerApiVersion::default(),
        };
        let api_requests = parse_requests(requests)?;

        let client = AnafClient::new();
        let api = client.vat_payer(version);

        self.runtime
            .block_on(async { api.send(api_requests).await })
            .map(PhpVatPayerResponse::from)
            .map_err(|e| PhpException::default(e.to_string()).into())
    }

    /// Query VAT payer information asynchronously (bulk API).
    ///
    /// This is the async variant that supports more requests.
    ///
    /// @param array $requests List of [cui, date] arrays (max 100 for V9, 500 for V8/V7).
    /// @param ?string $version API version to use ("V9" = default, "V8", "V7").
    /// @return VatPayerResponse Response with company information.
    #[cfg(feature = "vat_payer_async_api")]
    pub fn get_vat_payer_async(
        &self,
        requests: &ZendHashTable,
        version: Option<String>,
    ) -> PhpResult<PhpVatPayerResponse> {
        let version = match version.as_deref() {
            Some("V8") => VatPayerApiVersion::V8,
            Some("V7") => VatPayerApiVersion::V7,
            Some("V9") | None => VatPayerApiVersion::V9,
            _ => VatPayerApiVersion::default(),
        };
        let api_requests = parse_requests(requests)?;

        let client = AnafClient::new();
        let api = client.async_vat_payer(version);

        self.runtime
            .block_on(async { api.send(api_requests).await })
            .map(PhpVatPayerResponse::from)
            .map_err(|e| PhpException::default(e.to_string()).into())
    }

    /// Query cult registry information.
    ///
    /// @param array $requests List of [cui, date] arrays (max 500).
    /// @param ?string $version API version to use ("V2" = default).
    /// @return CultResponse Response with cult information.
    #[cfg(feature = "cults_api")]
    pub fn get_cult(
        &self,
        requests: &ZendHashTable,
        version: Option<String>,
    ) -> PhpResult<PhpCultResponse> {
        let version = match version.as_deref() {
            Some("V2") | None => CultApiVersion::V2,
            _ => CultApiVersion::default(),
        };
        let api_requests = parse_requests(requests)?;

        let client = AnafClient::new();
        let api = client.cult(version);

        self.runtime
            .block_on(async { api.send(api_requests).await })
            .map(PhpCultResponse::from)
            .map_err(|e| PhpException::default(e.to_string()).into())
    }

    /// Query farmer registry information.
    ///
    /// @param array $requests List of [cui, date] arrays (max 500).
    /// @param ?string $version API version to use ("V2" = default).
    /// @return FarmerResponse Response with farmer information.
    #[cfg(feature = "farmers_api")]
    pub fn get_farmer(
        &self,
        requests: &ZendHashTable,
        version: Option<String>,
    ) -> PhpResult<PhpFarmerResponse> {
        let version = match version.as_deref() {
            Some("V2") | None => FarmerApiVersion::V2,
            _ => FarmerApiVersion::default(),
        };
        let api_requests = parse_requests(requests)?;

        let client = AnafClient::new();
        let api = client.farmer(version);

        self.runtime
            .block_on(async { api.send(api_requests).await })
            .map(PhpFarmerResponse::from)
            .map_err(|e| PhpException::default(e.to_string()).into())
    }

    /// Query company balance information.
    ///
    /// @param int $cui Company unique identification number.
    /// @param int $year Year of the balance (e.g., 2023).
    /// @param ?string $version API version to use ("V1" = default).
    /// @return BalanceResponse Response with balance information.
    #[cfg(feature = "balance_api")]
    pub fn get_balance(
        &self,
        cui: i64,
        year: i64,
        version: Option<String>,
    ) -> PhpResult<PhpBalanceResponse> {
        let version = match version.as_deref() {
            Some("V1") | None => BalanceApiVersion::V1,
            _ => BalanceApiVersion::default(),
        };
        
        let request = BalanceRequest::new(cui as usize, year as usize);

        let client = AnafClient::new();
        let api = client.balance(version);

        self.runtime
            .block_on(async { api.send(request).await })
            .map(PhpBalanceResponse::from)
            .map_err(|e| PhpException::default(e.to_string()).into())
    }
    
    /// Helper: Get today's date in YYYY-MM-DD format.
    ///
    /// @return string Today's date.
    pub fn today() -> String {
        chrono::Utc::now().format("%Y-%m-%d").to_string()
    }
}

impl Default for PhpAnafClient {
    fn default() -> Self {
        Self::__construct().expect("Failed to create AnafClient")
    }
}
