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

/// SessionUser : Response for the /user/me endpoint, returns the currently active user (as `user` property) and, if this user is being impersonated, the original user in the `original` property.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionUser {
    #[serde(rename = "user")]
    pub user: models::UserSelf,
    #[serde(rename = "original", skip_serializing_if = "Option::is_none")]
    pub original: Option<models::UserSelf>,
}

impl SessionUser {
    /// Response for the /user/me endpoint, returns the currently active user (as `user` property) and, if this user is being impersonated, the original user in the `original` property.
    pub fn new(user: models::UserSelf) -> SessionUser {
        SessionUser { user, original: None }
    }
}
