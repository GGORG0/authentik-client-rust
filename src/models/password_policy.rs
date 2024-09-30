/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.8.3
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PasswordPolicy : Password Policy Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordPolicy {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// When this option is enabled, all executions of this policy will be logged. By default, only execution errors are logged.
    #[serde(rename = "execution_logging", skip_serializing_if = "Option::is_none")]
    pub execution_logging: Option<bool>,
    /// Get object component so that we know how to edit the object
    #[serde(rename = "component")]
    pub component: String,
    /// Return object's verbose_name
    #[serde(rename = "verbose_name")]
    pub verbose_name: String,
    /// Return object's plural verbose_name
    #[serde(rename = "verbose_name_plural")]
    pub verbose_name_plural: String,
    /// Return internal model name
    #[serde(rename = "meta_model_name")]
    pub meta_model_name: String,
    /// Return objects policy is bound to
    #[serde(rename = "bound_to")]
    pub bound_to: i32,
    /// Field key to check, field keys defined in Prompt stages are available.
    #[serde(rename = "password_field", skip_serializing_if = "Option::is_none")]
    pub password_field: Option<String>,
    #[serde(rename = "amount_digits", skip_serializing_if = "Option::is_none")]
    pub amount_digits: Option<u32>,
    #[serde(rename = "amount_uppercase", skip_serializing_if = "Option::is_none")]
    pub amount_uppercase: Option<u32>,
    #[serde(rename = "amount_lowercase", skip_serializing_if = "Option::is_none")]
    pub amount_lowercase: Option<u32>,
    #[serde(rename = "amount_symbols", skip_serializing_if = "Option::is_none")]
    pub amount_symbols: Option<u32>,
    #[serde(rename = "length_min", skip_serializing_if = "Option::is_none")]
    pub length_min: Option<u32>,
    #[serde(rename = "symbol_charset", skip_serializing_if = "Option::is_none")]
    pub symbol_charset: Option<String>,
    #[serde(rename = "error_message", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "check_static_rules", skip_serializing_if = "Option::is_none")]
    pub check_static_rules: Option<bool>,
    #[serde(rename = "check_have_i_been_pwned", skip_serializing_if = "Option::is_none")]
    pub check_have_i_been_pwned: Option<bool>,
    #[serde(rename = "check_zxcvbn", skip_serializing_if = "Option::is_none")]
    pub check_zxcvbn: Option<bool>,
    /// How many times the password hash is allowed to be on haveibeenpwned
    #[serde(rename = "hibp_allowed_count", skip_serializing_if = "Option::is_none")]
    pub hibp_allowed_count: Option<u32>,
    /// If the zxcvbn score is equal or less than this value, the policy will fail.
    #[serde(rename = "zxcvbn_score_threshold", skip_serializing_if = "Option::is_none")]
    pub zxcvbn_score_threshold: Option<u32>,
}

impl PasswordPolicy {
    /// Password Policy Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        component: String,
        verbose_name: String,
        verbose_name_plural: String,
        meta_model_name: String,
        bound_to: i32,
    ) -> PasswordPolicy {
        PasswordPolicy {
            pk,
            name,
            execution_logging: None,
            component,
            verbose_name,
            verbose_name_plural,
            meta_model_name,
            bound_to,
            password_field: None,
            amount_digits: None,
            amount_uppercase: None,
            amount_lowercase: None,
            amount_symbols: None,
            length_min: None,
            symbol_charset: None,
            error_message: None,
            check_static_rules: None,
            check_have_i_been_pwned: None,
            check_zxcvbn: None,
            hibp_allowed_count: None,
            zxcvbn_score_threshold: None,
        }
    }
}
