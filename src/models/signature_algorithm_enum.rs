/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.10.2
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignatureAlgorithmEnum {
    #[serde(rename = "http://www.w3.org/2000/09/xmldsig#rsa-sha1")]
    Variant2000Slash09SlashXmldsigHashRsaSha1,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256")]
    Variant2001Slash04SlashXmldsigMoreHashRsaSha256,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha384")]
    Variant2001Slash04SlashXmldsigMoreHashRsaSha384,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha512")]
    Variant2001Slash04SlashXmldsigMoreHashRsaSha512,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha1")]
    Variant2001Slash04SlashXmldsigMoreHashEcdsaSha1,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256")]
    Variant2001Slash04SlashXmldsigMoreHashEcdsaSha256,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha384")]
    Variant2001Slash04SlashXmldsigMoreHashEcdsaSha384,
    #[serde(rename = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512")]
    Variant2001Slash04SlashXmldsigMoreHashEcdsaSha512,
    #[serde(rename = "http://www.w3.org/2000/09/xmldsig#dsa-sha1")]
    Variant2000Slash09SlashXmldsigHashDsaSha1,
}

impl std::fmt::Display for SignatureAlgorithmEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant2000Slash09SlashXmldsigHashRsaSha1 => write!(f, "http://www.w3.org/2000/09/xmldsig#rsa-sha1"),
            Self::Variant2001Slash04SlashXmldsigMoreHashRsaSha256 => {
                write!(f, "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256")
            }
            Self::Variant2001Slash04SlashXmldsigMoreHashRsaSha384 => {
                write!(f, "http://www.w3.org/2001/04/xmldsig-more#rsa-sha384")
            }
            Self::Variant2001Slash04SlashXmldsigMoreHashRsaSha512 => {
                write!(f, "http://www.w3.org/2001/04/xmldsig-more#rsa-sha512")
            }
            Self::Variant2001Slash04SlashXmldsigMoreHashEcdsaSha1 => {
                write!(f, "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha1")
            }
            Self::Variant2001Slash04SlashXmldsigMoreHashEcdsaSha256 => {
                write!(f, "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256")
            }
            Self::Variant2001Slash04SlashXmldsigMoreHashEcdsaSha384 => {
                write!(f, "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha384")
            }
            Self::Variant2001Slash04SlashXmldsigMoreHashEcdsaSha512 => {
                write!(f, "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512")
            }
            Self::Variant2000Slash09SlashXmldsigHashDsaSha1 => write!(f, "http://www.w3.org/2000/09/xmldsig#dsa-sha1"),
        }
    }
}

impl Default for SignatureAlgorithmEnum {
    fn default() -> SignatureAlgorithmEnum {
        Self::Variant2000Slash09SlashXmldsigHashRsaSha1
    }
}
