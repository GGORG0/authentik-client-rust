/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPath {
    #[serde(rename = "paths")]
    pub paths: Vec<String>,
}

impl UserPath {
    pub fn new(paths: Vec<String>) -> UserPath {
        UserPath { paths }
    }
}
