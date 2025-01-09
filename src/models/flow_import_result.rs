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

/// FlowImportResult : Logs of an attempted flow import
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowImportResult {
    #[serde(rename = "logs")]
    pub logs: Vec<models::LogEvent>,
    #[serde(rename = "success")]
    pub success: bool,
}

impl FlowImportResult {
    /// Logs of an attempted flow import
    pub fn new(logs: Vec<models::LogEvent>, success: bool) -> FlowImportResult {
        FlowImportResult { logs, success }
    }
}
