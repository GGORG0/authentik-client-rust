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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NameIdPolicyEnum {
    #[serde(rename = "urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress")]
    Variant1Period1ColonNameidFormatColonEmailAddress,
    #[serde(rename = "urn:oasis:names:tc:SAML:2.0:nameid-format:persistent")]
    Variant2Period0ColonNameidFormatColonPersistent,
    #[serde(rename = "urn:oasis:names:tc:SAML:1.1:nameid-format:X509SubjectName")]
    Variant1Period1ColonNameidFormatColonX509SubjectName,
    #[serde(rename = "urn:oasis:names:tc:SAML:2.0:nameid-format:WindowsDomainQualifiedName")]
    Variant2Period0ColonNameidFormatColonWindowsDomainQualifiedName,
    #[serde(rename = "urn:oasis:names:tc:SAML:2.0:nameid-format:transient")]
    Variant2Period0ColonNameidFormatColonTransient,
}

impl std::fmt::Display for NameIdPolicyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant1Period1ColonNameidFormatColonEmailAddress => {
                write!(f, "urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress")
            }
            Self::Variant2Period0ColonNameidFormatColonPersistent => {
                write!(f, "urn:oasis:names:tc:SAML:2.0:nameid-format:persistent")
            }
            Self::Variant1Period1ColonNameidFormatColonX509SubjectName => {
                write!(f, "urn:oasis:names:tc:SAML:1.1:nameid-format:X509SubjectName")
            }
            Self::Variant2Period0ColonNameidFormatColonWindowsDomainQualifiedName => write!(
                f,
                "urn:oasis:names:tc:SAML:2.0:nameid-format:WindowsDomainQualifiedName"
            ),
            Self::Variant2Period0ColonNameidFormatColonTransient => {
                write!(f, "urn:oasis:names:tc:SAML:2.0:nameid-format:transient")
            }
        }
    }
}

impl Default for NameIdPolicyEnum {
    fn default() -> NameIdPolicyEnum {
        Self::Variant1Period1ColonNameidFormatColonEmailAddress
    }
}
