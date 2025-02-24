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

/// BlueprintInstance : Info about a single blueprint instance file
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintInstance {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "context",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub context: Option<Option<serde_json::Value>>,
    #[serde(rename = "last_applied")]
    pub last_applied: String,
    #[serde(rename = "last_applied_hash")]
    pub last_applied_hash: String,
    #[serde(rename = "status")]
    pub status: models::BlueprintInstanceStatusEnum,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "managed_models")]
    pub managed_models: Vec<String>,
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl BlueprintInstance {
    /// Info about a single blueprint instance file
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        last_applied: String,
        last_applied_hash: String,
        status: models::BlueprintInstanceStatusEnum,
        managed_models: Vec<String>,
        metadata: Option<serde_json::Value>,
    ) -> BlueprintInstance {
        BlueprintInstance {
            pk,
            name,
            path: None,
            context: None,
            last_applied,
            last_applied_hash,
            status,
            enabled: None,
            managed_models,
            metadata,
            content: None,
        }
    }
}
