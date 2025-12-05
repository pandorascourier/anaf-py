"""
ANAF API Python Client

This package provides Python bindings for the ANAF (Romanian Tax Authority) API client.
"""

from ._anaf_py import PyAnafClient as _PyAnafClient
import json

class AnafClient:
    """
    Python client for ANAF (Romanian Tax Authority) API.
    
    This client provides access to various ANAF APIs including:
    - VAT Payer information
    - Balance sheets
    - Farmers registry
    - Cults registry
    """
    
    def __init__(self):
        """Initialize the ANAF client."""
        self._client = _PyAnafClient()
    
    def _parse_response(self, json_str):
        """
        Parse JSON response string into a dictionary.
        
        Args:
            json_str: JSON string from the native module
            
        Returns:
            Parsed dictionary
            
        Raises:
            ValueError: If the response is not valid JSON
        """
        try:
            return json.loads(json_str)
        except json.JSONDecodeError as e:
            raise ValueError(f"Failed to parse API response: {e}") from e
    
    def vat_payer(self, requests, version=8, async_mode=False):
        """
        Query VAT Payer information.
        
        Args:
            requests: List of tuples (registration_code: int, date: str in YYYY-MM-DD format)
            version: API version (7 or 8, default is 8)
            async_mode: Whether to use async API (default False)
            
        Returns:
            Dictionary with the API response
            
        Example:
            >>> client = AnafClient()
            >>> result = client.vat_payer([(49201783, "2024-01-15")])
        """
        json_str = self._client.vat_payer(requests, version, async_mode)
        return self._parse_response(json_str)
    
    def balance(self, registration_code, year, version=1):
        """
        Query Balance information.
        
        Args:
            registration_code: Company registration code (CUI)
            year: Year for the balance query
            version: API version (default is 1)
            
        Returns:
            Dictionary with the API response
            
        Example:
            >>> client = AnafClient()
            >>> result = client.balance(40914732, 2022)
        """
        json_str = self._client.balance(registration_code, year, version)
        return self._parse_response(json_str)
    
    def farmer(self, requests, version=2):
        """
        Query Farmers Registry information.
        
        Args:
            requests: List of tuples (registration_code: int, date: str in YYYY-MM-DD format)
            version: API version (default is 2)
            
        Returns:
            Dictionary with the API response
            
        Example:
            >>> client = AnafClient()
            >>> result = client.farmer([(12345678, "2024-01-15")])
        """
        json_str = self._client.farmer(requests, version)
        return self._parse_response(json_str)
    
    def cult(self, requests, version=2):
        """
        Query Cults Registry information.
        
        Args:
            requests: List of tuples (registration_code: int, date: str in YYYY-MM-DD format)
            version: API version (default is 2)
            
        Returns:
            Dictionary with the API response
            
        Example:
            >>> client = AnafClient()
            >>> result = client.cult([(12345678, "2024-01-15")])
        """
        json_str = self._client.cult(requests, version)
        return self._parse_response(json_str)

__version__ = "0.1.0"
__all__ = ["AnafClient"]
