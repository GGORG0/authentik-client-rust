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

/// LoginMetrics : Login Metrics per 1h
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginMetrics {
    #[serde(rename = "logins")]
    pub logins: Vec<models::Coordinate>,
    #[serde(rename = "logins_failed")]
    pub logins_failed: Vec<models::Coordinate>,
    #[serde(rename = "authorizations")]
    pub authorizations: Vec<models::Coordinate>,
}

impl LoginMetrics {
    /// Login Metrics per 1h
    pub fn new(
        logins: Vec<models::Coordinate>,
        logins_failed: Vec<models::Coordinate>,
        authorizations: Vec<models::Coordinate>,
    ) -> LoginMetrics {
        LoginMetrics {
            logins,
            logins_failed,
            authorizations,
        }
    }
}
