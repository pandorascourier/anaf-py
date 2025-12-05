"""
Type stubs for anaf_api Python module.

This file provides type hints for IDE support.
"""

from typing import List, Optional, Dict


class VatPayerApiVersion:
    """VAT Payer API version selector."""
    
    @staticmethod
    def v8() -> "VatPayerApiVersion":
        """Create version V8 (latest)."""
        ...
    
    @staticmethod
    def v7() -> "VatPayerApiVersion":
        """Create version V7."""
        ...
    
    @staticmethod
    def latest() -> "VatPayerApiVersion":
        """Get the latest version."""
        ...


class BalanceApiVersion:
    """Balance API version selector."""
    
    @staticmethod
    def v1() -> "BalanceApiVersion":
        """Create version V1 (latest)."""
        ...
    
    @staticmethod
    def latest() -> "BalanceApiVersion":
        """Get the latest version."""
        ...


class CultApiVersion:
    """Cult API version selector."""
    
    @staticmethod
    def v2() -> "CultApiVersion":
        """Create version V2 (latest)."""
        ...
    
    @staticmethod
    def latest() -> "CultApiVersion":
        """Get the latest version."""
        ...


class FarmerApiVersion:
    """Farmer API version selector."""
    
    @staticmethod
    def v2() -> "FarmerApiVersion":
        """Create version V2 (latest)."""
        ...
    
    @staticmethod
    def latest() -> "FarmerApiVersion":
        """Get the latest version."""
        ...


class ApiRequest:
    """API request for VAT Payer, Cult, and Farmer APIs."""
    
    def __init__(self, registration_code: int, year: int, month: int, day: int) -> None:
        """
        Create a new API request.
        
        Args:
            registration_code: The company's unique registration code (CUI).
            year: The year for the query.
            month: The month for the query.
            day: The day for the query.
        """
        ...
    
    @staticmethod
    def today(registration_code: int) -> "ApiRequest":
        """
        Create a request for today's date.
        
        Args:
            registration_code: The company's unique registration code (CUI).
        """
        ...
    
    @property
    def registration_code(self) -> int:
        """The company's unique registration code (CUI)."""
        ...
    
    @property
    def date(self) -> str:
        """The date for the query."""
        ...


class BalanceRequest:
    """Balance API request."""
    
    def __init__(self, registration_code: int, year: int) -> None:
        """
        Create a new Balance API request.
        
        Args:
            registration_code: The company's unique registration code (CUI).
            year: The year to get balance for.
        """
        ...
    
    @property
    def registration_code(self) -> int:
        """The company's unique registration code (CUI)."""
        ...
    
    @property
    def year(self) -> int:
        """The year for the balance query."""
        ...


class Address:
    """Address information."""
    
    street: str
    number: str
    town: str
    town_code: str
    county: str
    county_code: str
    county_code_auto: str
    country: str
    details: str
    postal_code: str
    
    def full_address(self) -> str:
        """Get the full address as a formatted string."""
        ...


class CompanyInfo:
    """General company information."""
    
    unique_registration_code: int
    when: str
    name: str
    address: str
    commerce_registry_number: str
    phone: str
    fax: str
    postal_code: str
    act: str
    registration_status: str
    registration_date: str
    activity_code: str
    iban: str
    has_ro_einvoice: bool
    trusted_fiscal_activity: Optional[str]
    property_form: Optional[str]
    organization_form: Optional[str]
    juridic_form: Optional[str]


class VatPayerInterval:
    """VAT payer interval details."""
    
    from_: Optional[str]
    to: Optional[str]
    cancelled_at: Optional[str]
    cancelled_reason: Optional[str]


class VatScope:
    """VAT scope information."""
    
    is_payer: bool
    payer_interval: VatPayerInterval


class VatPayerIncome:
    """VAT payer income information."""
    
    from_: str
    to: Optional[str]
    updated_at: Optional[str]
    published_at: Optional[str]
    update_type: str
    status: bool


class InactiveStatus:
    """Inactive status information."""
    
    deactivated_at: str
    reactivated_at: str
    published_at: str
    erased_at: str
    status: bool


class VatSplit:
    """VAT split information."""
    
    started_at: str
    cancelled_at: str
    status: bool


