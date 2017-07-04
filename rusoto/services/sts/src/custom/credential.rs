use chrono::*;
use rusoto_core::{self, AwsCredentials, CredentialsError, ProvideAwsCredentials};

use {Sts, GetSessionTokenRequest, AssumeRoleRequest, AssumeRoleWithWebIdentityRequest};

pub const DEFAULT_DURATION_SECONDS: i32 = 3600;
pub const DEFAULT_ROLE_DURATION_SECONDS: i32 = 900;

impl Into<AwsCredentials> for ::Credentials {
    fn into(self) -> AwsCredentials {
        let expires_at = sts_creds
            .expiration
            .parse::<DateTime<UTC>>()
            .expect("STS did not return a valid UTC DateTime in its credentials");

        Ok(AwsCredentials::new(sts_creds.access_key_id,
                               sts_creds.secret_access_key,
                               Some(sts_creds.session_token),
                               expires_at))
    }
}

/// [AwsCredentials](../struct.AwsCredentials.html) provider that calls
/// `GetSessionToken` using the provided [Sts](trait.Sts.html) client.
/// To use with MFA, pass in the MFA serial number then set the MFA code.
/// You will need to ensure the provider has a valid code each time you
/// acquire a new STS token.
pub struct StsSessionCredentialsProvider<T: Sts> {
    sts_client: T,
    session_duration: Duration,
    mfa_serial: Option<String>,
    mfa_code: Option<String>,
}

impl<T: Sts> StsSessionCredentialsProvider<T> {
    /// Creates a new `StsSessionCredentialsProvider` with the given
    /// [Sts](trait.Sts.html) client and session parameters.
    ///
    /// * `sts_client` - The [Sts](trait.Sts.html) client to use to acquire session tokens.
    /// * `duration` - The duration of the session tokens. Default 1 hour.
    /// * `mfa_serial` - Optional MFA hardware device serial number or virtual device ARN. Set the MFA code with `set_mfa_code`.
    pub fn new(sts_client: T, duration: Option<Duration>, mfa_serial: Option<String>) -> Self {
        StsSessionCredentialsProvider {
            sts_client: sts_client,
            session_duration: duration.unwrap_or(Duration::seconds(DEFAULT_DURATION_SECONDS as
                                                                   i64)),
            mfa_serial: mfa_serial,
            mfa_code: None,
        }
    }

    /// Set the MFA code for use when acquiring session tokens.
    pub fn set_mfa_code<S>(&mut self, code: S)
        where S: Into<String>
    {
        self.mfa_code = Some(code.into());
    }

    /// Clear the MFA code.
    pub fn clear_mfa_code(&mut self) {
        self.mfa_code = None;
    }

    /// Calls `GetSessionToken` to get a session token from the STS Api.
    /// Optionally uses MFA if the MFA serial number and code are set.
    pub fn get_session_token(&self) -> Result<AwsCredentials, CredentialsError> {
        match self.sts_client
                  .get_session_token(&GetSessionTokenRequest {
                                          serial_number: self.mfa_serial.clone(),
                                          token_code: self.mfa_code.clone(),
                                          duration_seconds: Some(self.session_duration
                                                                     .num_seconds() as
                                                                 i64),
                                          ..Default::default()
                                      }) {
            Ok(resp) => {
                resp.credentials
                    .ok_or(CredentialsError::new("no credentials in response"))?
                    .into()
            }
            err => {
                Err(CredentialsError::new(format!("StsProvider get_session_token error: {:?}",
                                                  err)))
            }
        }
    }
}

impl<T: Sts> ProvideAwsCredentials for StsSessionCredentialsProvider<T> {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        self.get_session_token()
    }
}

/// [AwsCredentials](../struct.AwsCredentials.html) provider that calls
/// `AssumeRole` using the provided [Sts](trait.Sts.html).
/// To use with MFA, pass in the MFA serial number then set the MFA code.
/// You will need to ensure the provider has a valid code each time you
/// acquire a new STS token.
pub struct StsAssumeRoleSessionCredentialsProvider<T: Sts> {
    sts_client: T,
    role_arn: String,
    session_name: String,
    external_id: Option<String>,
    session_duration: Duration,
    scope_down_policy: Option<String>,
    mfa_serial: Option<String>,
    mfa_code: Option<String>,
}

impl<T: Sts> StsAssumeRoleSessionCredentialsProvider<T> {
    /// Creates a new `StsAssumeRoleSessionCredentialsProvider` with the given
    /// [Sts](trait.Sts.html) client and session parameters.
    ///
    /// * `sts_client` - [Sts](trait.Sts.html) client to use to acquire session tokens.
    /// * `role_arn` - The ARN of the role to assume.
    /// * `session_name` - An identifier for the assumed role session. Minimum length of 2. Maximum length of 64. Pattern: `[\w+=,.@-]*`
    /// * `external_id` -
    /// * `session_duration` - Duration of session tokens. Default 1 hour.
    /// * `scope_down_policy` - Optional inline IAM policy in JSON format to further restrict the access granted to the negotiated session.
    /// * `mfa_serial` - Optional MFA hardware device serial number or virtual device ARN. Use `set_mfa_code` to set the MFA code.
    pub fn new(sts_client: T,
               role_arn: String,
               session_name: String,
               external_id: Option<String>,
               session_duration: Option<Duration>,
               scope_down_policy: Option<String>,
               mfa_serial: Option<String>)
               -> Self {
        StsAssumeRoleSessionCredentialsProvider {
            sts_client: sts_client,
            role_arn: role_arn,
            session_name: session_name,
            external_id: external_id,
            session_duration:
                session_duration.unwrap_or(Duration::seconds(DEFAULT_ROLE_DURATION_SECONDS as i64)),
            scope_down_policy: scope_down_policy,
            mfa_serial: mfa_serial,
            mfa_code: None,
        }
    }

