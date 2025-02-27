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

/// EndpointDevice : Serializer for Endpoint authenticator devices
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointDevice {
    #[serde(rename = "pk", skip_serializing_if = "Option::is_none")]
    pub pk: Option<uuid::Uuid>,
    /// The human-readable name of this device.
    #[serde(rename = "name")]
    pub name: String,
}

impl EndpointDevice {
    /// Serializer for Endpoint authenticator devices
    pub fn new(name: String) -> EndpointDevice {
        EndpointDevice { pk: None, name }
    }
}
