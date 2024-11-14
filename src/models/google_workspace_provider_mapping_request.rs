/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GoogleWorkspaceProviderMappingRequest : GoogleWorkspaceProviderMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoogleWorkspaceProviderMappingRequest {
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(
        rename = "managed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expression")]
    pub expression: String,
}

impl GoogleWorkspaceProviderMappingRequest {
    /// GoogleWorkspaceProviderMapping Serializer
    pub fn new(name: String, expression: String) -> GoogleWorkspaceProviderMappingRequest {
        GoogleWorkspaceProviderMappingRequest {
            managed: None,
            name,
            expression,
        }
    }
}
