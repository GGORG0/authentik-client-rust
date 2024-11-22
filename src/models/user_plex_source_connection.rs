/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.4
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserPlexSourceConnection : Plex Source connection Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPlexSourceConnection {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "source")]
    pub source: models::Source,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl UserPlexSourceConnection {
    /// Plex Source connection Serializer
    pub fn new(
        pk: i32,
        user: i32,
        source: models::Source,
        created: String,
        identifier: String,
    ) -> UserPlexSourceConnection {
        UserPlexSourceConnection {
            pk,
            user,
            source,
            created,
            identifier,
        }
    }
}
