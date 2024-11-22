/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedUserOAuthSourceConnectionRequest : OAuth Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedUserOAuthSourceConnectionRequest {
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(
        rename = "access_token",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<Option<String>>,
}

impl PatchedUserOAuthSourceConnectionRequest {
    /// OAuth Source Serializer
    pub fn new() -> PatchedUserOAuthSourceConnectionRequest {
        PatchedUserOAuthSourceConnectionRequest {
            identifier: None,
            access_token: None,
        }
    }
}
