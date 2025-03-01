/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SyncObjectRequest : Sync object serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncObjectRequest {
    #[serde(rename = "sync_object_model")]
    pub sync_object_model: models::SyncObjectModelEnum,
    #[serde(rename = "sync_object_id")]
    pub sync_object_id: String,
    #[serde(rename = "override_dry_run", skip_serializing_if = "Option::is_none")]
    pub override_dry_run: Option<bool>,
}

impl SyncObjectRequest {
    /// Sync object serializer
    pub fn new(sync_object_model: models::SyncObjectModelEnum, sync_object_id: String) -> SyncObjectRequest {
        SyncObjectRequest {
            sync_object_model,
            sync_object_id,
            override_dry_run: None,
        }
    }
}
