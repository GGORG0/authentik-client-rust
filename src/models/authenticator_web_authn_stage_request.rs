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

/// AuthenticatorWebAuthnStageRequest : AuthenticatorWebAuthnStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorWebAuthnStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Flow used by an authenticated user to configure this Stage. If empty, user will not be able to configure this stage.
    #[serde(
        rename = "configure_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub configure_flow: Option<Option<uuid::Uuid>>,
    #[serde(
        rename = "friendly_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
    #[serde(rename = "user_verification", skip_serializing_if = "Option::is_none")]
    pub user_verification: Option<models::UserVerificationEnum>,
    #[serde(
        rename = "authenticator_attachment",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticator_attachment: Option<Option<models::AuthenticatorAttachmentEnum>>,
    #[serde(rename = "resident_key_requirement", skip_serializing_if = "Option::is_none")]
    pub resident_key_requirement: Option<models::ResidentKeyRequirementEnum>,
    #[serde(rename = "device_type_restrictions", skip_serializing_if = "Option::is_none")]
    pub device_type_restrictions: Option<Vec<uuid::Uuid>>,
}

impl AuthenticatorWebAuthnStageRequest {
    /// AuthenticatorWebAuthnStage Serializer
    pub fn new(name: String) -> AuthenticatorWebAuthnStageRequest {
        AuthenticatorWebAuthnStageRequest {
            name,
            flow_set: None,
            configure_flow: None,
            friendly_name: None,
            user_verification: None,
            authenticator_attachment: None,
            resident_key_requirement: None,
            device_type_restrictions: None,
        }
    }
}
