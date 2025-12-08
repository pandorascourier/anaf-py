//! Node.js wrapper for the ANAF API client.

use neon::prelude::*;
use neon::types::Finalize;
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

use super::responses::*;

// =============================================================================
// Client Wrapper - stored in JsBox
// =============================================================================

/// ANAF API Client wrapper for Node.js.
pub struct AnafClientWrapper {
    runtime: Arc<Runtime>,
}

impl Finalize for AnafClientWrapper {}

impl AnafClientWrapper {
    pub fn new() -> Result<Self, String> {
        let runtime = Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        Ok(Self {
            runtime: Arc::new(runtime),
        })
    }
}

// =============================================================================
// Exported Functions
// =============================================================================

/// Create a new ANAF API client.
/// 
/// Returns a boxed client that can be passed to other functions.
pub fn create_client(mut cx: FunctionContext) -> JsResult<JsBox<AnafClientWrapper>> {
    let client = AnafClientWrapper::new()
        .or_else(|e| cx.throw_error(e))?;
    Ok(cx.boxed(client))
}

/// Create a request object for API calls.
///
/// Arguments:
/// - cui: number (company unique identifier)
/// - date: string (YYYY-MM-DD format, optional - defaults to today)
pub fn create_request(mut cx: FunctionContext) -> JsResult<JsObject> {
    let cui = cx.argument::<JsNumber>(0)?.value(&mut cx) as u64;
    let date = cx.argument_opt(1)
        .and_then(|v| v.downcast::<JsString, _>(&mut cx).ok())
        .map(|s| s.value(&mut cx))
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());

    let obj = cx.empty_object();
    let cui_val = cx.number(cui as f64);
    let date_val = cx.string(&date);
    obj.set(&mut cx, "cui", cui_val)?;
    obj.set(&mut cx, "date", date_val)?;
    Ok(obj)
}

/// Parse a JavaScript request object into an ApiRequest.
fn parse_request(cx: &mut FunctionContext, obj: Handle<JsObject>) -> NeonResult<ApiRequest> {
    let cui: Handle<JsNumber> = obj.get(cx, "cui")?;
    let date: Handle<JsString> = obj.get(cx, "date")?;

    let cui_val = cui.value(cx) as usize;
    let date_str = date.value(cx);
    let date_val = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .unwrap_or_else(|_| chrono::Local::now().date_naive());

    Ok(ApiRequest::new(cui_val, date_val))
}

/// Parse an array of JavaScript request objects.
fn parse_requests(cx: &mut FunctionContext, arr: Handle<JsArray>) -> NeonResult<Vec<ApiRequest>> {
    let len = arr.len(cx);
    let mut requests = Vec::with_capacity(len as usize);
    
    for i in 0..len {
        let obj: Handle<JsObject> = arr.get(cx, i)?;
        requests.push(parse_request(cx, obj)?);
    }
    
    Ok(requests)
}

/// Query VAT payer information.
///
/// Arguments:
/// - client: JsBox<AnafClientWrapper>
/// - requests: Array<{cui: number, date: string}>
/// - version: string (optional, "V7", "V8", or "V9", defaults to "V9")
///
/// Returns: Promise<VatPayerResponse>
pub fn get_vat_payer(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = cx.argument::<JsBox<AnafClientWrapper>>(0)?;
    let requests_arr = cx.argument::<JsArray>(1)?;
    let version = cx.argument_opt(2)
        .and_then(|v| v.downcast::<JsString, _>(&mut cx).ok())
        .map(|s| s.value(&mut cx));

    let requests = parse_requests(&mut cx, requests_arr)?;
    
    let version = match version.as_deref() {
        Some("V8") => VatPayerApiVersion::V8,
        Some("V7") => VatPayerApiVersion::V7,
        Some("V9") | None => VatPayerApiVersion::V9,
        _ => VatPayerApiVersion::default(),
    };

    let runtime = client.runtime.clone();
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    std::thread::spawn(move || {
        let result = runtime.block_on(async {
            let anaf_client = AnafClient::new();
            let api = anaf_client.vat_payer(version);
            api.send(requests).await
        });

        deferred.settle_with(&channel, move |mut cx| {
            match result {
                Ok(response) => vat_payer_response_to_js(&mut cx, response),
                Err(e) => cx.throw_error(e.to_string()),
            }
        });
    });

    Ok(promise)
}

/// Query balance information.
#[cfg(feature = "balance_api")]
pub fn get_balance(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = cx.argument::<JsBox<AnafClientWrapper>>(0)?;
    let cui = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize;
    let year = cx.argument::<JsNumber>(2)?.value(&mut cx) as usize;

    let request = BalanceRequest::new(cui, year);
    let version = BalanceApiVersion::V1;

    let runtime = client.runtime.clone();
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    std::thread::spawn(move || {
        let result = runtime.block_on(async {
            let anaf_client = AnafClient::new();
            let api = anaf_client.balance(version);
            api.send(request).await
        });

        deferred.settle_with(&channel, move |mut cx| {
            match result {
                Ok(response) => balance_response_to_js(&mut cx, response),
                Err(e) => cx.throw_error(e.to_string()),
            }
        });
    });

    Ok(promise)
}

/// Query cult organization information.
#[cfg(feature = "cults_api")]
pub fn get_cult(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = cx.argument::<JsBox<AnafClientWrapper>>(0)?;
    let requests_arr = cx.argument::<JsArray>(1)?;

    let requests = parse_requests(&mut cx, requests_arr)?;
    let version = CultApiVersion::V2;

    let runtime = client.runtime.clone();
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    std::thread::spawn(move || {
        let result = runtime.block_on(async {
            let anaf_client = AnafClient::new();
            let api = anaf_client.cult(version);
            api.send(requests).await
        });

        deferred.settle_with(&channel, move |mut cx| {
            match result {
                Ok(response) => cult_response_to_js(&mut cx, response),
                Err(e) => cx.throw_error(e.to_string()),
            }
        });
    });

    Ok(promise)
}

/// Query farmer information.
#[cfg(feature = "farmers_api")]
pub fn get_farmer(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = cx.argument::<JsBox<AnafClientWrapper>>(0)?;
    let requests_arr = cx.argument::<JsArray>(1)?;

    let requests = parse_requests(&mut cx, requests_arr)?;
    let version = FarmerApiVersion::V2;

    let runtime = client.runtime.clone();
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    std::thread::spawn(move || {
        let result = runtime.block_on(async {
            let anaf_client = AnafClient::new();
            let api = anaf_client.farmer(version);
            api.send(requests).await
        });

        deferred.settle_with(&channel, move |mut cx| {
            match result {
                Ok(response) => farmer_response_to_js(&mut cx, response),
                Err(e) => cx.throw_error(e.to_string()),
            }
        });
    });

    Ok(promise)
}

