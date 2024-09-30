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

/// TokenRequest : Token Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenRequest {
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(
        rename = "managed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed: Option<Option<String>>,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<models::IntentEnum>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "expires",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires: Option<Option<String>>,
    #[serde(rename = "expiring", skip_serializing_if = "Option::is_none")]
    pub expiring: Option<bool>,
}

impl TokenRequest {
    /// Token Serializer
    pub fn new(identifier: String) -> TokenRequest {
        TokenRequest {
            managed: None,
            identifier,
            intent: None,
            user: None,
            description: None,
            expires: None,
            expiring: None,
        }
    }
}
