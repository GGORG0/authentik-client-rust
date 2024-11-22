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

/// RedirectUriRequest : A single allowed redirect URI entry
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedirectUriRequest {
    #[serde(rename = "matching_mode")]
    pub matching_mode: models::MatchingModeEnum,
    #[serde(rename = "url")]
    pub url: String,
}

impl RedirectUriRequest {
    /// A single allowed redirect URI entry
    pub fn new(matching_mode: models::MatchingModeEnum, url: String) -> RedirectUriRequest {
        RedirectUriRequest { matching_mode, url }
    }
}
