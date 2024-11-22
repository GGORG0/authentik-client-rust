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
pub struct PaginatedWebAuthnDeviceList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::WebAuthnDevice>,
}

impl PaginatedWebAuthnDeviceList {
    pub fn new(pagination: models::Pagination, results: Vec<models::WebAuthnDevice>) -> PaginatedWebAuthnDeviceList {
        PaginatedWebAuthnDeviceList { pagination, results }
    }
}
