# API Design Philosophy

This document describes the API design philosophy and shared syntax across all language bindings (Python, PHP, Node.js) for the ANAF API client.

## Core Principle: Consistent API Across Languages

The ANAF API client maintains consistent API design across all language bindings. The Rust implementation serves as the canonical reference for:

1. **Request/Response Structure**: All bindings use the same data structures
2. **Method Signatures**: Methods have equivalent names and parameters across languages
3. **Error Handling**: Similar error patterns and messages
4. **API Versioning**: Same version numbers and semantics

## Rust as the Source of Truth

The Rust implementation (`src/client.rs`, `src/apis/`) documents the official API behavior. Language bindings follow these patterns:

### Initialization

All bindings create a client instance that manages HTTP connections and API state.

**Rust:**
```rust
let client = AnafClient::new();
```

**Python:**
```python
client = AnafClient()
```

**PHP (planned):**
```php
$client = new AnafClient();
```

**Node.js (planned):**
```javascript
const client = new AnafClient();
```

### Request Format

Requests use consistent parameter patterns:

- **Registration Code**: Company CUI (Cod Unic de Identificare)
- **Date**: ISO format YYYY-MM-DD
- **Version**: API version number (integer)

**Rust:**
```rust
let request = vec![ApiRequest::new(49201783, now)];
let response = client.vat_payer(VatPayerApiVersion::V8).send(request).await?;
```

**Python:**
```python
requests = [(49201783, "2024-01-15")]
response = client.vat_payer(requests, version=8)
```

**PHP (planned):**
```php
$requests = [[49201783, "2024-01-15"]];
$response = $client->vatPayer($requests, version: 8);
```

**Node.js (planned):**
```javascript
const requests = [[49201783, "2024-01-15"]];
const response = await client.vatPayer(requests, {version: 8});
```

## API Methods

All bindings expose these core methods:

### 1. VAT Payer API (`vat_payer` / `vatPayer`)

Query VAT payer information.

**Parameters:**
- `requests`: List/Array of [registration_code, date] pairs
- `version`: API version (7 or 8, default: 8)
- `async_mode`: Use async API (default: false) - Python only

**Returns:** Response object with company VAT information

### 2. Balance API (`balance`)

Query company balance sheets.

**Parameters:**
- `registration_code`: Company CUI
- `year`: Year for balance query
- `version`: API version (default: 1)

**Returns:** Response object with balance information

### 3. Farmers Registry API (`farmer`)

Query farmers registry.

**Parameters:**
- `requests`: List/Array of [registration_code, date] pairs
- `version`: API version (default: 2)

**Returns:** Response object with farmer registration info

### 4. Cults Registry API (`cult`)

Query religious organizations registry.

**Parameters:**
- `requests`: List/Array of [registration_code, date] pairs
- `version`: API version (default: 2)

**Returns:** Response object with cult registration info

## Response Format

All responses follow the same structure based on the ANAF API:

```json
{
  "cod": 200,
  "message": "...",
  "found": [...],
  "notFound": [...]
}
```

### Response Fields

- `cod` (status): HTTP-like status code
- `message`: Status message
- `found` (data): Array of found results
- `notFound` (not_found): Array of registration codes not found

## Error Handling

All bindings use similar error types:

### Error Categories

1. **Service Unavailable**: ANAF API is under maintenance (503)
2. **API Error**: Server returned an error response
3. **Invalid Request**: Request parameters are invalid
4. **Network Error**: Connection issues
5. **Serialization Error**: JSON parsing failed

**Rust:**
```rust
match client.vat_payer(version).send(request).await {
    Ok(response) => // handle success,
    Err(ApiError::ServiceUnavailable) => // handle maintenance,
    Err(e) => // handle other errors,
}
```

**Python:**
```python
try:
    response = client.vat_payer(requests)
except Exception as e:
    # Handle error
    print(f"Error: {e}")
```

## Naming Conventions

### Rust (Snake Case)
- Functions: `vat_payer()`, `async_vat_payer()`
- Types: `ApiRequest`, `VatPayerResponse`
- Fields: `registration_code`, `company_info`

### Python (Snake Case)
- Methods: `vat_payer()`, `balance()`
- Variables: `registration_code`, `company_info`

### PHP (Camel Case - Planned)
- Methods: `vatPayer()`, `balance()`
- Properties: `registrationCode`, `companyInfo`

### JavaScript/Node.js (Camel Case - Planned)
- Methods: `vatPayer()`, `balance()`
- Properties: `registrationCode`, `companyInfo`

## API Versions

All bindings support the same API versions:

### VAT Payer API
- **V8** (default): Latest version with all fields
- **V7**: Older version, fewer fields

### Balance API
- **V1** (default): Current version

### Farmers Registry API
- **V2** (default): Current version

### Cults Registry API
- **V2** (default): Current version

## Date Handling

All bindings accept dates in ISO 8601 format: `YYYY-MM-DD`

**Rust** uses `chrono::NaiveDate`
**Python** accepts string in "YYYY-MM-DD" format
**PHP** will accept string or DateTime objects
**Node.js** will accept string or Date objects

## Type Safety

Each binding leverages its language's type system:

### Rust
- Strong static typing
- Compile-time checks
- Result types for errors

### Python
- Type hints (optional)
- Runtime validation
- Exception-based errors

### PHP (Planned)
- Type declarations
- Runtime type checks
- Exception-based errors

### Node.js (Planned)
- TypeScript definitions
- Runtime validation
- Promise-based async

## Async/Sync Support

### Rust
- All APIs are async (using tokio)
- Supports both sync and async VAT Payer endpoints

### Python
- Blocking API (async handled internally with tokio runtime)
- `async_mode` parameter for async VAT Payer endpoint

### PHP (Planned)
- Blocking API
- Optional async support via ReactPHP

### Node.js (Planned)
- Promise-based async API
- Native async/await support

## Documentation

- **Rust**: Primary documentation source (rustdoc)
- **Python**: Mirrors Rust API with Python docstrings
- **PHP**: Will mirror Rust API with PHPDoc
- **Node.js**: Will mirror Rust API with JSDoc/TSDoc

## Example Equivalence

Here's the same operation in all bindings:

### Query VAT Payer Information

**Rust:**
```rust
use anaf_api::{AnafClient, ApiRequest, vat_payer::VatPayerApiVersion};
use chrono::Local;

let client = AnafClient::new();
let now = Local::now().date_naive();
let request = vec![ApiRequest::new(49201783, now)];
let response = client.vat_payer(VatPayerApiVersion::V8).send(request).await?;
```

**Python:**
```python
from anaf_py import AnafClient
from datetime import date

client = AnafClient()
today = date.today().strftime("%Y-%m-%d")
response = client.vat_payer([(49201783, today)], version=8)
```

**PHP (planned):**
```php
use AnafPy\AnafClient;

$client = new AnafClient();
$today = date('Y-m-d');
$response = $client->vatPayer([[49201783, $today]], version: 8);
```

**Node.js (planned):**
```javascript
const { AnafClient } = require('anaf-py');

const client = new AnafClient();
const today = new Date().toISOString().split('T')[0];
const response = await client.vatPayer([[49201783, today]], {version: 8});
```

## Conclusion

By maintaining consistent API design across all language bindings while respecting each language's idioms, we provide:

1. **Predictable API**: Learn once, use anywhere
2. **Maintainability**: Single source of truth (Rust implementation)
3. **Reliability**: Shared core logic reduces bugs
4. **Documentation**: Rust code documents behavior for all bindings

This approach ensures that improvements and bug fixes in the core Rust library benefit all language bindings automatically.
