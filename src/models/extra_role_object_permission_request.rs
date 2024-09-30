/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ExtraRoleObjectPermissionRequest : User permission with additional object-related data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtraRoleObjectPermissionRequest {
    #[serde(rename = "object_pk")]
    pub object_pk: String,
}

impl ExtraRoleObjectPermissionRequest {
    /// User permission with additional object-related data
    pub fn new(object_pk: String) -> ExtraRoleObjectPermissionRequest {
        ExtraRoleObjectPermissionRequest { object_pk }
    }
}
