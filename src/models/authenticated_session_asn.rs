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

/// AuthenticatedSessionAsn : Get ASN Data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedSessionAsn {
    #[serde(rename = "asn")]
    pub asn: i32,
    #[serde(rename = "as_org", deserialize_with = "Option::deserialize")]
    pub as_org: Option<String>,
    #[serde(rename = "network", deserialize_with = "Option::deserialize")]
    pub network: Option<String>,
}

impl AuthenticatedSessionAsn {
    /// Get ASN Data
    pub fn new(asn: i32, as_org: Option<String>, network: Option<String>) -> AuthenticatedSessionAsn {
        AuthenticatedSessionAsn { asn, as_org, network }
    }
}
