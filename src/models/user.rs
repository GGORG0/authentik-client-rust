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

/// User : User Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "pk")]
    pub pk: i32,
    #[serde(rename = "username")]
    pub username: String,
    /// User's display name.
    #[serde(rename = "name")]
    pub name: String,
    /// Designates whether this user should be treated as active. Unselect this instead of deleting accounts.
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(
        rename = "last_login",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_login: Option<Option<String>>,
    #[serde(rename = "is_superuser")]
    pub is_superuser: bool,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "groups_obj", deserialize_with = "Option::deserialize")]
    pub groups_obj: Option<Vec<models::UserGroup>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User's avatar, either a http/https URL or a data URI
    #[serde(rename = "avatar")]
    pub avatar: String,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "uid")]
    pub uid: String,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::UserTypeEnum>,
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl User {
    /// User Serializer
    pub fn new(
        pk: i32,
        username: String,
        name: String,
        is_superuser: bool,
        groups_obj: Option<Vec<models::UserGroup>>,
        avatar: String,
        uid: String,
        uuid: uuid::Uuid,
    ) -> User {
        User {
            pk,
            username,
            name,
            is_active: None,
            last_login: None,
            is_superuser,
            groups: None,
            groups_obj,
            email: None,
            avatar,
            attributes: None,
            uid,
            path: None,
            r#type: None,
            uuid,
        }
    }
}
