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

/// Role : Role serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
}

impl Role {
    /// Role serializer
    pub fn new(pk: uuid::Uuid, name: String) -> Role {
        Role { pk, name }
    }
}
