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

/// FooterLink : Links returned in Config API
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FooterLink {
    #[serde(rename = "href", deserialize_with = "Option::deserialize")]
    pub href: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
}

impl FooterLink {
    /// Links returned in Config API
    pub fn new(href: Option<String>, name: String) -> FooterLink {
        FooterLink { href, name }
    }
}
