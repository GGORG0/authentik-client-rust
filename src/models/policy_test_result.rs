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

/// PolicyTestResult : result of a policy test
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyTestResult {
    #[serde(rename = "passing")]
    pub passing: bool,
    #[serde(rename = "messages")]
    pub messages: Vec<String>,
    #[serde(rename = "log_messages")]
    pub log_messages: Vec<models::LogEvent>,
}

impl PolicyTestResult {
    /// result of a policy test
    pub fn new(passing: bool, messages: Vec<String>, log_messages: Vec<models::LogEvent>) -> PolicyTestResult {
        PolicyTestResult {
            passing,
            messages,
            log_messages,
        }
    }
}
