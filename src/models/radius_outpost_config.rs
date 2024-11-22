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

/// RadiusOutpostConfig : RadiusProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadiusOutpostConfig {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "application_slug")]
    pub application_slug: String,
    #[serde(rename = "auth_flow_slug")]
    pub auth_flow_slug: String,
    /// List of CIDRs (comma-separated) that clients can connect from. A more specific CIDR will match before a looser one. Clients connecting from a non-specified CIDR will be dropped.
    #[serde(rename = "client_networks", skip_serializing_if = "Option::is_none")]
    pub client_networks: Option<String>,
    /// Shared secret between clients and server to hash packets.
    #[serde(rename = "shared_secret", skip_serializing_if = "Option::is_none")]
    pub shared_secret: Option<String>,
    /// When enabled, code-based multi-factor authentication can be used by appending a semicolon and the TOTP code to the password. This should only be enabled if all users that will bind to this provider have a TOTP device configured, as otherwise a password may incorrectly be rejected if it contains a semicolon.
    #[serde(rename = "mfa_support", skip_serializing_if = "Option::is_none")]
    pub mfa_support: Option<bool>,
}

impl RadiusOutpostConfig {
    /// RadiusProvider Serializer
    pub fn new(pk: i32, name: String, application_slug: String, auth_flow_slug: String) -> RadiusOutpostConfig {
        RadiusOutpostConfig {
            pk,
            name,
            application_slug,
            auth_flow_slug,
            client_networks: None,
            shared_secret: None,
            mfa_support: None,
        }
    }
}
