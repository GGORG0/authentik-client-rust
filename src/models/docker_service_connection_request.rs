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

/// DockerServiceConnectionRequest : DockerServiceConnection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerServiceConnectionRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// If enabled, use the local connection. Required Docker socket/Kubernetes Integration
    #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
    /// Can be in the format of 'unix://<path>' when connecting to a local docker daemon, or 'https://<hostname>:2376' when connecting to a remote system.
    #[serde(rename = "url")]
    pub url: String,
    /// CA which the endpoint's Certificate is verified against. Can be left empty for no validation.
    #[serde(
        rename = "tls_verification",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tls_verification: Option<Option<uuid::Uuid>>,
    /// Certificate/Key used for authentication. Can be left empty for no authentication.
    #[serde(
        rename = "tls_authentication",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tls_authentication: Option<Option<uuid::Uuid>>,
}

impl DockerServiceConnectionRequest {
    /// DockerServiceConnection Serializer
    pub fn new(name: String, url: String) -> DockerServiceConnectionRequest {
        DockerServiceConnectionRequest {
            name,
            local: None,
            url,
            tls_verification: None,
            tls_authentication: None,
        }
    }
}
