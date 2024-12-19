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

/// ExtraUserObjectPermission : User permission with additional object-related data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtraUserObjectPermission {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "codename")]
    pub codename: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "app_label")]
    pub app_label: String,
    #[serde(rename = "object_pk")]
    pub object_pk: String,
    #[serde(rename = "name")]
    pub name: String,
    /// Get app label from permission's model
    #[serde(rename = "app_label_verbose")]
    pub app_label_verbose: String,
    /// Get model label from permission's model
    #[serde(rename = "model_verbose")]
    pub model_verbose: String,
    /// Get model description from attached model. This operation takes at least one additional query, and the description is only shown if the user/role has the view_ permission on the object
    #[serde(rename = "object_description", deserialize_with = "Option::deserialize")]
    pub object_description: Option<String>,
}

impl ExtraUserObjectPermission {
    /// User permission with additional object-related data
    pub fn new(
        id: i32,
        codename: String,
        model: String,
        app_label: String,
        object_pk: String,
        name: String,
        app_label_verbose: String,
        model_verbose: String,
        object_description: Option<String>,
    ) -> ExtraUserObjectPermission {
        ExtraUserObjectPermission {
            id,
            codename,
            model,
            app_label,
            object_pk,
            name,
            app_label_verbose,
            model_verbose,
            object_description,
        }
    }
}
