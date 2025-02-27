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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderModelEnum {
    #[serde(rename = "authentik_providers_google_workspace.googleworkspaceprovider")]
    GoogleWorkspacePeriodGoogleworkspaceprovider,
    #[serde(rename = "authentik_providers_ldap.ldapprovider")]
    LdapPeriodLdapprovider,
    #[serde(rename = "authentik_providers_microsoft_entra.microsoftentraprovider")]
    MicrosoftEntraPeriodMicrosoftentraprovider,
    #[serde(rename = "authentik_providers_oauth2.oauth2provider")]
    Oauth2PeriodOauth2provider,
    #[serde(rename = "authentik_providers_proxy.proxyprovider")]
    ProxyPeriodProxyprovider,
    #[serde(rename = "authentik_providers_rac.racprovider")]
    RacPeriodRacprovider,
    #[serde(rename = "authentik_providers_radius.radiusprovider")]
    RadiusPeriodRadiusprovider,
    #[serde(rename = "authentik_providers_saml.samlprovider")]
    SamlPeriodSamlprovider,
    #[serde(rename = "authentik_providers_scim.scimprovider")]
    ScimPeriodScimprovider,
    #[serde(rename = "authentik_providers_ssf.ssfprovider")]
    SsfPeriodSsfprovider,
}

impl std::fmt::Display for ProviderModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GoogleWorkspacePeriodGoogleworkspaceprovider => {
                write!(f, "authentik_providers_google_workspace.googleworkspaceprovider")
            }
            Self::LdapPeriodLdapprovider => write!(f, "authentik_providers_ldap.ldapprovider"),
            Self::MicrosoftEntraPeriodMicrosoftentraprovider => {
                write!(f, "authentik_providers_microsoft_entra.microsoftentraprovider")
            }
            Self::Oauth2PeriodOauth2provider => write!(f, "authentik_providers_oauth2.oauth2provider"),
            Self::ProxyPeriodProxyprovider => write!(f, "authentik_providers_proxy.proxyprovider"),
            Self::RacPeriodRacprovider => write!(f, "authentik_providers_rac.racprovider"),
            Self::RadiusPeriodRadiusprovider => write!(f, "authentik_providers_radius.radiusprovider"),
            Self::SamlPeriodSamlprovider => write!(f, "authentik_providers_saml.samlprovider"),
            Self::ScimPeriodScimprovider => write!(f, "authentik_providers_scim.scimprovider"),
            Self::SsfPeriodSsfprovider => write!(f, "authentik_providers_ssf.ssfprovider"),
        }
    }
}

impl Default for ProviderModelEnum {
    fn default() -> ProviderModelEnum {
        Self::GoogleWorkspacePeriodGoogleworkspaceprovider
    }
}
