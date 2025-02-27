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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedNotificationRuleList {
    #[serde(rename = "pagination")]
    pub pagination: models::Pagination,
    #[serde(rename = "results")]
    pub results: Vec<models::NotificationRule>,
}

impl PaginatedNotificationRuleList {
    pub fn new(
        pagination: models::Pagination,
        results: Vec<models::NotificationRule>,
    ) -> PaginatedNotificationRuleList {
        PaginatedNotificationRuleList { pagination, results }
    }
}