    /// Set the MFA code for use when acquiring session tokens.
    pub fn set_mfa_code<S>(&mut self, code: S)
        where S: Into<String>
    {
        self.mfa_code = Some(code.into());
    }

    /// Clear the MFA code.
    pub fn clear_mfa_code(&mut self) {
        self.mfa_code = None;
    }

    /// Calls `AssumeRole` to get a session token from the STS Api.
    /// Optionally uses MFA if the MFA serial number and code are set.
    pub fn assume_role(&self) -> Result<AwsCredentials, CredentialsError> {
        match self.sts_client
                  .assume_role(&AssumeRoleRequest {
                                    role_arn: self.role_arn.clone(),
                                    role_session_name: self.session_name.clone(),
                                    duration_seconds: Some(self.session_duration.num_seconds() as
                                                           i64),
                                    external_id: self.external_id.clone(),
                                    policy: self.scope_down_policy.clone(),
                                    serial_number: self.mfa_serial.clone(),
                                    token_code: self.mfa_code.clone(),
                                    ..Default::default()
                                }) {
            Err(err) => Err(CredentialsError::new(format!("Sts AssumeRoleError: {:?}", err))),
            Ok(resp) => {
                resp.credentials
                    .ok_or(CredentialsError::new("no credentials in response"))?
                    .into()
            }
        }
    }
}

impl<T: Sts> ProvideAwsCredentials for StsAssumeRoleSessionCredentialsProvider<T> {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        self.assume_role()
    }
}

/// [AwsCredentials](../struct.AwsCredentials.html) provider that calls
/// `AssumeRoleWithWebIdentity` using the provided [Sts](trait.Sts.html) client.
pub struct StsWebIdentityFederationSessionCredentialsProvider<T: Sts> {
    sts_client: T,
    wif_token: String,
    wif_provider: Option<String>,
    role_arn: String,
    session_name: String,
    session_duration: Duration,
    scope_down_policy: Option<String>,
}

impl<T: Sts> StsWebIdentityFederationSessionCredentialsProvider<T> {
    /// Creates a new `StsWebIdentityFederationSessionCredentialsProvider` with the given
    /// [Sts](trait.Sts.html) client and session parameters.
    ///
    /// * `sts_client` - The [Sts](trait.Sts.html) client to use to acquire session tokens.
    /// * `wif_token` - The OAuth 2.0 access token or OpenID Connect ID token that is provided by the identity provider.
    /// * `wif_provider` - The fully qualified host component of the domain name of the identity provider. Only for OAuth 2.0 access tokens. Do not include URL schemes and port numbers.
    /// * `role_arn` - The ARN of the role to assume.
    /// * `session_name` - An identifier for the assumed role session. Minimum length of 2. Maximum length of 64. Pattern: `[\w+=,.@-]*`
    /// * `session_duration` - Duration of session tokens. Default 1 hour.
    /// * `scope_down_policy` - Optional inline IAM policy in JSON format to further restrict the access granted to the negotiated session.
    pub fn new(sts_client: T,
               wif_token: String,
               wif_provider: Option<String>,
               role_arn: String,
               session_name: String,
               session_duration: Option<Duration>,
               scope_down_policy: Option<String>)
               -> Self {
        StsWebIdentityFederationSessionCredentialsProvider {
            sts_client: sts_client,
            wif_token: wif_token,
            wif_provider: wif_provider,
            role_arn: role_arn,
            session_name: session_name,
            session_duration: session_duration
                .unwrap_or(Duration::seconds(DEFAULT_DURATION_SECONDS as i64)),
            scope_down_policy: scope_down_policy,
        }
    }

    /// Calls `AssumeRoleWithWebIdentity` to get a session token from the STS Api.
    pub fn assume_role_with_web_identity(&self) -> Result<AwsCredentials, CredentialsError> {
        match self.sts_client
                  .assume_role_with_web_identity(&AssumeRoleWithWebIdentityRequest {
                                                      web_identity_token: self.wif_token.clone(),
                                                      provider_id: self.wif_provider.clone(),
                                                      role_arn: self.role_arn.clone(),
                                                      role_session_name: self.session_name.clone(),
                                                      duration_seconds:
                                                          Some(self.session_duration
                                                                   .num_seconds() as
                                                               i64),
                                                      policy: self.scope_down_policy.clone(),
                                                      ..Default::default()
                                                  }) {
            Err(err) => {
                Err(CredentialsError::new(format!("Sts AssumeRoleWithWebIdentityError: {:?}", err)))
            }
            Ok(resp) => {
                let mut aws_creds: AwsCredentials = resp.credentials
                    .ok_or(CredentialsError::new("no credentials in response"))?
                    .into();

                if let Some(subject_from_wif) = resp.subject_from_web_identity_token {
                    aws_creds
                        .claims_mut()
                        .insert(rusoto_core::credential::claims::SUBJECT.to_owned(),
                                subject_from_wif);
                }

                if let Some(audience) = resp.audience {
                    aws_creds
                        .claims_mut()
                        .insert(rusoto_core::credential::claims::AUDIENCE.to_owned(),
                                audience);
                }

                if let Some(issuer) = resp.provider {
                    aws_creds
                        .claims_mut()
                        .insert(rusoto_core::credential::claims::ISSUER.to_owned(), issuer);
                }

                Ok(aws_creds)
            }
        }
    }
}

impl<T: Sts> ProvideAwsCredentials for StsWebIdentityFederationSessionCredentialsProvider<T> {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        self.assume_role_with_web_identity()
    }
}