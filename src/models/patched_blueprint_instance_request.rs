/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedBlueprintInstanceRequest : Info about a single blueprint instance file
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedBlueprintInstanceRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "context",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub context: Option<Option<serde_json::Value>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl PatchedBlueprintInstanceRequest {
    /// Info about a single blueprint instance file
    pub fn new() -> PatchedBlueprintInstanceRequest {
        PatchedBlueprintInstanceRequest {
            name: None,
            path: None,
            context: None,
            enabled: None,
            content: None,
        }
    }
}
