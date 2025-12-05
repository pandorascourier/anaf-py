#!/usr/bin/env python3
"""
Basic test to verify the Python bindings work correctly.
This test checks that the API can be imported and instantiated.
"""

import sys

def test_import():
    """Test that the module can be imported."""
    try:
        from anaf_py import AnafClient
        print("✓ Successfully imported AnafClient")
        return True
    except ImportError as e:
        print(f"✗ Failed to import: {e}")
        return False

def test_client_creation():
    """Test that the client can be instantiated."""
    try:
        from anaf_py import AnafClient
        client = AnafClient()
        print("✓ Successfully created AnafClient instance")
        return True
    except Exception as e:
        print(f"✗ Failed to create client: {e}")
        return False

def test_api_methods():
    """Test that all API methods are available."""
    try:
        from anaf_py import AnafClient
        client = AnafClient()
        
        methods = ['vat_payer', 'balance', 'farmer', 'cult']
        for method in methods:
            if hasattr(client, method):
                print(f"✓ Method '{method}' is available")
            else:
                print(f"✗ Method '{method}' is missing")
                return False
        return True
    except Exception as e:
        print(f"✗ Failed to check methods: {e}")
        return False

def main():
    """Run all tests."""
    print("Testing ANAF Python bindings...")
    print()
    
    tests = [
        ("Import test", test_import),
        ("Client creation test", test_client_creation),
        ("API methods test", test_api_methods),
    ]
    
    results = []
    for name, test_func in tests:
        print(f"\nRunning: {name}")
        print("-" * 40)
        result = test_func()
        results.append(result)
    
    print("\n" + "=" * 40)
    print("Test Summary:")
    print("=" * 40)
    
    passed = sum(results)
    total = len(results)
    
    for (name, _), result in zip(tests, results):
        status = "PASSED" if result else "FAILED"
        print(f"{name}: {status}")
    
    print()
    print(f"Total: {passed}/{total} tests passed")
    
    return 0 if all(results) else 1

if __name__ == "__main__":
    sys.exit(main())
