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
pub struct PaginatedUserLogoutStageList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::UserLogoutStage>,
}

impl PaginatedUserLogoutStageList {
    pub fn new(pagination: models::Pagination, results: Vec<models::UserLogoutStage>) -> PaginatedUserLogoutStageList {
        PaginatedUserLogoutStageList { pagination, results }
    }
}
