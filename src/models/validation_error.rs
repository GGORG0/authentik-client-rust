/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ValidationError : Validation Error
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    #[serde(rename = "non_field_errors", skip_serializing_if = "Option::is_none")]
    pub non_field_errors: Option<Vec<String>>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ValidationError {
    /// Validation Error
    pub fn new() -> ValidationError {
        ValidationError {
            non_field_errors: None,
            code: None,
        }
    }
}
