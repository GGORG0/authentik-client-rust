/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PromptTypeEnum {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "text_area")]
    TextArea,
    #[serde(rename = "text_read_only")]
    TextReadOnly,
    #[serde(rename = "text_area_read_only")]
    TextAreaReadOnly,
    #[serde(rename = "username")]
    Username,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "radio-button-group")]
    RadioButtonGroup,
    #[serde(rename = "dropdown")]
    Dropdown,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "separator")]
    Separator,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "ak-locale")]
    AkLocale,
}

impl std::fmt::Display for PromptTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::TextArea => write!(f, "text_area"),
            Self::TextReadOnly => write!(f, "text_read_only"),
            Self::TextAreaReadOnly => write!(f, "text_area_read_only"),
            Self::Username => write!(f, "username"),
            Self::Email => write!(f, "email"),
            Self::Password => write!(f, "password"),
            Self::Number => write!(f, "number"),
            Self::Checkbox => write!(f, "checkbox"),
            Self::RadioButtonGroup => write!(f, "radio-button-group"),
            Self::Dropdown => write!(f, "dropdown"),
            Self::Date => write!(f, "date"),
            Self::DateTime => write!(f, "date-time"),
            Self::File => write!(f, "file"),
            Self::Separator => write!(f, "separator"),
            Self::Hidden => write!(f, "hidden"),
            Self::Static => write!(f, "static"),
            Self::AkLocale => write!(f, "ak-locale"),
        }
    }
}

impl Default for PromptTypeEnum {
    fn default() -> PromptTypeEnum {
        Self::Text
    }
}
