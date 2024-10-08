// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageCustomText {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pre_header: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub greeting: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub footer_text: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub is_default: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginCustomText {
    #[prost(message, optional, tag="1")]
    pub details: ::core::option::Option<super::super::v1::ObjectDetails>,
    #[prost(message, optional, tag="2")]
    pub select_account_text: ::core::option::Option<SelectAccountScreenText>,
    #[prost(message, optional, tag="3")]
    pub login_text: ::core::option::Option<LoginScreenText>,
    #[prost(message, optional, tag="4")]
    pub password_text: ::core::option::Option<PasswordScreenText>,
    #[prost(message, optional, tag="5")]
    pub username_change_text: ::core::option::Option<UsernameChangeScreenText>,
    #[prost(message, optional, tag="6")]
    pub username_change_done_text: ::core::option::Option<UsernameChangeDoneScreenText>,
    #[prost(message, optional, tag="7")]
    pub init_password_text: ::core::option::Option<InitPasswordScreenText>,
    #[prost(message, optional, tag="8")]
    pub init_password_done_text: ::core::option::Option<InitPasswordDoneScreenText>,
    #[prost(message, optional, tag="9")]
    pub email_verification_text: ::core::option::Option<EmailVerificationScreenText>,
    #[prost(message, optional, tag="10")]
    pub email_verification_done_text: ::core::option::Option<EmailVerificationDoneScreenText>,
    #[prost(message, optional, tag="11")]
    pub initialize_user_text: ::core::option::Option<InitializeUserScreenText>,
    #[prost(message, optional, tag="12")]
    pub initialize_done_text: ::core::option::Option<InitializeUserDoneScreenText>,
    #[prost(message, optional, tag="13")]
    pub init_mfa_prompt_text: ::core::option::Option<InitMfaPromptScreenText>,
    #[prost(message, optional, tag="14")]
    pub init_mfa_otp_text: ::core::option::Option<InitMfaotpScreenText>,
    #[prost(message, optional, tag="15")]
    pub init_mfa_u2f_text: ::core::option::Option<InitMfau2fScreenText>,
    #[prost(message, optional, tag="16")]
    pub init_mfa_done_text: ::core::option::Option<InitMfaDoneScreenText>,
    #[prost(message, optional, tag="17")]
    pub mfa_providers_text: ::core::option::Option<MfaProvidersText>,
    #[prost(message, optional, tag="18")]
    pub verify_mfa_otp_text: ::core::option::Option<VerifyMfaotpScreenText>,
    #[prost(message, optional, tag="19")]
    pub verify_mfa_u2f_text: ::core::option::Option<VerifyMfau2fScreenText>,
    #[prost(message, optional, tag="20")]
    pub passwordless_text: ::core::option::Option<PasswordlessScreenText>,
    #[prost(message, optional, tag="21")]
    pub password_change_text: ::core::option::Option<PasswordChangeScreenText>,
    #[prost(message, optional, tag="22")]
    pub password_change_done_text: ::core::option::Option<PasswordChangeDoneScreenText>,
    #[prost(message, optional, tag="23")]
    pub password_reset_done_text: ::core::option::Option<PasswordResetDoneScreenText>,
    #[prost(message, optional, tag="24")]
    pub registration_option_text: ::core::option::Option<RegistrationOptionScreenText>,
    #[prost(message, optional, tag="25")]
    pub registration_user_text: ::core::option::Option<RegistrationUserScreenText>,
    #[prost(message, optional, tag="26")]
    pub registration_org_text: ::core::option::Option<RegistrationOrgScreenText>,
    #[prost(message, optional, tag="27")]
    pub linking_user_done_text: ::core::option::Option<LinkingUserDoneScreenText>,
    #[prost(message, optional, tag="28")]
    pub external_user_not_found_text: ::core::option::Option<ExternalUserNotFoundScreenText>,
    #[prost(message, optional, tag="29")]
    pub success_login_text: ::core::option::Option<SuccessLoginScreenText>,
    #[prost(message, optional, tag="30")]
    pub logout_text: ::core::option::Option<LogoutDoneScreenText>,
    #[prost(message, optional, tag="31")]
    pub footer_text: ::core::option::Option<FooterText>,
    #[prost(message, optional, tag="32")]
    pub passwordless_prompt_text: ::core::option::Option<PasswordlessPromptScreenText>,
    #[prost(message, optional, tag="33")]
    pub passwordless_registration_text: ::core::option::Option<PasswordlessRegistrationScreenText>,
    #[prost(message, optional, tag="34")]
    pub passwordless_registration_done_text: ::core::option::Option<PasswordlessRegistrationDoneScreenText>,
    #[prost(message, optional, tag="35")]
    pub external_registration_user_overview_text: ::core::option::Option<ExternalRegistrationUserOverviewScreenText>,
    #[prost(bool, tag="36")]
    pub is_default: bool,
    #[prost(message, optional, tag="37")]
    pub linking_user_prompt_text: ::core::option::Option<LinkingUserPromptScreenText>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectAccountScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub title_linking_process: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description_linking_process: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub other_user: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub session_state_active: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub session_state_inactive: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub user_must_be_member_of_org: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub title_linking_process: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description_linking_process: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user_must_be_member_of_org: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub login_name_label: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub register_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub external_user_description: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub user_name_placeholder: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub login_name_placeholder: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub reset_link_text: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub back_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub min_length: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub has_uppercase: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub has_lowercase: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub has_number: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub has_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub confirmation: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsernameChangeScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub username_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub cancel_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsernameChangeDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitPasswordScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub code_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_password_label: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub new_password_confirm_label: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub resend_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitPasswordDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub cancel_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmailVerificationScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub code_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub resend_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmailVerificationDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub cancel_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub login_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeUserScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub code_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_password_label: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub new_password_confirm_label: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub resend_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeUserDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub cancel_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitMfaPromptScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub otp_option: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub u2f_option: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub skip_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitMfaotpScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description_otp: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub secret_label: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub code_label: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub cancel_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitMfau2fScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub token_name_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub not_supported: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub register_token_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub error_retry: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitMfaDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub cancel_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MfaProvidersText {
    #[prost(string, tag="1")]
    pub choose_other: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub otp: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub u2f: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMfaotpScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub code_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMfau2fScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub validate_token_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub not_supported: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub error_retry: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordlessScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub login_with_pw_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub validate_token_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub not_supported: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub error_retry: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordChangeScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub old_password_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_password_label: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub new_password_confirm_label: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub cancel_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub expired_description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordChangeDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordResetDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationOptionScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user_name_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub external_login_description: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub login_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationUserScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description_org_register: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub firstname_label: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub lastname_label: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub email_label: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub username_label: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub language_label: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub gender_label: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub password_label: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub password_confirm_label: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub tos_and_privacy_label: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub tos_confirm: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub tos_link_text: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub privacy_confirm: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub privacy_link_text: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub back_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalRegistrationUserOverviewScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub email_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub username_label: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub firstname_label: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub lastname_label: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub nickname_label: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub language_label: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub phone_label: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub tos_and_privacy_label: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub tos_confirm: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub tos_link_text: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub privacy_link_text: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub back_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub privacy_confirm: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationOrgScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub orgname_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub firstname_label: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub lastname_label: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub username_label: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub email_label: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub password_label: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub password_confirm_label: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub tos_and_privacy_label: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub tos_confirm: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub tos_link_text: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub privacy_confirm: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub privacy_link_text: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub save_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkingUserPromptScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub link_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub other_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkingUserDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub cancel_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalUserNotFoundScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub link_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub auto_register_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub tos_and_privacy_label: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub tos_confirm: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub tos_link_text: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub privacy_link_text: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub privacy_confirm: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessLoginScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// Text to describe that auto-redirect should happen after successful login
    #[prost(string, tag="2")]
    pub auto_redirect_description: ::prost::alloc::string::String,
    /// Text to describe that the window can be closed after redirect
    #[prost(string, tag="3")]
    pub redirected_description: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub next_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogoutDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub login_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FooterText {
    #[prost(string, tag="1")]
    pub tos: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub privacy_policy: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub help: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub support_email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordlessPromptScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description_init: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub passwordless_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub skip_button_text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordlessRegistrationScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub token_name_label: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub not_supported: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub register_token_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub error_retry: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordlessRegistrationDoneScreenText {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub next_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub cancel_button_text: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub description_close: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
