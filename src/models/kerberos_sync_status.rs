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

/// KerberosSyncStatus : Kerberos Source sync status
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KerberosSyncStatus {
    #[serde(rename = "is_running")]
    pub is_running: bool,
    #[serde(rename = "tasks")]
    pub tasks: Vec<models::SystemTask>,
}

impl KerberosSyncStatus {
    /// Kerberos Source sync status
    pub fn new(is_running: bool, tasks: Vec<models::SystemTask>) -> KerberosSyncStatus {
        KerberosSyncStatus { is_running, tasks }
    }
}
