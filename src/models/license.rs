/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// License : License Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "license_uuid")]
    pub license_uuid: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "expiry")]
    pub expiry: String,
    #[serde(rename = "internal_users")]
    pub internal_users: i32,
    #[serde(rename = "external_users")]
    pub external_users: i32,
}

impl License {
    /// License Serializer
    pub fn new(
        license_uuid: uuid::Uuid,
        name: String,
        key: String,
        expiry: String,
        internal_users: i32,
        external_users: i32,
    ) -> License {
        License {
            license_uuid,
            name,
            key,
            expiry,
            internal_users,
            external_users,
        }
    }
}
