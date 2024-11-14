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

/// App : Serialize Application info
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "label")]
    pub label: String,
}

impl App {
    /// Serialize Application info
    pub fn new(name: String, label: String) -> App {
        App { name, label }
    }
}
