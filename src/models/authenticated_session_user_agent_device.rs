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

/// AuthenticatedSessionUserAgentDevice : User agent device
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedSessionUserAgentDevice {
    #[serde(rename = "brand")]
    pub brand: String,
    #[serde(rename = "family")]
    pub family: String,
    #[serde(rename = "model")]
    pub model: String,
}

impl AuthenticatedSessionUserAgentDevice {
    /// User agent device
    pub fn new(brand: String, family: String, model: String) -> AuthenticatedSessionUserAgentDevice {
        AuthenticatedSessionUserAgentDevice { brand, family, model }
    }
}
