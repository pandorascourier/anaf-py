use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub enum VatPayerApiVersion {
    #[default]
    V9,
    V8,
    V7,
}

impl VatPayerApiVersion {
    pub fn latest() -> Self {
        Self::default()
    }

    pub fn all() -> Vec<Self> {
        vec![Self::V9, Self::V8, Self::V7]
    }

    /// Returns the URL path format for this version.
    /// V9 uses a different URL structure than V7/V8.
    pub fn url_path(&self, base_url: &str) -> String {
        match self {
            // V9: /api/PlatitorTvaRest/v9/tva
            VatPayerApiVersion::V9 => {
                format!("{}/api/PlatitorTvaRest/v9/tva", base_url)
            }
            // V7/V8: /PlatitorTvaRest/api/{version}/ws/tva
            VatPayerApiVersion::V8 | VatPayerApiVersion::V7 => {
                format!("{}/PlatitorTvaRest/api/{}/ws/tva", base_url, self)
            }
        }
    }

    /// Returns the maximum number of CUIs allowed per request.
    pub fn max_cuis(&self) -> usize {
        match self {
            VatPayerApiVersion::V9 => 100,
            VatPayerApiVersion::V8 | VatPayerApiVersion::V7 => 500,
        }
    }
}

impl Display for VatPayerApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VatPayerApiVersion::V9 => "v9",
                VatPayerApiVersion::V8 => "v8",
                VatPayerApiVersion::V7 => "v7",
            }
        )
    }
}

