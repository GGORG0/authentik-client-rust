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

/// GroupMemberRequest : Stripped down user serializer to show relevant users for groups
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMemberRequest {
    /// Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only.
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
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl GroupMemberRequest {
    /// Stripped down user serializer to show relevant users for groups
    pub fn new(username: String, name: String) -> GroupMemberRequest {
        GroupMemberRequest {
            username,
            name,
            is_active: None,
            last_login: None,
            email: None,
            attributes: None,
        }
    }
}
