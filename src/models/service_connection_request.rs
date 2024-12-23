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

/// ServiceConnectionRequest : ServiceConnection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceConnectionRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// If enabled, use the local connection. Required Docker socket/Kubernetes Integration
    #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
}

impl ServiceConnectionRequest {
    /// ServiceConnection Serializer
    pub fn new(name: String) -> ServiceConnectionRequest {
        ServiceConnectionRequest { name, local: None }
    }
}
