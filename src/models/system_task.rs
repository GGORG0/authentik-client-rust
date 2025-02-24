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

/// SystemTask : Serialize TaskInfo and TaskResult
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemTask {
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Get full name with UID
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: String,
    #[serde(rename = "finish_timestamp")]
    pub finish_timestamp: String,
    #[serde(rename = "duration")]
    pub duration: f64,
    #[serde(rename = "status")]
    pub status: models::SystemTaskStatusEnum,
    #[serde(rename = "messages")]
    pub messages: Vec<models::LogEvent>,
    #[serde(
        rename = "expires",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires: Option<Option<String>>,
    #[serde(rename = "expiring", skip_serializing_if = "Option::is_none")]
    pub expiring: Option<bool>,
}

impl SystemTask {
    /// Serialize TaskInfo and TaskResult
    pub fn new(
        uuid: uuid::Uuid,
        name: String,
        full_name: String,
        description: String,
        start_timestamp: String,
        finish_timestamp: String,
        duration: f64,
        status: models::SystemTaskStatusEnum,
        messages: Vec<models::LogEvent>,
    ) -> SystemTask {
        SystemTask {
            uuid,
            name,
            full_name,
            uid: None,
            description,
            start_timestamp,
            finish_timestamp,
            duration,
            status,
            messages,
            expires: None,
            expiring: None,
        }
    }
}
