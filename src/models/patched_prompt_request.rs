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

/// PatchedPromptRequest : Prompt Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedPromptRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of the form field, also used to store the value
    #[serde(rename = "field_key", skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::PromptTypeEnum>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Optionally provide a short hint that describes the expected input value. When creating a fixed choice field, enable interpreting as expression and return a list to return multiple choices.
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Optionally pre-fill the input with an initial value. When creating a fixed choice field, enable interpreting as expression and return a list to return multiple default choices.
    #[serde(rename = "initial_value", skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "promptstage_set", skip_serializing_if = "Option::is_none")]
    pub promptstage_set: Option<Vec<models::StageRequest>>,
    #[serde(rename = "sub_text", skip_serializing_if = "Option::is_none")]
    pub sub_text: Option<String>,
    #[serde(rename = "placeholder_expression", skip_serializing_if = "Option::is_none")]
    pub placeholder_expression: Option<bool>,
    #[serde(rename = "initial_value_expression", skip_serializing_if = "Option::is_none")]
    pub initial_value_expression: Option<bool>,
}

impl PatchedPromptRequest {
    /// Prompt Serializer
    pub fn new() -> PatchedPromptRequest {
        PatchedPromptRequest {
            name: None,
            field_key: None,
            label: None,
            r#type: None,
            required: None,
            placeholder: None,
            initial_value: None,
            order: None,
            promptstage_set: None,
            sub_text: None,
            placeholder_expression: None,
            initial_value_expression: None,
        }
    }
}
