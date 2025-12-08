/**
 * TypeScript type definitions for ANAF API Node.js native module
 */

/**
 * ANAF API Client handle (opaque type)
 */
export type AnafClient = object;

/**
 * API request object
 */
export interface ApiRequest {
    /** Company Unique Identifier (CUI) */
    cui: number;
    /** Date in YYYY-MM-DD format */
    date: string;
}

/**
 * General entity data from ANAF
 */
export interface GeneralData {
    /** Company Unique Identifier */
    cui: number;
    /** Query date */
    queryDate: string;
    /** Company legal name */
    name: string;
    /** Full address */
    address: string;
    /** Trade register number (e.g., "F40/8875/2023") */
    tradeRegisterNumber: string;
    /** Phone number */
    phone: string;
    /** Fax number */
    fax: string;
    /** Postal code */
    postalCode: string;
    /** Authorization act */
    authorizationAct: string;
    /** Registration status (e.g., "INREGISTRAT din data 28.11.2023") */
    registrationStatus: string;
    /** Registration date (YYYY-MM-DD) */
    registrationDate: string;
    /** CAEN activity code */
    caenCode: string;
    /** IBAN for split TVA payments */
    iban: string;
    /** RO e-Factura registry status */
    roEfacturaStatus: boolean;
    /** RO e-Factura registration date */
    roEfacturaRegistrationDate: string;
    /** Competent fiscal authority */
    fiscalAuthority: string;
    /** Ownership form */
    ownershipForm: string;
    /** Organization form */
    organizationForm: string;
    /** Legal/juridical form */
    legalForm: string;
}

/**
 * VAT period information
 */
export interface VatPeriod {
    /** Start date (YYYY-MM-DD) */
    startDate: string;
    /** End date (YYYY-MM-DD) */
    endDate: string;
    /** Annulment date */
    annulmentDate: string;
    /** Annulment message */
    annulmentMessage: string;
}

/**
 * VAT registration status
 */
export interface VatRegistration {
    /** Whether the entity is VAT registered */
    isRegistered: boolean;
    /** VAT registration periods */
    periods: VatPeriod[];
}

/**
 * VAT collection (TVA la încasare) status
 */
export interface VatCollection {
    /** Whether VAT collection system is applied */
    isApplied: boolean;
    /** Start date */
    startDate: string;
    /** End date */
    endDate: string;
    /** Update type */
    updateType: string;
    /** Last update date */
    updateDate: string;
    /** Publication date */
    publicationDate: string;
}

/**
 * Inactivity status
 */
export interface InactivityStatus {
    /** Whether the entity is inactive */
    isInactive: boolean;
    /** Inactivation date */
    inactivationDate: string;
    /** Reactivation date */
    reactivationDate: string;
    /** Publication date */
    publicationDate: string;
    /** Deregistration date */
    deregistrationDate: string;
}

/**
 * Split VAT status
 */
export interface SplitVat {
    /** Whether split VAT is applied */
    isApplied: boolean;
    /** Start date */
    startDate: string;
    /** Cancellation date */
    cancellationDate: string;
}

/**
 * Address information
 */
export interface Address {
    /** Country */
    country: string;
    /** County/district name */
    county: string;
    /** County code */
    countyCode: string;
    /** County auto code (e.g., "B" for București) */
    countyAutoCode: string;
    /** Locality/city name */
    locality: string;
    /** Locality code */
    localityCode: string;
    /** Street name */
    street: string;
    /** Street number */
    streetNumber: string;
    /** Additional details */
    details: string;
    /** Postal code */
    postalCode: string;
    /** Formatted full address */
    fullAddress: string;
}

/**
 * Complete tax payer entity information
 */
export interface TaxPayerEntity {
    /** General data about the entity */
    generalData: GeneralData;
    /** VAT registration details */
    vatRegistration: VatRegistration;
    /** VAT collection (cash-based VAT) details */
    vatCollection: VatCollection;
    /** Inactivity/reactivation status */
    inactivityStatus: InactivityStatus;
    /** Split VAT payment details */
    splitVat: SplitVat;
    /** Social headquarters address */
    headquartersAddress: Address;
    /** Fiscal domicile address */
    fiscalAddress: Address;
}

