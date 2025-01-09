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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintFile {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "last_m")]
    pub last_m: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "meta")]
    pub meta: models::Metadata,
}

impl BlueprintFile {
    pub fn new(path: String, last_m: String, hash: String, meta: models::Metadata) -> BlueprintFile {
        BlueprintFile {
            path,
            last_m,
            hash,
            meta,
        }
    }
}
