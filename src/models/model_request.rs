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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "provider_model")]
pub enum ModelRequest {
    #[serde(rename = "authentik_providers_google_workspace.googleworkspaceprovider")]
    AuthentikProvidersGoogleWorkspacePeriodGoogleworkspaceprovider(models::GoogleWorkspaceProviderRequest),
    #[serde(rename = "authentik_providers_ldap.ldapprovider")]
    AuthentikProvidersLdapPeriodLdapprovider(models::LdapProviderRequest),
    #[serde(rename = "authentik_providers_microsoft_entra.microsoftentraprovider")]
    AuthentikProvidersMicrosoftEntraPeriodMicrosoftentraprovider(models::MicrosoftEntraProviderRequest),
    #[serde(rename = "authentik_providers_oauth2.oauth2provider")]
    AuthentikProvidersOauth2PeriodOauth2provider(models::OAuth2ProviderRequest),
    #[serde(rename = "authentik_providers_proxy.proxyprovider")]
    AuthentikProvidersProxyPeriodProxyprovider(models::ProxyProviderRequest),
    #[serde(rename = "authentik_providers_rac.racprovider")]
    AuthentikProvidersRacPeriodRacprovider(models::RacProviderRequest),
    #[serde(rename = "authentik_providers_radius.radiusprovider")]
    AuthentikProvidersRadiusPeriodRadiusprovider(models::RadiusProviderRequest),
    #[serde(rename = "authentik_providers_saml.samlprovider")]
    AuthentikProvidersSamlPeriodSamlprovider(models::SamlProviderRequest),
    #[serde(rename = "authentik_providers_scim.scimprovider")]
    AuthentikProvidersScimPeriodScimprovider(models::ScimProviderRequest),
}

impl Default for ModelRequest {
    fn default() -> Self {
        Self::AuthentikProvidersGoogleWorkspacePeriodGoogleworkspaceprovider(Default::default())
    }
}
