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

/// PatchedExtraRoleObjectPermissionRequest : User permission with additional object-related data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedExtraRoleObjectPermissionRequest {
    #[serde(rename = "object_pk", skip_serializing_if = "Option::is_none")]
    pub object_pk: Option<String>,
}

impl PatchedExtraRoleObjectPermissionRequest {
    /// User permission with additional object-related data
    pub fn new() -> PatchedExtraRoleObjectPermissionRequest {
        PatchedExtraRoleObjectPermissionRequest { object_pk: None }
    }
}
