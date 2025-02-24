/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserSourceConnection : User source connection
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSourceConnection {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "source")]
    pub source: uuid::Uuid,
    #[serde(rename = "source_obj")]
    pub source_obj: models::Source,
    #[serde(rename = "created")]
    pub created: String,
}

impl UserSourceConnection {
    /// User source connection
    pub fn new(
        pk: i32,
        user: i32,
        source: uuid::Uuid,
        source_obj: models::Source,
        created: String,
    ) -> UserSourceConnection {
        UserSourceConnection {
            pk,
            user,
            source,
            source_obj,
            created,
        }
    }
}
