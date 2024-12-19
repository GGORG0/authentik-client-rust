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

/// GenericError : Generic API Error
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericError {
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl GenericError {
    /// Generic API Error
    pub fn new(detail: String) -> GenericError {
        GenericError { detail, code: None }
    }
}
