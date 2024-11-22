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

/// PatchedPlexSourceRequest : Plex Source Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedPlexSourceRequest {
    /// Source's display Name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Internal source name, used in URLs.
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Flow to use when authenticating existing users.
    #[serde(
        rename = "authentication_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_flow: Option<Option<uuid::Uuid>>,
    /// Flow to use when enrolling new users.
    #[serde(
        rename = "enrollment_flow",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enrollment_flow: Option<Option<uuid::Uuid>>,
    #[serde(rename = "user_property_mappings", skip_serializing_if = "Option::is_none")]
    pub user_property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "group_property_mappings", skip_serializing_if = "Option::is_none")]
    pub group_property_mappings: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "policy_engine_mode", skip_serializing_if = "Option::is_none")]
    pub policy_engine_mode: Option<models::PolicyEngineMode>,
    /// How the source determines if an existing user should be authenticated or a new user enrolled.
    #[serde(rename = "user_matching_mode", skip_serializing_if = "Option::is_none")]
    pub user_matching_mode: Option<models::UserMatchingModeEnum>,
    #[serde(rename = "user_path_template", skip_serializing_if = "Option::is_none")]
    pub user_path_template: Option<String>,
    /// How the source determines if an existing group should be used or a new group created.
    #[serde(rename = "group_matching_mode", skip_serializing_if = "Option::is_none")]
    pub group_matching_mode: Option<models::GroupMatchingModeEnum>,
    /// Client identifier used to talk to Plex.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Which servers a user has to be a member of to be granted access. Empty list allows every server.
    #[serde(rename = "allowed_servers", skip_serializing_if = "Option::is_none")]
    pub allowed_servers: Option<Vec<String>>,
    /// Allow friends to authenticate, even if you don't share a server.
    #[serde(rename = "allow_friends", skip_serializing_if = "Option::is_none")]
    pub allow_friends: Option<bool>,
    /// Plex token used to check friends
    #[serde(rename = "plex_token", skip_serializing_if = "Option::is_none")]
    pub plex_token: Option<String>,
}

impl PatchedPlexSourceRequest {
    /// Plex Source Serializer
    pub fn new() -> PatchedPlexSourceRequest {
        PatchedPlexSourceRequest {
            name: None,
            slug: None,
            enabled: None,
            authentication_flow: None,
            enrollment_flow: None,
            user_property_mappings: None,
            group_property_mappings: None,
            policy_engine_mode: None,
            user_matching_mode: None,
            user_path_template: None,
            group_matching_mode: None,
            client_id: None,
            allowed_servers: None,
            allow_friends: None,
            plex_token: None,
        }
    }
}
