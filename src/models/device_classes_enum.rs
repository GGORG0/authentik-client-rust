/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeviceClassesEnum {
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "webauthn")]
    Webauthn,
    #[serde(rename = "duo")]
    Duo,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "email")]
    Email,
}

impl std::fmt::Display for DeviceClassesEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Static => write!(f, "static"),
            Self::Totp => write!(f, "totp"),
            Self::Webauthn => write!(f, "webauthn"),
            Self::Duo => write!(f, "duo"),
            Self::Sms => write!(f, "sms"),
            Self::Email => write!(f, "email"),
        }
    }
}

impl Default for DeviceClassesEnum {
    fn default() -> DeviceClassesEnum {
        Self::Static
    }
}
