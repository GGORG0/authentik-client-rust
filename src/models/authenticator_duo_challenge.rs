/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AuthenticatorDuoChallenge : Duo Challenge
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorDuoChallenge {
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
    #[serde(rename = "activation_barcode")]
    pub activation_barcode: String,
    #[serde(rename = "activation_code")]
    pub activation_code: String,
    #[serde(rename = "stage_uuid")]
    pub stage_uuid: String,
}

impl AuthenticatorDuoChallenge {
    /// Duo Challenge
    pub fn new(
        pending_user: String,
        pending_user_avatar: String,
        activation_barcode: String,
        activation_code: String,
        stage_uuid: String,
    ) -> AuthenticatorDuoChallenge {
        AuthenticatorDuoChallenge {
            flow_info: None,
            component: None,
            response_errors: None,
            pending_user,
            pending_user_avatar,
            activation_barcode,
            activation_code,
            stage_uuid,
        }
    }
}
