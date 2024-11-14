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

/// TenantRequest : Tenant Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantRequest {
    #[serde(rename = "schema_name")]
    pub schema_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ready", skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

impl TenantRequest {
    /// Tenant Serializer
    pub fn new(schema_name: String, name: String) -> TenantRequest {
        TenantRequest {
            schema_name,
            name,
            ready: None,
        }
    }
}
