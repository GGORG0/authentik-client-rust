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

/// IdentificationStageRequest : IdentificationStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentificationStageRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSetRequest>>,
    /// Fields of the user object to match against. (Hold shift to select multiple options)
    #[serde(rename = "user_fields", skip_serializing_if = "Option::is_none")]
    pub user_fields: Option<Vec<models::UserFieldsEnum>>,
    /// When set, shows a password field, instead of showing the password field as separate step.
    #[serde(
        rename = "password_stage",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub password_stage: Option<Option<uuid::Uuid>>,
    /// When set, adds functionality exactly like a Captcha stage, but baked into the Identification stage.
    #[serde(
        rename = "captcha_stage",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub captcha_stage: Option<Option<uuid::Uuid>>,
    /// When enabled, user fields are matched regardless of their casing.
    #[serde(rename = "case_insensitive_matching", skip_serializing_if = "Option::is_none")]
    pub case_insensitive_matching: Option<bool>,
    /// When a valid username/email has been entered, and this option is enabled, the user's username and avatar will be shown. Otherwise, the text that the user entered will be shown
    #[serde(rename = "show_matched_user", skip_serializing_if = "Option::is_none")]
    pub show_matched_user: Option<bool>,
    /// Optional enrollment flow, which is linked at the bottom of the page.
    #[serde(
        rename = "enrollment_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enrollment_flow: Option<Option<uuid::Uuid>>,
    /// Optional recovery flow, which is linked at the bottom of the page.
    #[serde(
        rename = "recovery_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_flow: Option<Option<uuid::Uuid>>,
    /// Optional passwordless flow, which is linked at the bottom of the page.
    #[serde(
        rename = "passwordless_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub passwordless_flow: Option<Option<uuid::Uuid>>,
    /// Specify which sources should be shown.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "show_source_labels", skip_serializing_if = "Option::is_none")]
    pub show_source_labels: Option<bool>,
    /// When enabled, the stage will succeed and continue even when incorrect user info is entered.
    #[serde(rename = "pretend_user_exists", skip_serializing_if = "Option::is_none")]
    pub pretend_user_exists: Option<bool>,
}

impl IdentificationStageRequest {
    /// IdentificationStage Serializer
    pub fn new(name: String) -> IdentificationStageRequest {
        IdentificationStageRequest {
            name,
            flow_set: None,
            user_fields: None,
            password_stage: None,
            captcha_stage: None,
            case_insensitive_matching: None,
            show_matched_user: None,
            enrollment_flow: None,
            recovery_flow: None,
            passwordless_flow: None,
            sources: None,
            show_source_labels: None,
            pretend_user_exists: None,
        }
    }
}
