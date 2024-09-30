/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SelectableStage : Serializer for stages which can be selected by users
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectableStage {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
}

impl SelectableStage {
    /// Serializer for stages which can be selected by users
    pub fn new(pk: uuid::Uuid, name: String, verbose_name: String, meta_model_name: String) -> SelectableStage {
        SelectableStage {
            pk,
            name,
            verbose_name,
            meta_model_name,
        }
    }
}
