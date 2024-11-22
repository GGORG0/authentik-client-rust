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

/// StaticDeviceToken : Serializer for static device's tokens
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticDeviceToken {
    #[serde(rename = "token")]
    pub token: String,
}

impl StaticDeviceToken {
    /// Serializer for static device's tokens
    pub fn new(token: String) -> StaticDeviceToken {
        StaticDeviceToken { token }
    }
}
