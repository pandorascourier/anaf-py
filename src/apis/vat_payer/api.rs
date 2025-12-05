use reqwest::{Client, StatusCode};

use crate::{ApiError, ApiRequest, Result};

use super::{VatPayerApiVersion, VatPayerResponse};

#[derive(Debug)]
pub struct VatPayerApi {
    api_url: String,
    version: VatPayerApiVersion,
    client: Client,
}

impl VatPayerApi {
    pub fn new(version: VatPayerApiVersion, client: Client, api_url: &str) -> Self {
        Self {
            version,
            client,
            api_url: api_url.to_owned(),
        }
    }
}

impl VatPayerApi {
    pub async fn send(&self, request: Vec<ApiRequest>) -> Result<VatPayerResponse> {
        let max_cuis = self.version.max_cuis();
        if request.is_empty() || request.len() > max_cuis {
            return Err(ApiError::InvalidRequestError(request.len()));
        }

        tracing::info!(
            "Making ANAF VatPayer API {version} call",
            version = self.version
        );
        tracing::debug!("URL: {:#?}", self.api_url);
        tracing::debug!("Request: {:#?}", request);

        let response = self
            .client
            .post(&self.api_url)
            .json(&request)
            .send()
            .await?;

        match response.status() {
            StatusCode::SERVICE_UNAVAILABLE => Err(ApiError::ServiceUnavailable),
            StatusCode::OK => {
                let response = response.json::<VatPayerResponse>().await?;
                tracing::debug!("Response: {:#?}", response);
                Ok(response)
            }
            _ => {
                let response = response.text().await?;
                tracing::debug!("Error Response: {:#?}", response);
                Err(ApiError::ApiError(response))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use reqwest::Client;

    use crate::{
        vat_payer::{VatPayerApi, VatPayerApiVersion},
        ApiRequest,
    };

    #[tokio::test]
    async fn api_handles_200() {
        let mut server = mockito::Server::new_async().await;

        let mock_url = server.url();

        let version = VatPayerApiVersion::V8;
        let endpoint = format!("/PlatitorTvaRest/api/{}/ws/tva", version);
        let api_url = format!("{}{}", mock_url, endpoint);

        let mock = server
            .mock("POST", endpoint.clone().as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"cod\":200,\"message\":\"\",\"data\":[],\"not_found\":[]}")
            .create();

        let request = vec![ApiRequest::new(111111111, Utc::now().date_naive())];
        let api = VatPayerApi::new(version, Client::new(), api_url.clone().as_str());
        let response = api.send(request).await;

        assert!(response.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn api_handles_503() {
        let mut server = mockito::Server::new_async().await;

        let mock_url = server.url();

        let version = VatPayerApiVersion::V8;
        let endpoint = format!("/PlatitorTvaRest/api/{}/ws/tva", version);
        let api_url = format!("{}{}", mock_url, endpoint);

        let mock = server
            .mock("POST", endpoint.clone().as_str())
            .with_status(503)
            .with_header("content-type", "text/html")
            .create();

        let request = vec![ApiRequest::new(111111111, Utc::now().date_naive())];
        let api = VatPayerApi::new(version, Client::new(), api_url.clone().as_str());
        let response = api.send(request).await;

        assert!(response.is_err());
        mock.assert_async().await;
    }
}
