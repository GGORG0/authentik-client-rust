/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedMicrosoftEntraProviderUserList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::MicrosoftEntraProviderUser>,
}

impl PaginatedMicrosoftEntraProviderUserList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::MicrosoftEntraProviderUser>,
    ) -> PaginatedMicrosoftEntraProviderUserList {
        PaginatedMicrosoftEntraProviderUserList { pagination, results }
    }
}
