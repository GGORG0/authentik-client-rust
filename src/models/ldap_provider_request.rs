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

/// LdapProviderRequest : LDAPProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapProviderRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// Flow used for authentication when the associated application is accessed by an un-authenticated user.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow used when authorizing this provider.
    #[serde(rename = "authorization_flow")]
    pub authorization_flow: uuid::Uuid,
    /// Flow used ending the session from a provider.
    #[serde(rename = "invalidation_flow")]
    pub invalidation_flow: uuid::Uuid,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// DN under which objects are accessible.
    #[serde(rename = "base_dn", skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,
    #[serde(
        rename = "certificate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate: Option<Option<uuid::Uuid>>,
    #[serde(rename = "tls_server_name", skip_serializing_if = "Option::is_none")]
    pub tls_server_name: Option<String>,
    /// The start for uidNumbers, this number is added to the user.pk to make sure that the numbers aren't too low for POSIX users. Default is 2000 to ensure that we don't collide with local users uidNumber
    #[serde(rename = "uid_start_number", skip_serializing_if = "Option::is_none")]
    pub uid_start_number: Option<i32>,
    /// The start for gidNumbers, this number is added to a number generated from the group.pk to make sure that the numbers aren't too low for POSIX groups. Default is 4000 to ensure that we don't collide with local groups or users primary groups gidNumber
    #[serde(rename = "gid_start_number", skip_serializing_if = "Option::is_none")]
    pub gid_start_number: Option<i32>,
    #[serde(rename = "search_mode", skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<models::LdapapiAccessMode>,
    #[serde(rename = "bind_mode", skip_serializing_if = "Option::is_none")]
    pub bind_mode: Option<models::LdapapiAccessMode>,
    /// When enabled, code-based multi-factor authentication can be used by appending a semicolon and the TOTP code to the password. This should only be enabled if all users that will bind to this provider have a TOTP device configured, as otherwise a password may incorrectly be rejected if it contains a semicolon.
    #[serde(rename = "mfa_support", skip_serializing_if = "Option::is_none")]
    pub mfa_support: Option<bool>,
}

impl LdapProviderRequest {
    /// LDAPProvider Serializer
    pub fn new(name: String, authorization_flow: uuid::Uuid, invalidation_flow: uuid::Uuid) -> LdapProviderRequest {
        LdapProviderRequest {
            name,
            authentication_flow: None,
            authorization_flow,
            invalidation_flow,
            property_mappings: None,
            base_dn: None,
            certificate: None,
            tls_server_name: None,
            uid_start_number: None,
            gid_start_number: None,
            search_mode: None,
            bind_mode: None,
            mfa_support: None,
        }
    }
}
