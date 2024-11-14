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

/// PatchedWebAuthnDeviceRequest : Serializer for WebAuthn authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWebAuthnDeviceRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchedWebAuthnDeviceRequest {
    /// Serializer for WebAuthn authenticator devices
    pub fn new() -> PatchedWebAuthnDeviceRequest {
        PatchedWebAuthnDeviceRequest { name: None }
    }
}
