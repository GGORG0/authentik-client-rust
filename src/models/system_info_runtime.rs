/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SystemInfoRuntime : Get versions
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemInfoRuntime {
    #[serde(rename = "python_version")]
    pub python_version: String,
    #[serde(rename = "environment")]
    pub environment: String,
    #[serde(rename = "architecture")]
    pub architecture: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "uname")]
    pub uname: String,
    #[serde(rename = "openssl_version")]
    pub openssl_version: String,
    #[serde(rename = "openssl_fips_enabled", deserialize_with = "Option::deserialize")]
    pub openssl_fips_enabled: Option<bool>,
    #[serde(rename = "authentik_version")]
    pub authentik_version: String,
}

impl SystemInfoRuntime {
    /// Get versions
    pub fn new(
        python_version: String,
        environment: String,
        architecture: String,
        platform: String,
        uname: String,
        openssl_version: String,
        openssl_fips_enabled: Option<bool>,
        authentik_version: String,
    ) -> SystemInfoRuntime {
        SystemInfoRuntime {
            python_version,
            environment,
            architecture,
            platform,
            uname,
            openssl_version,
            openssl_fips_enabled,
            authentik_version,
        }
    }
}
