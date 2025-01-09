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

/// StagePrompt : Serializer for a single Prompt field
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StagePrompt {
    #[serde(rename = "field_key")]
    pub field_key: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "type")]
    pub r#type: models::PromptTypeEnum,
    #[serde(rename = "required")]
    pub required: bool,
    #[serde(rename = "placeholder")]
    pub placeholder: String,
    #[serde(rename = "initial_value")]
    pub initial_value: String,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "sub_text")]
    pub sub_text: String,
    #[serde(rename = "choices", deserialize_with = "Option::deserialize")]
    pub choices: Option<Vec<String>>,
}

impl StagePrompt {
    /// Serializer for a single Prompt field
    pub fn new(
        field_key: String,
        label: String,
        r#type: models::PromptTypeEnum,
        required: bool,
        placeholder: String,
        initial_value: String,
        order: i32,
        sub_text: String,
        choices: Option<Vec<String>>,
    ) -> StagePrompt {
        StagePrompt {
            field_key,
            label,
            r#type,
            required,
            placeholder,
            initial_value,
            order,
            sub_text,
            choices,
        }
    }
}
