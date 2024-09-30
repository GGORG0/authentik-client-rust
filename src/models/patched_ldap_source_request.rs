/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PatchedLdapSourceRequest : LDAP Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedLdapSourceRequest {
    /// Source's display Name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Internal source name, used in URLs.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Flow to use when authenticating existing users.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow to use when enrolling new users.
    #[serde(
        rename = "enrollment_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enrollment_flow: Option<Option<uuid::Uuid>>,
    #[serde(rename = "user_property_mappings", skip_serializing_if = "Option::is_none")]
    pub user_property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "group_property_mappings", skip_serializing_if = "Option::is_none")]
    pub group_property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// How the source determines if an existing user should be authenticated or a new user enrolled.
    #[serde(rename = "user_matching_mode", skip_serializing_if = "Option::is_none")]
    pub user_matching_mode: Option<models::UserMatchingModeEnum>,
    #[serde(rename = "user_path_template", skip_serializing_if = "Option::is_none")]
    pub user_path_template: Option<String>,
    #[serde(rename = "server_uri", skip_serializing_if = "Option::is_none")]
    pub server_uri: Option<String>,
    /// Optionally verify the LDAP Server's Certificate against the CA Chain in this keypair.
    #[serde(
        rename = "peer_certificate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub peer_certificate: Option<Option<uuid::Uuid>>,
    /// Client certificate to authenticate against the LDAP Server's Certificate.
    #[serde(
        rename = "client_certificate",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_certificate: Option<Option<uuid::Uuid>>,
    #[serde(rename = "bind_cn", skip_serializing_if = "Option::is_none")]
    pub bind_cn: Option<String>,
    #[serde(rename = "bind_password", skip_serializing_if = "Option::is_none")]
    pub bind_password: Option<String>,
    #[serde(rename = "start_tls", skip_serializing_if = "Option::is_none")]
    pub start_tls: Option<bool>,
    #[serde(rename = "sni", skip_serializing_if = "Option::is_none")]
    pub sni: Option<bool>,
    #[serde(rename = "base_dn", skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,
    /// Prepended to Base DN for User-queries.
    #[serde(rename = "additional_user_dn", skip_serializing_if = "Option::is_none")]
    pub additional_user_dn: Option<String>,
    /// Prepended to Base DN for Group-queries.
    #[serde(rename = "additional_group_dn", skip_serializing_if = "Option::is_none")]
    pub additional_group_dn: Option<String>,
    /// Consider Objects matching this filter to be Users.
    #[serde(rename = "user_object_filter", skip_serializing_if = "Option::is_none")]
    pub user_object_filter: Option<String>,
    /// Consider Objects matching this filter to be Groups.
    #[serde(rename = "group_object_filter", skip_serializing_if = "Option::is_none")]
    pub group_object_filter: Option<String>,
    /// Field which contains members of a group.
    #[serde(rename = "group_membership_field", skip_serializing_if = "Option::is_none")]
    pub group_membership_field: Option<String>,
    /// Field which contains a unique Identifier.
    #[serde(rename = "object_uniqueness_field", skip_serializing_if = "Option::is_none")]
    pub object_uniqueness_field: Option<String>,
    /// Update internal authentik password when login succeeds with LDAP
    #[serde(
        rename = "password_login_update_internal_password",
        skip_serializing_if = "Option::is_none"
    )]
    pub password_login_update_internal_password: Option<bool>,
    #[serde(rename = "sync_users", skip_serializing_if = "Option::is_none")]
    pub sync_users: Option<bool>,
    /// When a user changes their password, sync it back to LDAP. This can only be enabled on a single LDAP source.
    #[serde(rename = "sync_users_password", skip_serializing_if = "Option::is_none")]
    pub sync_users_password: Option<bool>,
    #[serde(rename = "sync_groups", skip_serializing_if = "Option::is_none")]
    pub sync_groups: Option<bool>,
    #[serde(
        rename = "sync_parent_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sync_parent_group: Option<Option<uuid::Uuid>>,
}

impl PatchedLdapSourceRequest {
    /// LDAP Source Serializer
    pub fn new() -> PatchedLdapSourceRequest {
        PatchedLdapSourceRequest {
            name: None,
            slug: None,
            enabled: None,
            authentication_flow: None,
            enrollment_flow: None,
            user_property_mappings: None,
            group_property_mappings: None,
            policy_engine_mode: None,
            user_matching_mode: None,
            user_path_template: None,
            server_uri: None,
            peer_certificate: None,
            client_certificate: None,
            bind_cn: None,
            bind_password: None,
            start_tls: None,
            sni: None,
            base_dn: None,
            additional_user_dn: None,
            additional_group_dn: None,
            user_object_filter: None,
            group_object_filter: None,
            group_membership_field: None,
            object_uniqueness_field: None,
            password_login_update_internal_password: None,
            sync_users: None,
            sync_users_password: None,
            sync_groups: None,
            sync_parent_group: None,
        }
    }
}
