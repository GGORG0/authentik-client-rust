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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlowLayoutEnum {
    #[serde(rename = "stacked")]
    Stacked,
    #[serde(rename = "content_left")]
    ContentLeft,
    #[serde(rename = "content_right")]
    ContentRight,
    #[serde(rename = "sidebar_left")]
    SidebarLeft,
    #[serde(rename = "sidebar_right")]
    SidebarRight,
}

impl std::fmt::Display for FlowLayoutEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Stacked => write!(f, "stacked"),
            Self::ContentLeft => write!(f, "content_left"),
            Self::ContentRight => write!(f, "content_right"),
            Self::SidebarLeft => write!(f, "sidebar_left"),
            Self::SidebarRight => write!(f, "sidebar_right"),
        }
    }
}

impl Default for FlowLayoutEnum {
    fn default() -> FlowLayoutEnum {
        Self::Stacked
    }
}
