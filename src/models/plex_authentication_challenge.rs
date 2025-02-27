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

/// PlexAuthenticationChallenge : Challenge shown to the user in identification stage
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlexAuthenticationChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl PlexAuthenticationChallenge {
    /// Challenge shown to the user in identification stage
    pub fn new(client_id: String, slug: String) -> PlexAuthenticationChallenge {
        PlexAuthenticationChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            client_id,
            slug,
        }
    }
}
