/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// DummyStage : DummyStage Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DummyStage {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Get object type so that we know how to edit the object
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
    #[serde(rename = "flow_set", skip_serializing_if = "Option::is_none")]
    pub flow_set: Option<Vec<models::FlowSet>>,
    #[serde(rename = "throw_error", skip_serializing_if = "Option::is_none")]
    pub throw_error: Option<bool>,
}

impl DummyStage {
    /// DummyStage Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
    ) -> DummyStage {
        DummyStage {
            pk,
            name,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            flow_set: None,
            throw_error: None,
        }
    }
}
