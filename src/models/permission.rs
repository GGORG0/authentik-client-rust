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

/// Permission : Global permission
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "codename")]
    pub codename: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "app_label")]
    pub app_label: String,
    /// Human-readable app label
    #[serde(rename = "app_label_verbose")]
    pub app_label_verbose: String,
    /// Human-readable model name
    #[serde(rename = "model_verbose")]
    pub model_verbose: String,
}

impl Permission {
    /// Global permission
    pub fn new(
        id: i32,
        name: String,
        codename: String,
        model: String,
        app_label: String,
        app_label_verbose: String,
        model_verbose: String,
    ) -> Permission {
        Permission {
            id,
            name,
            codename,
            model,
            app_label,
            app_label_verbose,
            model_verbose,
        }
    }
}
