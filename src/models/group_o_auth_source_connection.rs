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

/// GroupOAuthSourceConnection : OAuth Group-Source connection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupOAuthSourceConnection {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "group")]
    pub group: uuid::Uuid,
    #[serde(rename = "source")]
    pub source: models::Source,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "created")]
    pub created: String,
}

impl GroupOAuthSourceConnection {
    /// OAuth Group-Source connection Serializer
    pub fn new(
        pk: i32,
        group: uuid::Uuid,
        source: models::Source,
        identifier: String,
        created: String,
    ) -> GroupOAuthSourceConnection {
        GroupOAuthSourceConnection {
            pk,
            group,
            source,
            identifier,
            created,
        }
    }
}
