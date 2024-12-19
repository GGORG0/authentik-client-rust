/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PermissionAssignResult : Result from assigning permissions to a user/role
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionAssignResult {
    #[serde(rename = "id")]
    pub id: String,
}

impl PermissionAssignResult {
    /// Result from assigning permissions to a user/role
    pub fn new(id: String) -> PermissionAssignResult {
        PermissionAssignResult { id }
    }
}
