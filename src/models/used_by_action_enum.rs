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
pub enum UsedByActionEnum {
    #[serde(rename = "cascade")]
    Cascade,
    #[serde(rename = "cascade_many")]
    CascadeMany,
    #[serde(rename = "set_null")]
    SetNull,
    #[serde(rename = "set_default")]
    SetDefault,
}

impl std::fmt::Display for UsedByActionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Cascade => write!(f, "cascade"),
            Self::CascadeMany => write!(f, "cascade_many"),
            Self::SetNull => write!(f, "set_null"),
            Self::SetDefault => write!(f, "set_default"),
        }
    }
}

impl Default for UsedByActionEnum {
    fn default() -> UsedByActionEnum {
        Self::Cascade
    }
}
