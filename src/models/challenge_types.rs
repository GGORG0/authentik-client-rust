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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "component")]
pub enum ChallengeTypes {
    #[serde(rename = "ak-stage-access-denied")]
    AkStageAccessDenied(models::AccessDeniedChallenge),
    #[serde(rename = "ak-source-oauth-apple")]
    AkSourceOauthApple(models::AppleLoginChallenge),
    #[serde(rename = "ak-stage-authenticator-duo")]
    AkStageAuthenticatorDuo(models::AuthenticatorDuoChallenge),
    #[serde(rename = "ak-stage-authenticator-email")]
    AkStageAuthenticatorEmail(models::AuthenticatorEmailChallenge),
    #[serde(rename = "ak-stage-authenticator-sms")]
    AkStageAuthenticatorSms(models::AuthenticatorSmsChallenge),
    #[serde(rename = "ak-stage-authenticator-static")]
    AkStageAuthenticatorStatic(models::AuthenticatorStaticChallenge),
    #[serde(rename = "ak-stage-authenticator-totp")]
    AkStageAuthenticatorTotp(models::AuthenticatorTotpChallenge),
    #[serde(rename = "ak-stage-authenticator-validate")]
    AkStageAuthenticatorValidate(models::AuthenticatorValidationChallenge),
    #[serde(rename = "ak-stage-authenticator-webauthn")]
    AkStageAuthenticatorWebauthn(models::AuthenticatorWebAuthnChallenge),
    #[serde(rename = "ak-stage-autosubmit")]
    AkStageAutosubmit(models::AutosubmitChallenge),
    #[serde(rename = "ak-stage-captcha")]
    AkStageCaptcha(models::CaptchaChallenge),
    #[serde(rename = "ak-stage-consent")]
    AkStageConsent(models::ConsentChallenge),
    #[serde(rename = "ak-stage-dummy")]
    AkStageDummy(models::DummyChallenge),
    #[serde(rename = "ak-stage-email")]
    AkStageEmail(models::EmailChallenge),
    #[serde(rename = "ak-stage-flow-error")]
    AkStageFlowError(models::FlowErrorChallenge),
    #[serde(rename = "xak-flow-frame")]
    XakFlowFrame(models::FrameChallenge),
    #[serde(rename = "ak-stage-identification")]
    AkStageIdentification(models::IdentificationChallenge),
    #[serde(rename = "ak-provider-oauth2-device-code")]
    AkProviderOauth2DeviceCode(models::OAuthDeviceCodeChallenge),
    #[serde(rename = "ak-provider-oauth2-device-code-finish")]
    AkProviderOauth2DeviceCodeFinish(models::OAuthDeviceCodeFinishChallenge),
    #[serde(rename = "ak-stage-password")]
    AkStagePassword(models::PasswordChallenge),
    #[serde(rename = "ak-source-plex")]
    AkSourcePlex(models::PlexAuthenticationChallenge),
    #[serde(rename = "ak-stage-prompt")]
    AkStagePrompt(models::PromptChallenge),
    #[serde(rename = "xak-flow-redirect")]
    XakFlowRedirect(models::RedirectChallenge),
    #[serde(rename = "ak-stage-session-end")]
    AkStageSessionEnd(models::SessionEndChallenge),
    #[serde(rename = "xak-flow-shell")]
    XakFlowShell(models::ShellChallenge),
    #[serde(rename = "ak-stage-user-login")]
    AkStageUserLogin(models::UserLoginChallenge),
}

impl Default for ChallengeTypes {
    fn default() -> Self {
        Self::AkStageAccessDenied(Default::default())
    }
}
