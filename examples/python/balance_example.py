#!/usr/bin/env python3
"""
Example usage of the ANAF Python API client for Balance queries.
"""

from anaf_py import AnafClient
import json

def main():
    # Initialize the client
    client = AnafClient()
    
    # Query balance information
    # Using the same example from the Rust code: CUI 40914732, year 2022
    registration_code = 40914732
    year = 2022
    
    print(f"Querying balance information for CUI: {registration_code}")
    print(f"Year: {year}")
    print()
    
    try:
        # Send the request
        response = client.balance(registration_code, year)
        
        # Pretty print the response
        print("Response:")
        print(json.dumps(response, indent=2, ensure_ascii=False))
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
