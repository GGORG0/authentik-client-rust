/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetailedCountryField {
    #[serde(rename = "code")]
    pub code: models::CountryCodeEnum,
    #[serde(rename = "name")]
    pub name: String,
}

impl DetailedCountryField {
    pub fn new(code: models::CountryCodeEnum, name: String) -> DetailedCountryField {
        DetailedCountryField { code, name }
    }
}
