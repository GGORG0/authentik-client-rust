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

/// RadiusCheckAccess : Base serializer class which doesn't implement create/update methods
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadiusCheckAccess {
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(rename = "access")]
    pub access: models::PolicyTestResult,
}

impl RadiusCheckAccess {
    /// Base serializer class which doesn't implement create/update methods
    pub fn new(access: models::PolicyTestResult) -> RadiusCheckAccess {
        RadiusCheckAccess {
            attributes: None,
            access,
        }
    }
}
