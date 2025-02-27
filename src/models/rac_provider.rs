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

/// RacProvider : RACProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RacProvider {
    #[serde(rename = "pk")]
    pub pk: i32,
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
    #[serde(rename = "property_mappings", skip_serializing_if = "Option::is_none")]
    pub property_mappings: Option<Vec<uuid::Uuid>>,
    /// Get object component so that we know how to edit the object
    #[serde(rename = "component")]
    pub component: String,
    /// Internal application name, used in URLs.
    #[serde(rename = "assigned_application_slug")]
    pub assigned_application_slug: String,
    /// Application's display Name.
    #[serde(rename = "assigned_application_name")]
    pub assigned_application_name: String,
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
    #[serde(
        rename = "settings",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub settings: Option<Option<serde_json::Value>>,
    #[serde(rename = "outpost_set")]
    pub outpost_set: Vec<String>,
    /// Determines how long a session lasts. Default of 0 means that the sessions lasts until the browser is closed. (Format: hours=-1;minutes=-2;seconds=-3)
    #[serde(rename = "connection_expiry", skip_serializing_if = "Option::is_none")]
    pub connection_expiry: Option<String>,
    /// When set to true, connection tokens will be deleted upon disconnect.
    #[serde(rename = "delete_token_on_disconnect", skip_serializing_if = "Option::is_none")]
    pub delete_token_on_disconnect: Option<bool>,
}

impl RacProvider {
    /// RACProvider Serializer
    pub fn new(
        pk: i32,
        name: String,
        authorization_flow: uuid::Uuid,
        component: String,
        assigned_application_slug: String,
        assigned_application_name: String,
        assigned_backchannel_application_slug: String,
        assigned_backchannel_application_name: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        outpost_set: Vec<String>,
    ) -> RacProvider {
        RacProvider {
            pk,
            name,
            authentication_flow: None,
            authorization_flow,
            property_mappings: None,
            component,
            assigned_application_slug,
            assigned_application_name,
            assigned_backchannel_application_slug,
            assigned_backchannel_application_name,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            settings: None,
            outpost_set,
            connection_expiry: None,
            delete_token_on_disconnect: None,
        }
    }
}
