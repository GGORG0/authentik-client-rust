/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2025.2.0
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CertificateKeyPairRequest : CertificateKeyPair Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateKeyPairRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// PEM-encoded Certificate data
    #[serde(rename = "certificate_data")]
    pub certificate_data: String,
    /// Optional Private Key. If this is set, you can use this keypair for encryption.
    #[serde(rename = "key_data", skip_serializing_if = "Option::is_none")]
    pub key_data: Option<String>,
}

impl CertificateKeyPairRequest {
    /// CertificateKeyPair Serializer
    pub fn new(name: String, certificate_data: String) -> CertificateKeyPairRequest {
        CertificateKeyPairRequest {
            name,
            certificate_data,
            key_data: None,
        }
    }
}
