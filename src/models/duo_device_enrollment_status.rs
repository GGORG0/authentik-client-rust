/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DuoDeviceEnrollmentStatus {
    #[serde(rename = "duo_response")]
    pub duo_response: models::DuoResponseEnum,
}

impl DuoDeviceEnrollmentStatus {
    pub fn new(duo_response: models::DuoResponseEnum) -> DuoDeviceEnrollmentStatus {
        DuoDeviceEnrollmentStatus { duo_response }
    }
}
