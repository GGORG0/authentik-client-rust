/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedLdapSourcePropertyMappingList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::LdapSourcePropertyMapping>,
}

impl PaginatedLdapSourcePropertyMappingList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::LdapSourcePropertyMapping>,
    ) -> PaginatedLdapSourcePropertyMappingList {
        PaginatedLdapSourcePropertyMappingList { pagination, results }
    }
}
