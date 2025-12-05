# Installation Guide for ANAF-PY

This guide provides detailed instructions for installing the ANAF Python API client.

## Prerequisites

Before installing, ensure you have:

- **Python 3.8 or higher**
- **Rust toolchain** (cargo and rustc)
- **Git**

### Installing Rust

If you don't have Rust installed:

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download and run: https://rustup.rs/
```

Verify installation:
```bash
rustc --version
cargo --version
```

## Installation Methods

### Method 1: Git Clone + Local Install (Recommended for v0.1.0)

This is the recommended method for the current development version.

#### Step 1: Clone the Repository

```bash
git clone https://github.com/pandorascourier/anaf-py.git
cd anaf-py
```

#### Step 2: Create a Virtual Environment (Recommended)

```bash
# Create virtual environment
python3 -m venv .venv

# Activate it
source .venv/bin/activate  # Linux/macOS
# or
.venv\Scripts\activate     # Windows
```

#### Step 3: Install Maturin

```bash
pip install maturin
```

#### Step 4: Build and Install the Package

For development (editable install):
```bash
maturin develop
```

For production (release build):
```bash
maturin develop --release
```

The release build is optimized and will be faster but takes longer to compile.

#### Step 5: Verify Installation

```bash
python3 -c "from anaf_py import AnafClient; print('Installation successful!')"
```

Or run the test suite:
```bash
python3 examples/python/test_basic.py
```

### Method 2: Install in Another Project

If you want to use anaf-py in a different Python project:

#### Option A: Add to requirements.txt

```txt
# requirements.txt
anaf-py @ git+https://github.com/pandorascourier/anaf-py.git
```

Then install:
```bash
pip install -r requirements.txt
```

#### Option B: Direct pip install from Git

```bash
pip install git+https://github.com/pandorascourier/anaf-py.git
```

**Note:** This requires maturin to be available during installation. You may need to install it first:
```bash
pip install maturin
pip install git+https://github.com/pandorascourier/anaf-py.git
```

## Troubleshooting

### Issue: "maturin: command not found"

**Solution:** Install maturin in your current environment:
```bash
pip install maturin
```

### Issue: "No module named '_anaf_py'"

**Solution:** The native extension wasn't built properly. Try:
```bash
# Clean build artifacts
rm -rf target/
# Rebuild
maturin develop --release
```

### Issue: Rust compiler errors

**Solution:** Ensure you have the latest stable Rust:
```bash
rustup update stable
```

### Issue: Python version mismatch

**Solution:** Ensure you're using Python 3.8 or higher:
```bash
python3 --version
```

If you have multiple Python versions, specify the one to use:
```bash
# Use a specific Python version
python3.11 -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop --release
```

## Verifying Your Installation

After installation, run these verification steps:

### 1. Basic Import Test

```bash
python3 -c "from anaf_py import AnafClient; client = AnafClient(); print('OK')"
```

### 2. Run the Test Suite

```bash
python3 examples/python/test_basic.py
```

Expected output:
```
Testing ANAF Python bindings...
...
Total: 3/3 tests passed
```

### 3. Try an Example

```bash
# This will make a real API call to ANAF
python3 examples/python/vat_payer_example.py
```

## Development Setup

If you want to contribute or modify the code:

```bash
# Clone the repo
git clone https://github.com/pandorascourier/anaf-py.git
cd anaf-py

# Create and activate virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install development tools
pip install maturin

# Build in development mode (faster builds, includes debug info)
maturin develop

# Make your changes to src/python.rs or python/anaf_py/__init__.py
# ...

# Rebuild after changes
maturin develop

# Test your changes
python3 examples/python/test_basic.py
```

## Uninstalling

To uninstall the package:

```bash
pip uninstall anaf-py
```

To remove all build artifacts:
```bash
# Remove Python build artifacts
rm -rf build/ dist/ *.egg-info/

# Remove Rust build artifacts  
rm -rf target/

# Remove virtual environment
rm -rf .venv/
```

## Next Steps

After successful installation:

1. Read the [README.md](README.md) for API documentation
2. Try the examples in [examples/python/](examples/python/)
3. Integrate the library into your project

## Getting Help

If you encounter issues:

1. Check this troubleshooting guide
2. Look for similar issues on [GitHub Issues](https://github.com/pandorascourier/anaf-py/issues)
3. Create a new issue with:
   - Your OS and version
   - Python version (`python3 --version`)
   - Rust version (`rustc --version`)
   - Full error message
   - Steps to reproduce
