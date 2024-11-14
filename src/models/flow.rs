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

/// Flow : Flow Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Flow {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "policybindingmodel_ptr_id")]
    pub policybindingmodel_ptr_id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Visible in the URL.
    #[serde(rename = "slug")]
    pub slug: String,
    /// Shown as the Title in Flow pages.
    #[serde(rename = "title")]
    pub title: String,
    /// Decides what this Flow is used for. For example, the Authentication flow is redirect to when an un-authenticated user visits authentik.
    #[serde(rename = "designation")]
    pub designation: models::FlowDesignationEnum,
    /// Get the URL to the background image. If the name is /static or starts with http it is returned as-is
    #[serde(rename = "background")]
    pub background: String,
    #[serde(rename = "stages")]
    pub stages: Vec<uuid::Uuid>,
    #[serde(rename = "policies")]
    pub policies: Vec<uuid::Uuid>,
    /// Get count of cached flows
    #[serde(rename = "cache_count")]
    pub cache_count: i32,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// Enable compatibility mode, increases compatibility with password managers on mobile devices.
    #[serde(rename = "compatibility_mode", skip_serializing_if = "Option::is_none")]
    pub compatibility_mode: Option<bool>,
    /// Get export URL for flow
    #[serde(rename = "export_url")]
    pub export_url: String,
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<models::FlowLayoutEnum>,
    /// Configure what should happen when a flow denies access to a user.
    #[serde(rename = "denied_action", skip_serializing_if = "Option::is_none")]
    pub denied_action: Option<models::DeniedActionEnum>,
    /// Required level of authentication and authorization to access a flow.
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<models::AuthenticationEnum>,
}

impl Flow {
    /// Flow Serializer
    pub fn new(
        pk: uuid::Uuid,
        policybindingmodel_ptr_id: uuid::Uuid,
        name: String,
        slug: String,
        title: String,
        designation: models::FlowDesignationEnum,
        background: String,
        stages: Vec<uuid::Uuid>,
        policies: Vec<uuid::Uuid>,
        cache_count: i32,
        export_url: String,
    ) -> Flow {
        Flow {
            pk,
            policybindingmodel_ptr_id,
            name,
            slug,
            title,
            designation,
            background,
            stages,
            policies,
            cache_count,
            policy_engine_mode: None,
            compatibility_mode: None,
            export_url,
            layout: None,
            denied_action: None,
            authentication: None,
        }
    }
}
