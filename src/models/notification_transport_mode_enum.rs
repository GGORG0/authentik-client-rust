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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationTransportModeEnum {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "webhook")]
    Webhook,
    #[serde(rename = "webhook_slack")]
    WebhookSlack,
    #[serde(rename = "email")]
    Email,
}

impl std::fmt::Display for NotificationTransportModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Webhook => write!(f, "webhook"),
            Self::WebhookSlack => write!(f, "webhook_slack"),
            Self::Email => write!(f, "email"),
        }
    }
}

impl Default for NotificationTransportModeEnum {
    fn default() -> NotificationTransportModeEnum {
        Self::Local
    }
}
