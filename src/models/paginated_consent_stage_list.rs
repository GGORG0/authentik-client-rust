/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedConsentStageList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::ConsentStage>,
}

impl PaginatedConsentStageList {
    pub fn new(pagination: models::Pagination, results: Vec<models::ConsentStage>) -> PaginatedConsentStageList {
        PaginatedConsentStageList { pagination, results }
    }
}
