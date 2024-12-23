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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatorDuoStageDeviceImportResponse {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "error")]
    pub error: String,
}

impl AuthenticatorDuoStageDeviceImportResponse {
    pub fn new(count: i32, error: String) -> AuthenticatorDuoStageDeviceImportResponse {
        AuthenticatorDuoStageDeviceImportResponse { count, error }
    }
}
