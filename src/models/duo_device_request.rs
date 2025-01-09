/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DuoDeviceRequest : Serializer for Duo authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DuoDeviceRequest {
    /// The human-readable name of this device.
    #[serde(rename = "name")]
    pub name: String,
}

impl DuoDeviceRequest {
    /// Serializer for Duo authenticator devices
    pub fn new(name: String) -> DuoDeviceRequest {
        DuoDeviceRequest { name }
    }
}
