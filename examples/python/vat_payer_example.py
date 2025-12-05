#!/usr/bin/env python3
"""
Example usage of the ANAF Python API client for VAT Payer queries.
"""

from anaf_py import AnafClient
from datetime import date
import json

def main():
    # Initialize the client
    client = AnafClient()
    
    # Query VAT payer information
    # Using the same example from the Rust code: CUI 49201783
    registration_code = 49201783
    query_date = date.today().strftime("%Y-%m-%d")
    
    print(f"Querying VAT payer information for CUI: {registration_code}")
    print(f"Query date: {query_date}")
    print()
    
    try:
        # Create request list with (registration_code, date) tuples
        requests = [(registration_code, query_date)]
        
        # Send the request to the latest API version (v8)
        response = client.vat_payer(requests)
        
        # Pretty print the response
        print("Response:")
        print(json.dumps(response, indent=2, ensure_ascii=False))
        
        # You can also use API version 7
        # response_v7 = client.vat_payer(requests, version=7)
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