class VatPayerResponseItem:
    """Detailed information about a VAT payer company."""
    
    company_info: CompanyInfo
    vat_scope: VatScope
    vat_payer_income: VatPayerIncome
    inactive: InactiveStatus
    vat_split: VatSplit
    hq_address: Address
    fiscal_address: Address


class VatPayerResponse:
    """Response from the VAT Payer API."""
    
    status: int
    message: str
    data: List[VatPayerResponseItem]
    not_found: List[int]
    
    def __len__(self) -> int: ...


class CultResponseItem:
    """Information about a registered cult."""
    
    unique_registration_code: int
    when: str
    name: str
    address: str
    commerce_registry_number: str
    phone: str
    fax: str
    postal_code: str
    act: str
    registration_status: str
    cult_since: str
    cult_until: Optional[str]
    is_active: bool


class CultResponse:
    """Response from the Cult API."""
    
    status: int
    message: str
    data: List[CultResponseItem]
    not_found: List[int]
    
    def __len__(self) -> int: ...


class FarmerResponseItem:
    """Information about a registered farmer."""
    
    unique_registration_code: int
    when: str
    name: str
    address: str
    commerce_registry_number: str
    phone: str
    fax: str
    postal_code: str
    act: str
    registration_status: str
    farmer_since: str
    farmer_until: Optional[str]
    is_active: bool


class FarmerResponse:
    """Response from the Farmer API."""
    
    status: int
    message: str
    data: List[FarmerResponseItem]
    not_found: List[int]
    
    def __len__(self) -> int: ...


class RawBalance:
    """Raw balance indicator data."""
    
    code: str
    name: str
    value: int


class BalanceResponse:
    """Response from the Balance API."""
    
    kind: str
    year: int
    unique_registration_code: int
    name: str
    activity_code: int
    activity_name: str
    balance: Dict[str, RawBalance]
    
    def is_company(self) -> bool:
        """Check if this is a company balance."""
        ...
    
    def is_ngo(self) -> bool:
        """Check if this is an NGO balance."""
        ...


class AnafClient:
    """
    ANAF API Client for Python.
    
    Provides access to Romanian National Agency for Fiscal Administration APIs.
    """
    
    def __init__(self) -> None:
        """Create a new ANAF API client."""
        ...
    
    def get_vat_payer(
        self,
        requests: List[ApiRequest],
        version: Optional[VatPayerApiVersion] = None,
    ) -> VatPayerResponse:
        """
        Query VAT payer information for companies.
        
        Args:
            requests: List of ApiRequest objects (max 500).
            version: API version to use (default: latest).
        
        Returns:
            VatPayerResponse with company information.
        """
        ...
    
    def get_vat_payer_async(
        self,
        requests: List[ApiRequest],
        version: Optional[VatPayerApiVersion] = None,
    ) -> VatPayerResponse:
        """
        Query VAT payer information asynchronously (bulk API).
        
        Args:
            requests: List of ApiRequest objects (max 500).
            version: API version to use (default: latest).
        
        Returns:
            VatPayerResponse with company information.
        """
        ...
    
    def get_cult(
        self,
        requests: List[ApiRequest],
        version: Optional[CultApiVersion] = None,
    ) -> CultResponse:
        """
        Query cult registry information.
        
        Args:
            requests: List of ApiRequest objects (max 500).
            version: API version to use (default: latest).
        
        Returns:
            CultResponse with cult information.
        """
        ...
    
    def get_farmer(
        self,
        requests: List[ApiRequest],
        version: Optional[FarmerApiVersion] = None,
    ) -> FarmerResponse:
        """
        Query farmer registry information.
        
        Args:
            requests: List of ApiRequest objects (max 500).
            version: API version to use (default: latest).
        
        Returns:
            FarmerResponse with farmer information.
        """
        ...
    
    def get_balance(
        self,
        request: BalanceRequest,
        version: Optional[BalanceApiVersion] = None,
    ) -> BalanceResponse:
        """
        Query company balance information.
        
        Args:
            request: BalanceRequest object.
            version: API version to use (default: latest).
        
        Returns:
            BalanceResponse with balance information.
        """
        ...
