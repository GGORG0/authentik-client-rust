/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserKerberosSourceConnectionRequest : Kerberos Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserKerberosSourceConnectionRequest {
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl UserKerberosSourceConnectionRequest {
    /// Kerberos Source Serializer
    pub fn new(user: i32, identifier: String) -> UserKerberosSourceConnectionRequest {
        UserKerberosSourceConnectionRequest { user, identifier }
    }
}
