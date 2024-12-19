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

/// StaticDevice : Serializer for static authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticDevice {
    /// The human-readable name of this device.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "token_set")]
    pub token_set: Vec<models::StaticDeviceToken>,
    #[serde(rename = "pk")]
    pub pk: i32,
}

impl StaticDevice {
    /// Serializer for static authenticator devices
    pub fn new(name: String, token_set: Vec<models::StaticDeviceToken>, pk: i32) -> StaticDevice {
        StaticDevice { name, token_set, pk }
    }
}
