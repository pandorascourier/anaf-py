# ANAF API Client

A high-performance, unofficial ANAF (Romanian National Agency for Fiscal Administration) WebService client implemented in Rust with bindings for Python, PHP, and Node.js.

> **Note:** This API client supports synchronous web services. The VAT Payer API also supports async operations.

## Table of Contents

- [Features](#features)
- [Supported APIs](#supported-apis)
- [Quick Start](#quick-start)
  - [Rust](#rust-usage)
  - [Python](#python-usage)
  - [PHP](#php-usage)
  - [Node.js](#nodejs-usage)
- [Building from Source](#building-from-source)
- [Running Examples](#running-examples)
- [API Reference](#api-reference)
- [Unsupported APIs](#unsupported-apis)

## Features

- 🦀 Written in Rust for maximum performance and safety
- 🐍 Python bindings via PyO3
- 🐘 PHP extension via ext-php-rs
- 📦 Node.js native addon via Neon
- ⚡ Async support for VAT Payer API
- 🔒 Type-safe responses

## Supported APIs

| API | Versions | Status |
|-----|----------|--------|
| VAT Payer (PlatitorTvaRest) | V9 (default), V8, V7 | ✅ Stable |
| Balance (Bilant) | V1 | ✅ Stable |
| Farmers Registry | V2 | ✅ Stable |
| Cult Registry | V2 | ✅ Stable |

## Quick Start

### Rust Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
anaf-api = { git = "https://github.com/pandorascourier/anaf-py", features = ["vat_payer_async_api", "balance_api"] }
```

```rust
use anaf_api::{AnafClient, ApiRequest, VatPayerApiVersion};
use chrono::Local;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AnafClient::new();
    let today = Local::now().date_naive();
    
    // Query VAT payer info
    let requests = vec![ApiRequest::new(18158683, today)];
    let response = client.vat_payer(VatPayerApiVersion::V9)
        .send(requests)
        .await?;
    
    for entity in response.data {
        println!("Company: {}", entity.general_data.name);
        println!("VAT Registered: {}", entity.vat_registration.is_registered);
    }
    
    Ok(())
}
```

### Python Usage

```python
from anaf_api import AnafClient, ApiRequest

# Create client and request
client = AnafClient()
request = ApiRequest.today(18158683)  # Query using today's date

# Query VAT payer information
response = client.get_vat_payer([request])

for company in response.data:
    print(f"Company: {company.general_data.name}")
    print(f"CUI: {company.general_data.cui}")
    print(f"VAT Registered: {company.vat_registration.is_registered}")
    print(f"Address: {company.headquarters_address.full_address()}")
```

### PHP Usage

```php
<?php
use Anaf\AnafClient;

$client = new AnafClient();
$today = AnafClient::today();

// Query VAT payer info
$response = $client->getVatPayer([
    [18158683, $today]
]);

foreach ($response->getData() as $company) {
    echo "Company: " . $company->getGeneralData()->getName() . "\n";
    echo "CUI: " . $company->getGeneralData()->getCui() . "\n";
    echo "VAT Registered: " . ($company->getVatRegistration()->getIsRegistered() ? 'Yes' : 'No') . "\n";
}
```

### Node.js Usage

```javascript
const anaf = require('./index.node');

async function main() {
    const client = anaf.createClient();
    const today = anaf.today();

    // Query VAT payer info
    const response = await anaf.getVatPayer(client, [
        anaf.createRequest(18158683, today)
    ]);

    for (const company of response.data) {
        console.log(`Company: ${company.generalData.name}`);
        console.log(`CUI: ${company.generalData.cui}`);
        console.log(`VAT Registered: ${company.vatRegistration.isRegistered}`);
    }
}

main().catch(console.error);
```

## Building from Source

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- For Python: Python 3.8+ with pip
- For PHP: PHP 8.0+ with development headers
- For Node.js: Node.js 16+ with npm

### Build Commands

**Rust library only:**
```bash
cargo build --release
```

**Python bindings:**
```bash
cargo build --release --features python
# The library will be at target/release/libanaf_api.so (Linux) or .dylib (macOS)
```

**PHP extension:**
```bash
cargo build --release --no-default-features --features "php,vat_payer_async_api,balance_api,cults_api,farmers_api"
# Load with: php -d "extension=target/release/libanaf_api.so" script.php
```

**Node.js addon:**
```bash
cargo build --release --no-default-features --features "nodejs,vat_payer_async_api,balance_api,cults_api,farmers_api"
cp target/release/libanaf_api.so ./index.node
```

## Running Examples

### Python Example

```bash
# Build the Python bindings
cargo build --release --features python

# Set the library path and run
export LD_LIBRARY_PATH=target/release:$LD_LIBRARY_PATH
python examples/python_example.py
```

Or use the example directly:
```bash
cd examples
python python_example.py
```

**Example output:**
```
ANAF API Python Example
=======================

Querying VAT payer information for CUI 14399840...

Company: DANTE INTERNATIONAL SA
CUI: 14399840
Address: MUNICIPIUL BUCUREŞTI, SECTOR 6, ŞOS. VIRTUŢII, NR.148
VAT Registered: True
Registration Status: INREGISTRAT din data 29.08.2006
```

### PHP Example

```bash
# Build the PHP extension
cargo build --release --no-default-features --features "php,vat_payer_async_api,balance_api,cults_api,farmers_api"

# Run the example
php -d "extension=target/release/libanaf_api.so" examples/php_example.php
```

**Example output:**
```
ANAF API PHP Example
====================

Querying VAT payer information...

Company: DANTE INTERNATIONAL SA
CUI: 14399840
VAT Registered: Yes
Address: MUNICIPIUL BUCUREŞTI, SECTOR 6, ŞOS. VIRTUŢII, NR.148
```

### Node.js Example

```bash
# Build the Node.js addon
cargo build --release --no-default-features --features "nodejs,vat_payer_async_api,balance_api,cults_api,farmers_api"

# Copy the addon
cp target/release/libanaf_api.so index.node

# Run the example
node examples/nodejs_example.js
```

**Example output:**
```
ANAF API Node.js Example
========================

✓ Client created successfully

Today's date: 2025-12-08

Querying VAT payer information...

Status: 0
Message: 
Found: 1 companies

-----------------------------------
Company: DANTE INTERNATIONAL SA
CUI: 14399840
VAT Registered: true
Address: Şos. Virtuţii 148, spatiul E47, Sector 6 Mun. Bucureşti
```

### Running Tests

**Node.js tests:**
```bash
node tests/nodejs_test.js
```

**Rust tests:**
```bash
cargo test
```

## API Reference

### VAT Payer API

Query Romanian company fiscal information including VAT registration status, addresses, and more.

**Response fields:**
- `generalData` - Company name, CUI, address, CAEN code, registration status
- `vatRegistration` - VAT registration status and history
- `vatCollection` - Cash-based VAT (TVA la încasare) status
- `inactivityStatus` - Company activity status
- `splitVat` - Split VAT payment status
- `headquartersAddress` - Registered office address
- `fiscalAddress` - Fiscal domicile address

### Balance API

Query company financial balance sheets.

**Parameters:**
- `cui` - Company Unique Identifier
- `year` - Fiscal year (e.g., 2023)

### Cult Registry API

Query registered religious/cult organizations.

### Farmers Registry API

Query registered agricultural producers.

## Unsupported APIs

The following APIs are not currently supported:
- e-Factura API (requires JWT authentication)
- e-Transport API (requires JWT authentication)

These may be implemented in the future based on demand.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Links

- [ANAF Official Documentation](https://static.anaf.ro/static/10/Anaf/Informatii_R/API/Oauth_procedura_inregistrare_aplicatii_portal_ANAF.pdf)
- [VAT Payer API V9 Documentation](https://static.anaf.ro/static/10/Anaf/Informatii_R/Servicii_web/doc_WS_V9.txt)