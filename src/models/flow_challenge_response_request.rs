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
#[serde(tag = "component")]
pub enum FlowChallengeResponseRequest {
    #[serde(rename = "ak-source-oauth-apple")]
    AkSourceOauthApple(models::AppleChallengeResponseRequest),
    #[serde(rename = "ak-stage-authenticator-duo")]
    AkStageAuthenticatorDuo(models::AuthenticatorDuoChallengeResponseRequest),
    #[serde(rename = "ak-stage-authenticator-sms")]
    AkStageAuthenticatorSms(models::AuthenticatorSmsChallengeResponseRequest),
    #[serde(rename = "ak-stage-authenticator-static")]
    AkStageAuthenticatorStatic(models::AuthenticatorStaticChallengeResponseRequest),
    #[serde(rename = "ak-stage-authenticator-totp")]
    AkStageAuthenticatorTotp(models::AuthenticatorTotpChallengeResponseRequest),
    #[serde(rename = "ak-stage-authenticator-validate")]
    AkStageAuthenticatorValidate(models::AuthenticatorValidationChallengeResponseRequest),
    #[serde(rename = "ak-stage-authenticator-webauthn")]
    AkStageAuthenticatorWebauthn(models::AuthenticatorWebAuthnChallengeResponseRequest),
    #[serde(rename = "ak-stage-autosubmit")]
    AkStageAutosubmit(models::AutoSubmitChallengeResponseRequest),
    #[serde(rename = "ak-stage-captcha")]
    AkStageCaptcha(models::CaptchaChallengeResponseRequest),
    #[serde(rename = "ak-stage-consent")]
    AkStageConsent(models::ConsentChallengeResponseRequest),
    #[serde(rename = "ak-stage-dummy")]
    AkStageDummy(models::DummyChallengeResponseRequest),
    #[serde(rename = "ak-stage-email")]
    AkStageEmail(models::EmailChallengeResponseRequest),
    #[serde(rename = "xak-flow-frame")]
    XakFlowFrame(models::FrameChallengeResponseRequest),
    #[serde(rename = "ak-stage-identification")]
    AkStageIdentification(models::IdentificationChallengeResponseRequest),
    #[serde(rename = "ak-provider-oauth2-device-code")]
    AkProviderOauth2DeviceCode(models::OAuthDeviceCodeChallengeResponseRequest),
    #[serde(rename = "ak-provider-oauth2-device-code-finish")]
    AkProviderOauth2DeviceCodeFinish(models::OAuthDeviceCodeFinishChallengeResponseRequest),
    #[serde(rename = "ak-stage-password")]
    AkStagePassword(models::PasswordChallengeResponseRequest),
    #[serde(rename = "ak-source-plex")]
    AkSourcePlex(models::PlexAuthenticationChallengeResponseRequest),
    #[serde(rename = "ak-stage-prompt")]
    AkStagePrompt(models::PromptChallengeResponseRequest),
    #[serde(rename = "ak-stage-user-login")]
    AkStageUserLogin(models::UserLoginChallengeResponseRequest),
}

impl Default for FlowChallengeResponseRequest {
    fn default() -> Self {
        Self::AkSourceOauthApple(Default::default())
    }
}
