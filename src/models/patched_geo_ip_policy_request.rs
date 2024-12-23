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

/// PatchedGeoIpPolicyRequest : GeoIP Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedGeoIpPolicyRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    #[serde(rename = "asns", skip_serializing_if = "Option::is_none")]
    pub asns: Option<Vec<i32>>,
    #[serde(rename = "countries", skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<models::CountryCodeEnum>>,
}

impl PatchedGeoIpPolicyRequest {
    /// GeoIP Policy Serializer
    pub fn new() -> PatchedGeoIpPolicyRequest {
        PatchedGeoIpPolicyRequest {
            name: None,
            execution_logging: None,
            asns: None,
            countries: None,
        }
    }
}
