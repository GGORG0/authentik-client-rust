/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedGoogleWorkspaceProviderMappingList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::GoogleWorkspaceProviderMapping>,
}

impl PaginatedGoogleWorkspaceProviderMappingList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::GoogleWorkspaceProviderMapping>,
    ) -> PaginatedGoogleWorkspaceProviderMappingList {
        PaginatedGoogleWorkspaceProviderMappingList { pagination, results }
    }
}
