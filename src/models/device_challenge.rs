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

/// DeviceChallenge : Single device challenge
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceChallenge {
    #[serde(rename = "device_class")]
    pub device_class: String,
    #[serde(rename = "device_uid")]
    pub device_uid: String,
    #[serde(rename = "challenge")]
    pub challenge: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "last_used", deserialize_with = "Option::deserialize")]
    pub last_used: Option<String>,
}

impl DeviceChallenge {
    /// Single device challenge
    pub fn new(
        device_class: String,
        device_uid: String,
        challenge: std::collections::HashMap<String, serde_json::Value>,
        last_used: Option<String>,
    ) -> DeviceChallenge {
        DeviceChallenge {
            device_class,
            device_uid,
            challenge,
            last_used,
        }
    }
}
