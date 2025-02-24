/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SsfProvider : SSFProvider Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsfProvider {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "name")]
    pub name: String,
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
    /// Key used to sign the SSF Events.
    #[serde(rename = "signing_key")]
    pub signing_key: uuid::Uuid,
    #[serde(rename = "token_obj")]
    pub token_obj: models::Token,
    #[serde(rename = "oidc_auth_providers", skip_serializing_if = "Option::is_none")]
    pub oidc_auth_providers: Option<Vec<i32>>,
    #[serde(rename = "ssf_url", deserialize_with = "Option::deserialize")]
    pub ssf_url: Option<String>,
    #[serde(rename = "event_retention", skip_serializing_if = "Option::is_none")]
    pub event_retention: Option<String>,
}

impl SsfProvider {
    /// SSFProvider Serializer
    pub fn new(
        pk: i32,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        signing_key: uuid::Uuid,
        token_obj: models::Token,
        ssf_url: Option<String>,
    ) -> SsfProvider {
        SsfProvider {
            pk,
            name,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            signing_key,
            token_obj,
            oidc_auth_providers: None,
            ssf_url,
            event_retention: None,
        }
    }
}
