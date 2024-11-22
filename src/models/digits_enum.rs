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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DigitsEnum {
    #[serde(rename = "6")]
    Variant6,
    #[serde(rename = "8")]
    Variant8,
}

impl std::fmt::Display for DigitsEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant6 => write!(f, "6"),
            Self::Variant8 => write!(f, "8"),
        }
    }
}

impl Default for DigitsEnum {
    fn default() -> DigitsEnum {
        Self::Variant6
    }
}
