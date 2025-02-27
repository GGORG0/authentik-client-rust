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

/// KerberosSource : Kerberos Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KerberosSource {
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
    /// How the source determines if an existing group should be used or a new group created.
    #[serde(rename = "group_matching_mode", skip_serializing_if = "Option::is_none")]
    pub group_matching_mode: Option<models::GroupMatchingModeEnum>,
    /// Kerberos realm
    #[serde(rename = "realm")]
    pub realm: String,
    /// Custom krb5.conf to use. Uses the system one by default
    #[serde(rename = "krb5_conf", skip_serializing_if = "Option::is_none")]
    pub krb5_conf: Option<String>,
    /// KAdmin server type
    #[serde(rename = "kadmin_type", skip_serializing_if = "Option::is_none")]
    pub kadmin_type: Option<models::KadminTypeEnum>,
    /// Sync users from Kerberos into authentik
    #[serde(rename = "sync_users", skip_serializing_if = "Option::is_none")]
    pub sync_users: Option<bool>,
    /// When a user changes their password, sync it back to Kerberos
    #[serde(rename = "sync_users_password", skip_serializing_if = "Option::is_none")]
    pub sync_users_password: Option<bool>,
    /// Principal to authenticate to kadmin for sync.
    #[serde(rename = "sync_principal", skip_serializing_if = "Option::is_none")]
    pub sync_principal: Option<String>,
    /// Credentials cache to authenticate to kadmin for sync. Must be in the form TYPE:residual
    #[serde(rename = "sync_ccache", skip_serializing_if = "Option::is_none")]
    pub sync_ccache: Option<String>,
    /// Get cached source connectivity
    #[serde(rename = "connectivity", deserialize_with = "Option::deserialize")]
    pub connectivity: Option<std::collections::HashMap<String, String>>,
    /// Force the use of a specific server name for SPNEGO. Must be in the form HTTP@hostname
    #[serde(rename = "spnego_server_name", skip_serializing_if = "Option::is_none")]
    pub spnego_server_name: Option<String>,
    /// Credential cache to use for SPNEGO in form type:residual
    #[serde(rename = "spnego_ccache", skip_serializing_if = "Option::is_none")]
    pub spnego_ccache: Option<String>,
    /// If enabled, the authentik-stored password will be updated upon login with the Kerberos password backend
    #[serde(
        rename = "password_login_update_internal_password",
        skip_serializing_if = "Option::is_none"
    )]
    pub password_login_update_internal_password: Option<bool>,
}

impl KerberosSource {
    /// Kerberos Source Serializer
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
        realm: String,
        connectivity: Option<std::collections::HashMap<String, String>>,
    ) -> KerberosSource {
        KerberosSource {
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
            group_matching_mode: None,
            realm,
            krb5_conf: None,
            kadmin_type: None,
            sync_users: None,
            sync_users_password: None,
            sync_principal: None,
            sync_ccache: None,
            connectivity,
            spnego_server_name: None,
            spnego_ccache: None,
            password_login_update_internal_password: None,
        }
    }
}
