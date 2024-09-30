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
pub struct PaginatedSamlSourceList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::SamlSource>,
}

impl PaginatedSamlSourceList {
    pub fn new(pagination: models::Pagination, results: Vec<models::SamlSource>) -> PaginatedSamlSourceList {
        PaginatedSamlSourceList { pagination, results }
    }
}
