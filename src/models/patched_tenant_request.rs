/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedTenantRequest : Tenant Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedTenantRequest {
    #[serde(rename = "schema_name", skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ready", skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

impl PatchedTenantRequest {
    /// Tenant Serializer
    pub fn new() -> PatchedTenantRequest {
        PatchedTenantRequest {
            schema_name: None,
            name: None,
            ready: None,
        }
    }
}
