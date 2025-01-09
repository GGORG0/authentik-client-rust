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

/// LdapCheckAccess : Base serializer class which doesn't implement create/update methods
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapCheckAccess {
    #[serde(rename = "has_search_permission", skip_serializing_if = "Option::is_none")]
    pub has_search_permission: Option<bool>,
    #[serde(rename = "access")]
    pub access: models::PolicyTestResult,
}

impl LdapCheckAccess {
    /// Base serializer class which doesn't implement create/update methods
    pub fn new(access: models::PolicyTestResult) -> LdapCheckAccess {
        LdapCheckAccess {
            has_search_permission: None,
            access,
        }
    }
}
