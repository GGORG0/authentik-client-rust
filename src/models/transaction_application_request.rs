/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TransactionApplicationRequest : Serializer for creating a provider and an application in one transaction
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionApplicationRequest {
    #[serde(rename = "app")]
    pub app: models::ApplicationRequest,
    #[serde(rename = "provider_model")]
    pub provider_model: models::ProviderModelEnum,
    #[serde(rename = "provider")]
    pub provider: models::ModelRequest,
    #[serde(rename = "policy_bindings", skip_serializing_if = "Option::is_none")]
    pub policy_bindings: Option<Vec<models::TransactionPolicyBindingRequest>>,
}

impl TransactionApplicationRequest {
    /// Serializer for creating a provider and an application in one transaction
    pub fn new(
        app: models::ApplicationRequest,
        provider_model: models::ProviderModelEnum,
        provider: models::ModelRequest,
    ) -> TransactionApplicationRequest {
        TransactionApplicationRequest {
            app,
            provider_model,
            provider,
            policy_bindings: None,
        }
    }
}
