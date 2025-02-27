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

/// ConsentPermission : Permission used for consent
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsentPermission {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: String,
}

impl ConsentPermission {
    /// Permission used for consent
    pub fn new(name: String, id: String) -> ConsentPermission {
        ConsentPermission { name, id }
    }
}
