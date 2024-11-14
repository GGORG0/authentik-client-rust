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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstallId {
    #[serde(rename = "install_id")]
    pub install_id: String,
}

impl InstallId {
    pub fn new(install_id: String) -> InstallId {
        InstallId { install_id }
    }
}
