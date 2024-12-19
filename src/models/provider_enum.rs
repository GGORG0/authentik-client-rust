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
pub enum ProviderEnum {
    #[serde(rename = "twilio")]
    Twilio,
    #[serde(rename = "generic")]
    Generic,
}

impl std::fmt::Display for ProviderEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Twilio => write!(f, "twilio"),
            Self::Generic => write!(f, "generic"),
        }
    }
}

impl Default for ProviderEnum {
    fn default() -> ProviderEnum {
        Self::Twilio
    }
}
