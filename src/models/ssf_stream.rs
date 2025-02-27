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

/// SsfStream : SSFStream Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsfStream {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "provider")]
    pub provider: i32,
    #[serde(rename = "provider_obj")]
    pub provider_obj: models::SsfProvider,
    #[serde(rename = "delivery_method")]
    pub delivery_method: models::DeliveryMethodEnum,
    #[serde(
        rename = "endpoint_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_url: Option<Option<String>>,
    #[serde(rename = "events_requested", skip_serializing_if = "Option::is_none")]
    pub events_requested: Option<Vec<models::EventsRequestedEnum>>,
    #[serde(rename = "format")]
    pub format: String,
    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    pub aud: Option<Vec<String>>,
    #[serde(rename = "iss")]
    pub iss: String,
}

impl SsfStream {
    /// SSFStream Serializer
    pub fn new(
        pk: uuid::Uuid,
        provider: i32,
        provider_obj: models::SsfProvider,
        delivery_method: models::DeliveryMethodEnum,
        format: String,
        iss: String,
    ) -> SsfStream {
        SsfStream {
            pk,
            provider,
            provider_obj,
            delivery_method,
            endpoint_url: None,
            events_requested: None,
            format,
            aud: None,
            iss,
        }
    }
}
