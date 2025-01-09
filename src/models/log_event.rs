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

/// LogEvent : Single log message with all context logged.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogEvent {
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "log_level")]
    pub log_level: models::LogLevelEnum,
    #[serde(rename = "logger")]
    pub logger: String,
    #[serde(rename = "event")]
    pub event: String,
    #[serde(rename = "attributes")]
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
}

impl LogEvent {
    /// Single log message with all context logged.
    pub fn new(
        timestamp: String,
        log_level: models::LogLevelEnum,
        logger: String,
        event: String,
        attributes: std::collections::HashMap<String, serde_json::Value>,
    ) -> LogEvent {
        LogEvent {
            timestamp,
            log_level,
            logger,
            event,
            attributes,
        }
    }
}
