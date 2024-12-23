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

/// ConsentChallenge : Challenge info for consent screens
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsentChallenge {
    #[serde(rename = "flow_info", skip_serializing_if = "Option::is_none")]
    pub flow_info: Option<models::ContextualFlowInfo>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "response_errors", skip_serializing_if = "Option::is_none")]
    pub response_errors: Option<std::collections::HashMap<String, Vec<models::ErrorDetail>>>,
    #[serde(rename = "pending_user")]
    pub pending_user: String,
    #[serde(rename = "pending_user_avatar")]
    pub pending_user_avatar: String,
    #[serde(rename = "header_text", skip_serializing_if = "Option::is_none")]
    pub header_text: Option<String>,
    #[serde(rename = "permissions")]
    pub permissions: Vec<models::ConsentPermission>,
    #[serde(rename = "additional_permissions")]
    pub additional_permissions: Vec<models::ConsentPermission>,
    #[serde(rename = "token")]
    pub token: String,
}

impl ConsentChallenge {
    /// Challenge info for consent screens
    pub fn new(
        pending_user: String,
        pending_user_avatar: String,
        permissions: Vec<models::ConsentPermission>,
        additional_permissions: Vec<models::ConsentPermission>,
        token: String,
    ) -> ConsentChallenge {
        ConsentChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            pending_user,
            pending_user_avatar,
            header_text: None,
            permissions,
            additional_permissions,
            token,
        }
    }
}
