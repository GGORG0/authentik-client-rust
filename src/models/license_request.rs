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

/// LicenseRequest : License Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseRequest {
    #[serde(rename = "key")]
    pub key: String,
}

impl LicenseRequest {
    /// License Serializer
    pub fn new(key: String) -> LicenseRequest {
        LicenseRequest { key }
    }
}
