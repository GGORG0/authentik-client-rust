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

/// AuthenticatedSessionUserAgent : Get parsed user agent
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedSessionUserAgent {
    #[serde(rename = "device")]
    pub device: models::AuthenticatedSessionUserAgentDevice,
    #[serde(rename = "os")]
    pub os: models::AuthenticatedSessionUserAgentOs,
    #[serde(rename = "user_agent")]
    pub user_agent: models::AuthenticatedSessionUserAgentUserAgent,
    #[serde(rename = "string")]
    pub string: String,
}

impl AuthenticatedSessionUserAgent {
    /// Get parsed user agent
    pub fn new(
        device: models::AuthenticatedSessionUserAgentDevice,
        os: models::AuthenticatedSessionUserAgentOs,
        user_agent: models::AuthenticatedSessionUserAgentUserAgent,
        string: String,
    ) -> AuthenticatedSessionUserAgent {
        AuthenticatedSessionUserAgent {
            device,
            os,
            user_agent,
            string,
        }
    }
}
