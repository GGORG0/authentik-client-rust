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

/// AuthenticatedSessionUserAgentUserAgent : User agent browser
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedSessionUserAgentUserAgent {
    #[serde(rename = "family")]
    pub family: String,
    #[serde(rename = "major")]
    pub major: String,
    #[serde(rename = "minor")]
    pub minor: String,
    #[serde(rename = "patch")]
    pub patch: String,
}

impl AuthenticatedSessionUserAgentUserAgent {
    /// User agent browser
    pub fn new(family: String, major: String, minor: String, patch: String) -> AuthenticatedSessionUserAgentUserAgent {
        AuthenticatedSessionUserAgentUserAgent {
            family,
            major,
            minor,
            patch,
        }
    }
}
