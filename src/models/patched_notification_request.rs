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

/// PatchedNotificationRequest : Notification Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedNotificationRequest {
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<models::EventRequest>,
    #[serde(rename = "seen", skip_serializing_if = "Option::is_none")]
    pub seen: Option<bool>,
}

impl PatchedNotificationRequest {
    /// Notification Serializer
    pub fn new() -> PatchedNotificationRequest {
        PatchedNotificationRequest {
            event: None,
            seen: None,
        }
    }
}
