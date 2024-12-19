/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LdapSource : LDAP Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LdapSource {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    /// Source's display Name.
    #[serde(rename = "name")]
    pub name: String,
    /// Internal source name, used in URLs.
    #[serde(rename = "slug")]
    pub slug: String,
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
    /// Get object component so that we know how to edit the object
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
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// How the source determines if an existing user should be authenticated or a new user enrolled.
    #[serde(rename = "user_matching_mode", skip_serializing_if = "Option::is_none")]
    pub user_matching_mode: Option<models::UserMatchingModeEnum>,
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(rename = "managed", deserialize_with = "Option::deserialize")]
    pub managed: Option<String>,
    #[serde(rename = "user_path_template", skip_serializing_if = "Option::is_none")]
    pub user_path_template: Option<String>,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "server_uri")]
    pub server_uri: String,
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
    #[serde(rename = "start_tls", skip_serializing_if = "Option::is_none")]
    pub start_tls: Option<bool>,
    #[serde(rename = "sni", skip_serializing_if = "Option::is_none")]
    pub sni: Option<bool>,
    #[serde(rename = "base_dn")]
    pub base_dn: String,
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
    /// Get cached source connectivity
    #[serde(rename = "connectivity", deserialize_with = "Option::deserialize")]
    pub connectivity: Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
}

impl LdapSource {
    /// LDAP Source Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        slug: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        managed: Option<String>,
        icon: String,
        server_uri: String,
        base_dn: String,
        connectivity: Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    ) -> LdapSource {
        LdapSource {
            pk,
            name,
            slug,
            enabled: None,
            authentication_flow: None,
            enrollment_flow: None,
            user_property_mappings: None,
            group_property_mappings: None,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            policy_engine_mode: None,
            user_matching_mode: None,
            managed,
            user_path_template: None,
            icon,
            server_uri,
            peer_certificate: None,
            client_certificate: None,
            bind_cn: None,
            start_tls: None,
            sni: None,
            base_dn,
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
            connectivity,
        }
    }
}
