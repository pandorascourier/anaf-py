# ANAF API Client

This is the unofficial ANAF (Romanian Tax Authority) WebService client, implemented in Rust with Python bindings.

> **Note:** The API client only supports the synchronous web services.

## Language Bindings

This project provides a Rust core library with bindings for:
- ✅ **Python** (available now via PyO3)
- 🚧 **PHP** (planned via php-ext-rs)
- 🚧 **Node.js** (planned via neon)

All bindings share the same API design and requesting logic as documented in the Rust implementation.

## Supported APIs

- ✅ **Balance API** - V1 (default)
- ✅ **VAT Payer API** - V8 (default), V7
  - Also supports async VAT Payer API
- ✅ **Farmers Registry API** - V2 (default)
- ✅ **Cult Registry API** - V2 (default)

## Unsupported APIs

Currently, we don't plan to support following APIs:
- ❌ e-Factura API
- ❌ e-Transport API

These might be implemented in the future if there is demand. Note that these APIs also require JWT authentication.

---

## Python Installation

### Installation via Git Clone (Recommended for v0.1.0)

Since the project is in early development (v0.1.0), installation via git clone is recommended:

```bash
# Clone the repository
git clone https://github.com/pandorascourier/anaf-py.git
cd anaf-py

# Create a virtual environment (recommended)
python3 -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install the package in development mode
pip install maturin
maturin develop --release
```

### Usage in Python

```python
from anaf_py import AnafClient
from datetime import date

# Initialize the client
client = AnafClient()

# Query VAT Payer information
requests = [(49201783, date.today().strftime("%Y-%m-%d"))]
response = client.vat_payer(requests)
print(response)

# Query Balance information
response = client.balance(40914732, 2022)
print(response)

# Query Farmers Registry
requests = [(12345678, "2024-01-15")]
response = client.farmer(requests)
print(response)

# Query Cults Registry
requests = [(12345678, "2024-01-15")]
response = client.cult(requests)
print(response)
```

### Python API Reference

#### `AnafClient()`

Initialize the ANAF API client.

#### `vat_payer(requests, version=8, async_mode=False)`

Query VAT Payer information.

**Parameters:**
- `requests`: List of tuples `(registration_code: int, date: str)` where date is in `YYYY-MM-DD` format
- `version`: API version (7 or 8, default is 8)
- `async_mode`: Whether to use async API (default False)

**Returns:** Dictionary with the API response

#### `balance(registration_code, year, version=1)`

Query Balance information.

**Parameters:**
- `registration_code`: Company registration code (CUI)
- `year`: Year for the balance query
- `version`: API version (default is 1)

**Returns:** Dictionary with the API response

#### `farmer(requests, version=2)`

Query Farmers Registry information.

**Parameters:**
- `requests`: List of tuples `(registration_code: int, date: str)` where date is in `YYYY-MM-DD` format
- `version`: API version (default is 2)

**Returns:** Dictionary with the API response

#### `cult(requests, version=2)`

Query Cults Registry information.

**Parameters:**
- `requests`: List of tuples `(registration_code: int, date: str)` where date is in `YYYY-MM-DD` format
- `version`: API version (default is 2)

**Returns:** Dictionary with the API response

### Python Examples

See the [examples/python](examples/python) directory for complete working examples.

---

## Rust Usage

For Rust usage, add this to your `Cargo.toml`:

```toml
[dependencies]
anaf-api = { git = "https://github.com/pandorascourier/anaf-py.git" }
```

See the Rust examples in [examples/](examples/) directory.

---

## Development Roadmap

### Pre-v1.0 Features
- ✅ Python bindings (PyO3)
- 🚧 PHP bindings (php-ext-rs)
- 🚧 Node.js bindings (neon)

### Post-v1.0 Features
- ANAF e-Factura API
- ANAF e-Transport API

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.