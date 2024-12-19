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

/// SamlPropertyMappingRequest : SAMLPropertyMapping Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamlPropertyMappingRequest {
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
    #[serde(rename = "saml_name")]
    pub saml_name: String,
    #[serde(
        rename = "friendly_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub friendly_name: Option<Option<String>>,
}

impl SamlPropertyMappingRequest {
    /// SAMLPropertyMapping Serializer
    pub fn new(name: String, expression: String, saml_name: String) -> SamlPropertyMappingRequest {
        SamlPropertyMappingRequest {
            managed: None,
            name,
            expression,
            saml_name,
            friendly_name: None,
        }
    }
}
