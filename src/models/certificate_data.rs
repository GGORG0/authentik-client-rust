/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CertificateData : Get CertificateKeyPair's data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateData {
    #[serde(rename = "data")]
    pub data: String,
}

impl CertificateData {
    /// Get CertificateKeyPair's data
    pub fn new(data: String) -> CertificateData {
        CertificateData { data }
    }
}
