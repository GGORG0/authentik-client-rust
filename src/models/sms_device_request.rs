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

/// SmsDeviceRequest : Serializer for sms authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsDeviceRequest {
    /// The human-readable name of this device.
    #[serde(rename = "name")]
    pub name: String,
}

impl SmsDeviceRequest {
    /// Serializer for sms authenticator devices
    pub fn new(name: String) -> SmsDeviceRequest {
        SmsDeviceRequest { name }
    }
}
