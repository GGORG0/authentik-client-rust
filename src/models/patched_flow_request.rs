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

/// PatchedFlowRequest : Flow Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedFlowRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Visible in the URL.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Shown as the Title in Flow pages.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Decides what this Flow is used for. For example, the Authentication flow is redirect to when an un-authenticated user visits authentik.
    #[serde(rename = "designation", skip_serializing_if = "Option::is_none")]
    pub designation: Option<models::FlowDesignationEnum>,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// Enable compatibility mode, increases compatibility with password managers on mobile devices.
    #[serde(rename = "compatibility_mode", skip_serializing_if = "Option::is_none")]
    pub compatibility_mode: Option<bool>,
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<models::FlowLayoutEnum>,
    /// Configure what should happen when a flow denies access to a user.
    #[serde(rename = "denied_action", skip_serializing_if = "Option::is_none")]
    pub denied_action: Option<models::DeniedActionEnum>,
    /// Required level of authentication and authorization to access a flow.
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<models::AuthenticationEnum>,
}

impl PatchedFlowRequest {
    /// Flow Serializer
    pub fn new() -> PatchedFlowRequest {
        PatchedFlowRequest {
            name: None,
            slug: None,
            title: None,
            designation: None,
            policy_engine_mode: None,
            compatibility_mode: None,
            layout: None,
            denied_action: None,
            authentication: None,
        }
    }
}
