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
pub struct AuthenticatorDuoStageManualDeviceImportRequest {
    #[serde(rename = "duo_user_id")]
    pub duo_user_id: String,
    #[serde(rename = "username")]
    pub username: String,
}

impl AuthenticatorDuoStageManualDeviceImportRequest {
    pub fn new(duo_user_id: String, username: String) -> AuthenticatorDuoStageManualDeviceImportRequest {
        AuthenticatorDuoStageManualDeviceImportRequest { duo_user_id, username }
    }
}
