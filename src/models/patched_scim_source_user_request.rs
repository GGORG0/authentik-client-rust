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

/// PatchedScimSourceUserRequest : SCIMSourceUser Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedScimSourceUserRequest {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<i32>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<uuid::Uuid>,
    #[serde(
        rename = "attributes",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<Option<serde_json::Value>>,
}

impl PatchedScimSourceUserRequest {
    /// SCIMSourceUser Serializer
    pub fn new() -> PatchedScimSourceUserRequest {
        PatchedScimSourceUserRequest {
            id: None,
            user: None,
            source: None,
            attributes: None,
        }
    }
}
