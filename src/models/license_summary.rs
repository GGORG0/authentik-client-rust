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

/// LicenseSummary : Serializer for license status
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LicenseSummary {
    #[serde(rename = "internal_users")]
    pub internal_users: i32,
    #[serde(rename = "external_users")]
    pub external_users: i32,
    #[serde(rename = "status")]
    pub status: models::LicenseSummaryStatusEnum,
    #[serde(rename = "latest_valid")]
    pub latest_valid: String,
    #[serde(rename = "license_flags")]
    pub license_flags: Vec<models::LicenseFlagsEnum>,
}

impl LicenseSummary {
    /// Serializer for license status
    pub fn new(
        internal_users: i32,
        external_users: i32,
        status: models::LicenseSummaryStatusEnum,
        latest_valid: String,
        license_flags: Vec<models::LicenseFlagsEnum>,
    ) -> LicenseSummary {
        LicenseSummary {
            internal_users,
            external_users,
            status,
            latest_valid,
            license_flags,
        }
    }
}
