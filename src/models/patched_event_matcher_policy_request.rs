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

/// PatchedEventMatcherPolicyRequest : Event Matcher Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedEventMatcherPolicyRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    /// Match created events with this action type. When left empty, all action types will be matched.
    #[serde(
        rename = "action",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub action: Option<Option<models::EventActions>>,
    /// Matches Event's Client IP (strict matching, for network matching use an Expression Policy)
    #[serde(
        rename = "client_ip",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_ip: Option<Option<String>>,
    /// Match events created by selected application. When left empty, all applications are matched.
    #[serde(
        rename = "app",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app: Option<Option<models::AppEnum>>,
    /// Match events created by selected model. When left empty, all models are matched. When an app is selected, all the application's models are matched.
    #[serde(
        rename = "model",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub model: Option<Option<models::ModelEnum>>,
}

impl PatchedEventMatcherPolicyRequest {
    /// Event Matcher Policy Serializer
    pub fn new() -> PatchedEventMatcherPolicyRequest {
        PatchedEventMatcherPolicyRequest {
            name: None,
            execution_logging: None,
            action: None,
            client_ip: None,
            app: None,
            model: None,
        }
    }
}
