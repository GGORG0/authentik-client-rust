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

/// CurrentBrand : Partial brand information for styling
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentBrand {
    #[serde(rename = "matched_domain")]
    pub matched_domain: String,
    #[serde(rename = "branding_title")]
    pub branding_title: String,
    #[serde(rename = "branding_logo")]
    pub branding_logo: String,
    #[serde(rename = "branding_favicon")]
    pub branding_favicon: String,
    #[serde(rename = "ui_footer_links")]
    pub ui_footer_links: Vec<models::FooterLink>,
    #[serde(rename = "ui_theme")]
    pub ui_theme: models::UiThemeEnum,
    #[serde(rename = "flow_authentication", skip_serializing_if = "Option::is_none")]
    pub flow_authentication: Option<String>,
    #[serde(rename = "flow_invalidation", skip_serializing_if = "Option::is_none")]
    pub flow_invalidation: Option<String>,
    #[serde(rename = "flow_recovery", skip_serializing_if = "Option::is_none")]
    pub flow_recovery: Option<String>,
    #[serde(rename = "flow_unenrollment", skip_serializing_if = "Option::is_none")]
    pub flow_unenrollment: Option<String>,
    #[serde(rename = "flow_user_settings", skip_serializing_if = "Option::is_none")]
    pub flow_user_settings: Option<String>,
    #[serde(rename = "flow_device_code", skip_serializing_if = "Option::is_none")]
    pub flow_device_code: Option<String>,
    #[serde(rename = "default_locale")]
    pub default_locale: String,
}

impl CurrentBrand {
    /// Partial brand information for styling
    pub fn new(
        matched_domain: String,
        branding_title: String,
        branding_logo: String,
        branding_favicon: String,
        ui_footer_links: Vec<models::FooterLink>,
        ui_theme: models::UiThemeEnum,
        default_locale: String,
    ) -> CurrentBrand {
        CurrentBrand {
            matched_domain,
            branding_title,
            branding_logo,
            branding_favicon,
            ui_footer_links,
            ui_theme,
            flow_authentication: None,
            flow_invalidation: None,
            flow_recovery: None,
            flow_unenrollment: None,
            flow_user_settings: None,
            flow_device_code: None,
            default_locale,
        }
    }
}
