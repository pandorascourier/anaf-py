- **Entity Details**: The provided payload describes a Romanian tax-registered entity known as a "Persoană Fizică Autorizată" (Authorized Natural Person), identified by CUI 49201783. This is a standard structure for entities registered with ANAF (Romanian National Agency for Fiscal Administration), which includes general data, VAT (TVA) status, inactivity details, split VAT application, and address information. Based on ANAF's official API documentation, this aligns with the response format for querying VAT payers via the PlatitorTvaRest service.

- **Registration Status**: The entity is registered as of November 28, 2023, with no active VAT (TVA) registration, no split TVA, and no inactivity flags. It operates under CAEN code 6201 (likely custom computer programming activities) and is under the jurisdiction of the Timiș County Fiscal Administration.

- **Key Flags**:
  - Not registered for VAT purposes (scpTVA: false).
  - Not applying VAT at collection (statusTvaIncasare: false).
  - Not inactive (statusInactivi: false).
  - Not applying split VAT (statusSplitTVA: false).
  - Not in RO e-Factura registry (statusRO_e_Factura: false).

### General Data
- **CUI**: 49201783
- **Name**: SZEMERECZKI PETRU-NICOLAE PERSOANĂ FIZICĂ AUTORIZATĂ
- **Registration Date**: 2023-11-28
- **Trade Register Number**: F40/8875/2023
- **CAEN Code**: 6201
- **Fiscal Authority**: Administraţia Judeţeană a Finanţelor Publice Timiş
- **Address**: Municipiul Bucureşti, Sector 1, B-dul Bucureştii Noi, Nr.136, Parter, Ap.5 (Postal Code: 12366)

### VAT and Related Statuses
No active VAT periods, no VAT at collection, no split VAT, and no inactivity or deregistration events.

---

The following is a comprehensive schema for a Romanian tax payee registered entity, modeled as Rust structures. This schema is derived from the provided payload and cross-verified with ANAF's official PlatitorTvaRest API documentation (version 9), which defines the standard fields for querying fiscal entities by CUI (Cod Unic de Înregistrare). The API response structure captures details on general entity information, VAT (TVA) registration history, VAT at collection (RTVAI), inactivity status, split TVA, and detailed addresses for social headquarters and fiscal domicile.

ANAF's service is used to verify entities under Romanian fiscal law (e.g., art. 316 of the Fiscal Code), including checks against registries for VAT payers, inactive entities, split VAT appliers, and RO e-Factura participants. The schema accounts for potential multiple TVA periods (modeled as a vector, as seen in payloads where "perioade_TVA" can be an array), and uses optional fields to handle empty or null values common in responses.

Fields are named in snake_case to match the JSON structure, with types inferred from the data (e.g., CUI as u64, dates as strings in YYYY-MM-DD format, booleans for status flags). Serde is used for serialization/deserialization, making it suitable for API integrations.


This schema is self-contained and can represent any entity from ANAF's registry, including companies (SRL, SA), authorized natural persons (PFA), or other forms. For arrays like `perioade_tva`, it supports historical data if multiple periods exist. Optional fields (using `#[serde(default)]`) handle cases where data is absent, as seen in the payload.

| Field Category | Key Fields | Description | Data from Payload |
|---------------|------------|-------------|-------------------|
| **General Data** | cui, denumire, data_inregistrare, nr_reg_com, cod_caen | Core identification and registration info. | CUI: 49201783, Name: SZEMERECZKI PETRU-NICOLAE PERSOANĂ FIZICĂ AUTORIZATĂ, Reg Date: 2023-11-28, Trade Reg: F40/8875/2023, CAEN: 6201 |
| **VAT Registration** | scp_tva, perioade_tva | Status and history of VAT purposes registration. | scp_tva: false, perioade_tva: [] (no periods) |
| **VAT at Collection** | status_tva_incasare, data_inceput_tva_inc | Application of cash-based VAT. | status_tva_incasare: false, All dates empty |
| **Inactivity** | status_inactivi, data_radiere | Inactivity or deregistration flags. | status_inactivi: false, All dates empty |
| **Split VAT** | status_split_tva, data_inceput_split_tva | Split payment system status. | status_split_tva: false, All dates empty |
| **Addresses** | adresa_sediu_social, adresa_domiciliu_fiscal | Detailed components for headquarters and fiscal addresses. | Both addresses: București, Sector 1, B-dul Bucureștii Noi 136, Parter, Postal: 12366 |

Key Citations:
- [ANAF Web Services Documentation (V9)](https://static.anaf.ro/static/10/Anaf/Informatii_R/Servicii_web/doc_WS_V9.txt)
- [ANAF PlatitorTvaRest API Usage](https://community.alteryx.com/t5/Alteryx-Designer-Desktop-Discussions/Connection-with-Romanian-Tax-Administration-website-for-local/td-p/1138351)
- [ANAF Services Overview](https://www.anaf.ro/anaf/internet/ANAF/servicii_online/servicii_web_anaf)