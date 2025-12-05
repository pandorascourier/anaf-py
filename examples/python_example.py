#!/usr/bin/env python3
"""
Example usage of the anaf_api Python bindings.

This demonstrates how to use the ANAF API client to query
Romanian fiscal administration data.
"""

from anaf_api import (
    AnafClient,
    ApiRequest,
    BalanceRequest,
    VatPayerApiVersion,
    BalanceApiVersion,
)


def example_vat_payer():
    """Query VAT payer information for a company."""
    print("=" * 60)
    print("VAT Payer API Example")
    print("=" * 60)
    
    client = AnafClient()
    
    # Create a request for today's date
    # 49201783 is an example CUI (Unique Registration Code)
    request = ApiRequest.today(49201783)
    print(f"Request: {request}")
    
    # You can also specify a specific date
    # request = ApiRequest(49201783, 2024, 12, 5)
    
    try:
        # Query with the latest API version (default)
        response = client.get_vat_payer([request])
        
        print(f"\nResponse status: {response.status}")
        print(f"Message: {response.message}")
        print(f"Found: {len(response.data)} companies")
        print(f"Not found CUIs: {response.not_found}")
        
        for company in response.data:
            info = company.company_info
            print(f"\n--- {info.name} ---")
            print(f"  CUI: {info.unique_registration_code}")
            print(f"  Address: {info.address}")
            print(f"  Registration Status: {info.registration_status}")
            print(f"  Has RO e-Invoice: {info.has_ro_einvoice}")
            print(f"  VAT Payer: {company.vat_scope.is_payer}")
            print(f"  Inactive: {company.inactive.status}")
            
            # HQ Address details
            addr = company.hq_address
            print(f"  Full HQ Address: {addr.full_address()}")
            
    except Exception as e:
        print(f"Error: {e}")


def example_balance():
    """Query balance sheet information for a company/NGO."""
    print("\n" + "=" * 60)
    print("Balance API Example")
    print("=" * 60)
    
    client = AnafClient()
    
    # Query balance for year 2022
    # This CUI belongs to an NGO
    request = BalanceRequest(40914732, 2022)
    print(f"Request: {request}")
    
    try:
        response = client.get_balance(request)
        
        print(f"\nEntity: {response.name}")
        print(f"CUI: {response.unique_registration_code}")
        print(f"Type: {response.kind} (is_ngo={response.is_ngo()}, is_company={response.is_company()})")
        print(f"Year: {response.year}")
        print(f"Activity: {response.activity_name} (code: {response.activity_code})")
        
        print("\nBalance indicators:")
        for indicator_name, balance in response.balance.items():
            print(f"  {indicator_name}: {balance.value:,} ({balance.name})")
            
    except Exception as e:
        print(f"Error: {e}")


def example_multiple_companies():
    """Query multiple companies at once."""
    print("\n" + "=" * 60)
    print("Multiple Companies Query Example")
    print("=" * 60)
    
    client = AnafClient()
    
    # Query multiple CUIs at once (max 500)
    cuis = [49201783, 40914732, 12345678]  # Last one is invalid
    requests = [ApiRequest.today(cui) for cui in cuis]
    
    print(f"Querying {len(requests)} companies...")
    
    try:
        response = client.get_vat_payer(requests)
        
        print(f"\nFound: {len(response.data)} companies")
        print(f"Not found: {response.not_found}")
        
        for company in response.data:
            print(f"  - {company.company_info.name} (CUI: {company.company_info.unique_registration_code})")
            
    except Exception as e:
        print(f"Error: {e}")


def example_api_versions():
    """Demonstrate using different API versions."""
    print("\n" + "=" * 60)
    print("API Versions Example")
    print("=" * 60)
    
    print("Available VAT Payer API versions:")
    print(f"  - Latest: {VatPayerApiVersion.latest()}")
    print(f"  - V8: {VatPayerApiVersion.v8()}")
    print(f"  - V7: {VatPayerApiVersion.v7()}")
    
    print("\nAvailable Balance API versions:")
    print(f"  - Latest: {BalanceApiVersion.latest()}")
    print(f"  - V1: {BalanceApiVersion.v1()}")
    
    # Use a specific version
    client = AnafClient()
    request = ApiRequest.today(49201783)
    
    print("\nQuerying with V7 version...")
    try:
        response = client.get_vat_payer([request], version=VatPayerApiVersion.v7())
        print(f"Response: {response}")
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    # Run examples
    example_balance()  # This one works
    example_api_versions()
    
    # These may fail if ANAF API is not available
    # example_vat_payer()
    # example_multiple_companies()
