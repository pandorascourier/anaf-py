use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiRequest {
    #[serde(rename = "cui")]
    pub registration_code: usize,
    #[serde(rename = "data")]
    pub when: NaiveDate,
}

impl ApiRequest {
    pub fn new(registration_code: usize, when: NaiveDate) -> Self {
        Self {
            registration_code,
            when,
        }
    }
}