/**
 * VAT Payer API response
 */
export interface VatPayerResponse {
    /** Response status code */
    status: number;
    /** Response message */
    message: string;
    /** Found entities */
    data: TaxPayerEntity[];
    /** CUIs that were not found */
    notFound: number[];
}

/**
 * Balance API response
 */
export interface BalanceResponse {
    /** Entity type (Company or Ngo) */
    kind: string;
    /** Fiscal year */
    year: number;
    /** Company Unique Identifier */
    uniqueRegistrationCode: number;
    /** Company name */
    name: string;
    /** CAEN activity code */
    activityCode: number;
    /** CAEN activity description */
    activityName: string;
    /** Balance data (complex object) */
    balance: object;
}

/**
 * Cult organization entity
 */
export interface CultEntity {
    /** Company Unique Identifier */
    uniqueRegistrationCode: number;
    /** Query date */
    when: string;
    /** Organization name */
    name: string;
    /** Address */
    address: string;
    /** Commerce registry number */
    commerceRegistryNumber: string;
    /** Phone */
    phone: string;
    /** Fax */
    fax: string;
    /** Postal code */
    postalCode: string;
    /** Authorization act */
    act: string;
    /** Registration status */
    registrationStatus: string;
    /** Registration start date */
    cultSince: string;
    /** Registration end date (null if still active) */
    cultUntil: string | null;
    /** Whether actively registered */
    isActive: boolean;
}

/**
 * Cult API response
 */
export interface CultResponse {
    /** Response status code */
    status: number;
    /** Response message */
    message: string;
    /** Found entities */
    data: CultEntity[];
    /** CUIs that were not found */
    notFound: number[];
}

/**
 * Farmer entity
 */
export interface FarmerEntity {
    /** Company Unique Identifier */
    uniqueRegistrationCode: number;
    /** Query date */
    when: string;
    /** Organization name */
    name: string;
    /** Address */
    address: string;
    /** Commerce registry number */
    commerceRegistryNumber: string;
    /** Phone */
    phone: string;
    /** Fax */
    fax: string;
    /** Postal code */
    postalCode: string;
    /** Authorization act */
    act: string;
    /** Registration status */
    registrationStatus: string;
    /** Farmer registration start date */
    farmerSince: string;
    /** Farmer registration end date (null if still active) */
    farmerUntil: string | null;
    /** Whether actively registered */
    isActive: boolean;
}

/**
 * Farmer API response
 */
export interface FarmerResponse {
    /** Response status code */
    status: number;
    /** Response message */
    message: string;
    /** Found entities */
    data: FarmerEntity[];
    /** CUIs that were not found */
    notFound: number[];
}

/**
 * Create a new ANAF API client
 */
export function createClient(): AnafClient;

/**
 * Get today's date in YYYY-MM-DD format
 */
export function today(): string;

/**
 * Create an API request object
 * @param cui Company Unique Identifier
 * @param date Date in YYYY-MM-DD format (defaults to today)
 */
export function createRequest(cui: number, date?: string): ApiRequest;

/**
 * Query VAT payer information
 * @param client ANAF API client
 * @param requests Array of API requests
 * @param version API version ("V7", "V8", or "V9", defaults to "V9")
 */
export function getVatPayer(
    client: AnafClient,
    requests: ApiRequest[],
    version?: "V7" | "V8" | "V9"
): Promise<VatPayerResponse>;

/**
 * Query company balance information
 * @param client ANAF API client
 * @param cui Company Unique Identifier
 * @param year Fiscal year
 */
export function getBalance(
    client: AnafClient,
    cui: number,
    year: number
): Promise<BalanceResponse>;

/**
 * Query cult organization information
 * @param client ANAF API client
 * @param requests Array of API requests
 */
export function getCult(
    client: AnafClient,
    requests: ApiRequest[]
): Promise<CultResponse>;

/**
 * Query farmer information
 * @param client ANAF API client
 * @param requests Array of API requests
 */
export function getFarmer(
    client: AnafClient,
    requests: ApiRequest[]
): Promise<FarmerResponse>;
