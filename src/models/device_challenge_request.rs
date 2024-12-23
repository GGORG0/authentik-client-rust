/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DeviceChallengeRequest : Single device challenge
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceChallengeRequest {
    #[serde(rename = "device_class")]
    pub device_class: String,
    #[serde(rename = "device_uid")]
    pub device_uid: String,
    #[serde(rename = "challenge")]
    pub challenge: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "last_used", deserialize_with = "Option::deserialize")]
    pub last_used: Option<String>,
}

impl DeviceChallengeRequest {
    /// Single device challenge
    pub fn new(
        device_class: String,
        device_uid: String,
        challenge: std::collections::HashMap<String, serde_json::Value>,
        last_used: Option<String>,
    ) -> DeviceChallengeRequest {
        DeviceChallengeRequest {
            device_class,
            device_uid,
            challenge,
            last_used,
        }
    }
}
