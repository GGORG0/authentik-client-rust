/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedRacProviderRequest : RACProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedRacProviderRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Flow used for authentication when the associated application is accessed by an un-authenticated user.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow used when authorizing this provider.
    #[serde(rename = "authorization_flow", skip_serializing_if = "Option::is_none")]
    pub authorization_flow: Option<uuid::Uuid>,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(
        rename = "settings",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub settings: Option<Option<serde_json::Value>>,
    /// Determines how long a session lasts. Default of 0 means that the sessions lasts until the browser is closed. (Format: hours=-1;minutes=-2;seconds=-3)
    #[serde(rename = "connection_expiry", skip_serializing_if = "Option::is_none")]
    pub connection_expiry: Option<String>,
    /// When set to true, connection tokens will be deleted upon disconnect.
    #[serde(rename = "delete_token_on_disconnect", skip_serializing_if = "Option::is_none")]
    pub delete_token_on_disconnect: Option<bool>,
}

impl PatchedRacProviderRequest {
    /// RACProvider Serializer
    pub fn new() -> PatchedRacProviderRequest {
        PatchedRacProviderRequest {
            name: None,
            authentication_flow: None,
            authorization_flow: None,
            property_mappings: None,
            settings: None,
            connection_expiry: None,
            delete_token_on_disconnect: None,
        }
    }
}
