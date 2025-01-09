/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.12.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CertificateKeyPair : CertificateKeyPair Serializer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateKeyPair {
    #[serde(rename = "pk")]
    pub pk: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    /// Get certificate Hash (SHA256)
    #[serde(rename = "fingerprint_sha256", deserialize_with = "Option::deserialize")]
    pub fingerprint_sha256: Option<String>,
    /// Get certificate Hash (SHA1)
    #[serde(rename = "fingerprint_sha1", deserialize_with = "Option::deserialize")]
    pub fingerprint_sha1: Option<String>,
    /// Get certificate expiry
    #[serde(rename = "cert_expiry", deserialize_with = "Option::deserialize")]
    pub cert_expiry: Option<String>,
    /// Get certificate subject as full rfc4514
    #[serde(rename = "cert_subject", deserialize_with = "Option::deserialize")]
    pub cert_subject: Option<String>,
    /// Show if this keypair has a private key configured or not
    #[serde(rename = "private_key_available")]
    pub private_key_available: bool,
    /// Get the private key's type, if set
    #[serde(rename = "private_key_type", deserialize_with = "Option::deserialize")]
    pub private_key_type: Option<String>,
    /// Get URL to download certificate
    #[serde(rename = "certificate_download_url")]
    pub certificate_download_url: String,
    /// Get URL to download private key
    #[serde(rename = "private_key_download_url")]
    pub private_key_download_url: String,
    /// Objects that are managed by authentik. These objects are created and updated automatically. This flag only indicates that an object can be overwritten by migrations. You can still modify the objects via the API, but expect changes to be overwritten in a later update.
    #[serde(rename = "managed", deserialize_with = "Option::deserialize")]
    pub managed: Option<String>,
}

impl CertificateKeyPair {
    /// CertificateKeyPair Serializer
    pub fn new(
        pk: uuid::Uuid,
        name: String,
        fingerprint_sha256: Option<String>,
        fingerprint_sha1: Option<String>,
        cert_expiry: Option<String>,
        cert_subject: Option<String>,
        private_key_available: bool,
        private_key_type: Option<String>,
        certificate_download_url: String,
        private_key_download_url: String,
        managed: Option<String>,
    ) -> CertificateKeyPair {
        CertificateKeyPair {
            pk,
            name,
            fingerprint_sha256,
            fingerprint_sha1,
            cert_expiry,
            cert_subject,
            private_key_available,
            private_key_type,
            certificate_download_url,
            private_key_download_url,
            managed,
        }
    }
}
