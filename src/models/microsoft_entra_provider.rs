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

/// MicrosoftEntraProvider : MicrosoftEntraProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftEntraProvider {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Property mappings used for group creation/updating.
    #[serde(rename = "property_mappings_group", skip_serializing_if = "Option::is_none")]
    pub property_mappings_group: Option<Vec<uuid::Uuid>>,
    /// Get object component so that we know how to edit the object
    #[serde(rename = "component")]
    pub component: String,
    /// Internal application name, used in URLs.
    #[serde(rename = "assigned_backchannel_application_slug")]
    pub assigned_backchannel_application_slug: String,
    /// Application's display Name.
    #[serde(rename = "assigned_backchannel_application_name")]
    pub assigned_backchannel_application_name: String,
    /// Return object's verbose_name
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    /// Return object's plural verbose_name
    #[serde(rename = "verbose_name_plural")]
    pub verbose_name_plural: String,
    /// Return internal model name
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(rename = "exclude_users_service_account", skip_serializing_if = "Option::is_none")]
    pub exclude_users_service_account: Option<bool>,
    #[serde(
        rename = "filter_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_group: Option<Option<uuid::Uuid>>,
    #[serde(rename = "user_delete_action", skip_serializing_if = "Option::is_none")]
    pub user_delete_action: Option<models::OutgoingSyncDeleteAction>,
    #[serde(rename = "group_delete_action", skip_serializing_if = "Option::is_none")]
    pub group_delete_action: Option<models::OutgoingSyncDeleteAction>,
}

impl MicrosoftEntraProvider {
    /// MicrosoftEntraProvider Serializer
    pub fn new(
        pk: i32,
        name: String,
        component: String,
        assigned_backchannel_application_slug: String,
        assigned_backchannel_application_name: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        client_id: String,
        client_secret: String,
        tenant_id: String,
    ) -> MicrosoftEntraProvider {
        MicrosoftEntraProvider {
            pk,
            name,
            property_mappings: None,
            property_mappings_group: None,
            component,
            assigned_backchannel_application_slug,
            assigned_backchannel_application_name,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            client_id,
            client_secret,
            tenant_id,
            exclude_users_service_account: None,
            filter_group: None,
            user_delete_action: None,
            group_delete_action: None,
        }
    }
}
