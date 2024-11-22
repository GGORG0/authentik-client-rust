/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedScimProviderUserList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::ScimProviderUser>,
}

impl PaginatedScimProviderUserList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::ScimProviderUser>,
    ) -> PaginatedScimProviderUserList {
        PaginatedScimProviderUserList { pagination, results }
    }
}
