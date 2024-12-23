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

/// PatchedRacPropertyMappingRequest : RACPropertyMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedRacPropertyMappingRequest {
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(
        rename = "managed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed: Option<Option<String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "static_settings", skip_serializing_if = "Option::is_none")]
    pub static_settings: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedRacPropertyMappingRequest {
    /// RACPropertyMapping Serializer
    pub fn new() -> PatchedRacPropertyMappingRequest {
        PatchedRacPropertyMappingRequest {
            managed: None,
            name: None,
            expression: None,
            static_settings: None,
        }
    }
}
