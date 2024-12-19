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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkBindingEnum {
    #[serde(rename = "no_binding")]
    NoBinding,
    #[serde(rename = "bind_asn")]
    BindAsn,
    #[serde(rename = "bind_asn_network")]
    BindAsnNetwork,
    #[serde(rename = "bind_asn_network_ip")]
    BindAsnNetworkIp,
}

impl std::fmt::Display for NetworkBindingEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoBinding => write!(f, "no_binding"),
            Self::BindAsn => write!(f, "bind_asn"),
            Self::BindAsnNetwork => write!(f, "bind_asn_network"),
            Self::BindAsnNetworkIp => write!(f, "bind_asn_network_ip"),
        }
    }
}

impl Default for NetworkBindingEnum {
    fn default() -> NetworkBindingEnum {
        Self::NoBinding
    }
}
