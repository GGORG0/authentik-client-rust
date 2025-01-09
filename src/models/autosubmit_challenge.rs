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

/// AutosubmitChallenge : Autosubmit challenge used to send and navigate a POST request
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutosubmitChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "attrs")]
    pub attrs: std::collections::HashMap<String, String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl AutosubmitChallenge {
    /// Autosubmit challenge used to send and navigate a POST request
    pub fn new(url: String, attrs: std::collections::HashMap<String, String>) -> AutosubmitChallenge {
        AutosubmitChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            url,
            attrs,
            title: None,
        }
    }
}
