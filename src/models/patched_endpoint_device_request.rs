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

/// PatchedEndpointDeviceRequest : Serializer for Endpoint authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedEndpointDeviceRequest {
    #[serde(rename = "pk", skip_serializing_if = "Option::is_none")]
    pub pk: Option<uuid::Uuid>,
    /// The human-readable name of this device.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchedEndpointDeviceRequest {
    /// Serializer for Endpoint authenticator devices
    pub fn new() -> PatchedEndpointDeviceRequest {
        PatchedEndpointDeviceRequest { pk: None, name: None }
    }
}
