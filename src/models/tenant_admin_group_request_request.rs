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

/// TenantAdminGroupRequestRequest : Tenant admin group creation request serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantAdminGroupRequestRequest {
    #[serde(rename = "user")]
    pub user: String,
}

impl TenantAdminGroupRequestRequest {
    /// Tenant admin group creation request serializer
    pub fn new(user: String) -> TenantAdminGroupRequestRequest {
        TenantAdminGroupRequestRequest { user }
    }
}
