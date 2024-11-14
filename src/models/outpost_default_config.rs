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

/// OutpostDefaultConfig : Global default outpost config
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutpostDefaultConfig {
    #[serde(rename = "config")]
    pub config: std::collections::HashMap<String, serde_json::Value>,
}

impl OutpostDefaultConfig {
    /// Global default outpost config
    pub fn new(config: std::collections::HashMap<String, serde_json::Value>) -> OutpostDefaultConfig {
        OutpostDefaultConfig { config }
    }
}
