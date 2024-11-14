/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DockerServiceConnection : DockerServiceConnection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerServiceConnection {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// If enabled, use the local connection. Required Docker socket/Kubernetes Integration
    #[serde(rename = "local", skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
    #[serde(rename = "component")]
    pub component: String,
    /// Return object's verbose_name
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    /// Return object's plural verbose_name
    #[serde(rename = "verbose_name_plural")]
    pub verbose_name_plural: String,
    /// Return internal model name
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
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

impl DockerServiceConnection {
    /// DockerServiceConnection Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        url: String,
    ) -> DockerServiceConnection {
        DockerServiceConnection {
            pk,
            name,
            local: None,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            url,
            tls_verification: None,
            tls_authentication: None,
        }
    }
}
