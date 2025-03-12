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

/// ScimProvider : SCIMProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScimProvider {
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
    /// Base URL to SCIM requests, usually ends in /v2
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "verify_certificates", skip_serializing_if = "Option::is_none")]
    pub verify_certificates: Option<bool>,
    /// Authentication token
    #[serde(rename = "token")]
    pub token: String,
    /// Alter authentik behavior for vendor-specific SCIM implementations.
    #[serde(rename = "compatibility_mode", skip_serializing_if = "Option::is_none")]
    pub compatibility_mode: Option<models::CompatibilityModeEnum>,
    #[serde(rename = "exclude_users_service_account", skip_serializing_if = "Option::is_none")]
    pub exclude_users_service_account: Option<bool>,
    #[serde(
        rename = "filter_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_group: Option<Option<uuid::Uuid>>,
    /// When enabled, provider will not modify or create objects in the remote system.
    #[serde(rename = "dry_run", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl ScimProvider {
    /// SCIMProvider Serializer
    pub fn new(
        pk: i32,
        name: String,
        component: String,
        assigned_backchannel_application_slug: String,
        assigned_backchannel_application_name: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        url: String,
        token: String,
    ) -> ScimProvider {
        ScimProvider {
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
            url,
            verify_certificates: None,
            token,
            compatibility_mode: None,
            exclude_users_service_account: None,
            filter_group: None,
            dry_run: None,
        }
    }
}
