/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TransactionApplicationResponse : Transactional creation response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionApplicationResponse {
    #[serde(rename = "applied")]
    pub applied: bool,
    #[serde(rename = "logs")]
    pub logs: Vec<String>,
}

impl TransactionApplicationResponse {
    /// Transactional creation response
    pub fn new(applied: bool, logs: Vec<String>) -> TransactionApplicationResponse {
        TransactionApplicationResponse { applied, logs }
    }
}
