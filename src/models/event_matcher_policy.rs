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

/// EventMatcherPolicy : Event Matcher Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventMatcherPolicy {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
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
    /// Return objects policy is bound to
    #[serde(rename = "bound_to")]
    pub bound_to: i32,
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

impl EventMatcherPolicy {
    /// Event Matcher Policy Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        bound_to: i32,
    ) -> EventMatcherPolicy {
        EventMatcherPolicy {
            pk,
            name,
            execution_logging: None,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            bound_to,
            action: None,
            client_ip: None,
            app: None,
            model: None,
        }
    }
}
