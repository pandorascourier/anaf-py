use reqwest::Client;

use crate::apis::vat_payer::{VatPayerApi, VatPayerApiVersion};

#[cfg(feature = "balance_api")]
use crate::balance::{BalanceApi, BalanceApiVersion};
#[cfg(feature = "cults_api")]
use crate::cults::{CultApi, CultApiVersion};
#[cfg(feature = "farmers_api")]
use crate::farmers::{FarmerApi, FarmerApiVersion};

/// Anaf API Client
///
/// Provides access to all Anaf APIs and their versions.
///
/// Usage:
///
/// ```rust
/// // Initialize the client
/// let client = AnafClient::new();
///
/// // You'll need to send the current date to the API. You can use chrono for that:
/// let now = chrono::Local::now().date_naive();
///
/// // ANAF has the same request format for VAT Payer, Cult and Farmer APIs.
/// // However, you can use only one type at a time.
/// let vat_payer_request = vec![
///     VatPayerRequest::new(49201783, now)
/// ];
///
/// // Send the request to the latest API version.
/// let response = client.vat_payer(Default::default()).send(vat_payer_request).await?;
/// // ...or, send to a specific API version.
/// let response = client.vat_payer(VatPayerApiVersion::V7).send(vat_payer_request).await?;
///
/// ```
#[derive(Clone)]
pub struct AnafClient {
    base_url: String,
    client: Client,
}

impl Default for AnafClient {
    fn default() -> Self {
        Self {
            base_url: "https://webservicesp.anaf.ro".to_owned(),
            client: Client::new(),
        }
    }
}

impl AnafClient {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AnafClient {
    /// Initiates the VatPayer API.
    pub fn vat_payer(self, version: VatPayerApiVersion) -> VatPayerApi {
        VatPayerApi::new(
            version.clone(),
            self.client,
            &format!("{}/PlatitorTvaRest/api/{}/ws/tva", self.base_url, version),
        )
    }

    /// Initiates the VatPayer Async API.
    #[cfg(feature = "vat_payer_async_api")]
    pub fn async_vat_payer(self, version: VatPayerApiVersion) -> VatPayerApi {
        VatPayerApi::new(
            version.clone(),
            self.client,
            &format!("{}/AsynchWebService/api/{}/ws/tva", self.base_url, version),
        )
    }

    /// Initiates the Cult API.
    #[cfg(feature = "cults_api")]
    pub fn cult(self, version: CultApiVersion) -> CultApi {
        CultApi::new(
            version.clone(),
            self.client,
            &format!("{}/RegCult/api/{}/ws/cult", self.base_url, version),
        )
    }

    /// Initiates the Farmer API.
    #[cfg(feature = "farmers_api")]
    pub fn farmer(self, version: FarmerApiVersion) -> FarmerApi {
        FarmerApi::new(
            version.clone(),
            self.client,
            &format!("{}/RegAgric/api/{}/ws/agric", self.base_url, version),
        )
    }

    #[cfg(feature = "balance_api")]
    pub fn balance(self, version: BalanceApiVersion) -> BalanceApi {
        BalanceApi::new(
            version.clone(),
            self.client,
            &format!("{}/bilant", self.base_url),
        )
    }
}
