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

/// OutpostRequest : Outpost Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutpostRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: models::OutpostTypeEnum,
    #[serde(rename = "providers")]
    pub providers: Vec<i32>,
    /// Select Service-Connection authentik should use to manage this outpost. Leave empty if authentik should not handle the deployment.
    #[serde(
        rename = "service_connection",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_connection: Option<Option<uuid::Uuid>>,
    #[serde(rename = "config")]
    pub config: std::collections::HashMap<String, serde_json::Value>,
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(
        rename = "managed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed: Option<Option<String>>,
}

impl OutpostRequest {
    /// Outpost Serializer
    pub fn new(
        name: String,
        r#type: models::OutpostTypeEnum,
        providers: Vec<i32>,
        config: std::collections::HashMap<String, serde_json::Value>,
    ) -> OutpostRequest {
        OutpostRequest {
            name,
            r#type,
            providers,
            service_connection: None,
            config,
            managed: None,
        }
    }
}
