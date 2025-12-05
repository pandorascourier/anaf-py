# ANAF API client

This is the unofficial ANAF WebService client, implemented in Rust.

> **Note:** The API client only supports the synchronous web services.

## Goals
- supports following APIs:
  - [x] Balance API;
  - [x] VAT Payer API;
  - [x] Farmers Registry API;
  - [x] Cult Registry API;

## Supported APIs

### Balance API

> **Note:** The Balance API is currently experimental and may change in the future.

This API supports following versions:
- V1 (default)


### VAT Payer API

> **Note:** This crate also supports the async VAT Payer API.

This API supports following versions:
- V8 (default)
- V7


### Farmers Registry API

This API supports following versions:
- V2 (default)

### Cult Registry API

This API supports following versions:
- V2 (default)

## Unsupported APIs

Currently, we don't plan to support following APIs:
- [ ] e-Factura API;
- [ ] e-Transport API;

These might be implemented in the future, if there is a demand for them. However, please note that these APIs will also require JWT authentication.

## Language Bindings

This library provides bindings for multiple languages:

### Python Bindings

Build with PyO3:

```bash
cargo build --release --features python
```

Usage:
```python
from anaf_api import AnafClient, ApiRequest

client = AnafClient()
request = ApiRequest.today(18158683)  # Dedeman CUI
response = client.get_vat_payer([request])

for company in response.data:
    print(f"Company: {company.general_data.name}")
    print(f"VAT Payer: {company.vat_registration.is_registered}")
```

### PHP Bindings

Build with ext-php-rs:

```bash
cargo build --release --no-default-features --features "php,vat_payer_async_api,balance_api,cults_api,farmers_api"
```

This creates `target/release/libanaf_api.so`. Load it in PHP:

```bash
php -d "extension=/path/to/libanaf_api.so" your_script.php
```

Or add to php.ini:
```ini
extension=/path/to/libanaf_api.so
```

Usage:
```php
<?php
use Anaf\AnafClient;

$client = new AnafClient();
$today = AnafClient::today();

// Query VAT payer info - pass array of [cui, date] pairs
$response = $client->getVatPayer([
    [18158683, $today]  // Dedeman CUI
]);

foreach ($response->data as $company) {
    echo "Company: {$company->generalData->name}\n";
    echo "VAT Payer: " . ($company->vatRegistration->isRegistered ? 'Yes' : 'No') . "\n";
}

// Query balance information
$balance = $client->getBalance(18158683, 2023);
echo "Balance Year: {$balance->year}\n";
```

**Available PHP Classes:**
- `Anaf\AnafClient` - Main client with API methods
- `Anaf\VatPayerResponse`, `Anaf\TaxPayerEntity`, `Anaf\GeneralData`
- `Anaf\VatRegistration`, `Anaf\VatPeriod`, `Anaf\VatCollection`
- `Anaf\InactivityStatus`, `Anaf\SplitVat`
- `Anaf\HeadquartersAddress`, `Anaf\FiscalAddress`
- `Anaf\CultResponse`, `Anaf\CultResponseItem`
- `Anaf\FarmerResponse`, `Anaf\FarmerResponseItem`
- `Anaf\BalanceResponse`, `Anaf\RawBalance`
These might be implemented in the future, if there is a demand for them. However, please note that these APIs will also require JWT authentication.