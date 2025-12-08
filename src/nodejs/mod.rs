//! Node.js bindings for the ANAF API client.
//!
//! This module provides Node.js-accessible functions for the ANAF API,
//! allowing JavaScript/TypeScript applications to query Romanian fiscal data.
//!
//! # Usage (JavaScript)
//!
//! ```javascript
//! const anaf = require('./index.node');
//!
//! // Create a client
//! const client = anaf.createClient();
//!
//! // Query VAT payer info
//! const result = await anaf.getVatPayer(client, [{ cui: 12345678, date: '2024-01-15' }]);
//! console.log(result);
//! ```

mod client;
mod responses;

pub use client::*;

#[allow(unused_imports)]
pub use responses::*;

use neon::prelude::*;

/// Register the Node.js module with all exported functions.
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // Client lifecycle
    cx.export_function("createClient", create_client)?;

    // VAT Payer API
    cx.export_function("getVatPayer", get_vat_payer)?;

    // Balance API
    #[cfg(feature = "balance_api")]
    cx.export_function("getBalance", get_balance)?;

    // Cults API
    #[cfg(feature = "cults_api")]
    cx.export_function("getCult", get_cult)?;

    // Farmers API
    #[cfg(feature = "farmers_api")]
    cx.export_function("getFarmer", get_farmer)?;

    // Helper functions
    cx.export_function("today", today)?;
    cx.export_function("createRequest", create_request)?;

    Ok(())
}

/// Get today's date in YYYY-MM-DD format.
fn today(mut cx: FunctionContext) -> JsResult<JsString> {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    Ok(cx.string(date))
}
