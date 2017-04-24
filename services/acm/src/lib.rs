extern crate hyper;
extern crate rusoto;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[allow(warnings)]
        use hyper::Client;
        use hyper::status::StatusCode;
        use rusoto::request::DispatchSignedRequest;
        use rusoto::region;

        use std::fmt;
        use std::error::Error;
        use rusoto::request::HttpDispatchError;
        use rusoto::{CredentialsError, ProvideAwsCredentials};
    
use rusoto::signature::SignedRequest;
        use serde_json::Value as SerdeJsonValue;
        use serde_json::from_str;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AddTagsToCertificateRequest {
                #[doc="<p>String that contains the ARN of the ACM Certificate to which the tag is to be applied. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Arn,
#[doc="<p>The key-value pair that defines the tag. The tag value is optional.</p>"]
#[serde(rename="Tags")]
pub tags: TagList,
            }
            
pub type Arn = String;
pub type CertificateBody = String;
pub type CertificateBodyBlob = Vec<u8>;
pub type CertificateChain = String;
pub type CertificateChainBlob = Vec<u8>;
#[doc="<p>Contains detailed metadata about an ACM Certificate. This structure is returned in the response to a <a>DescribeCertificate</a> request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CertificateDetail {
                #[doc="<p>The Amazon Resource Name (ARN) of the certificate. For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Option<Arn>,
#[doc="<p>The time at which the certificate was requested. This value exists only when the certificate type is <code>AMAZON_ISSUED</code>.</p>"]
#[serde(rename="CreatedAt")]
pub created_at: Option<TStamp>,
#[doc="<p>The fully qualified domain name for the certificate, such as www.example.com or example.com.</p>"]
#[serde(rename="DomainName")]
pub domain_name: Option<DomainNameString>,
#[doc="<p>Contains information about the email address or addresses used for domain validation. This field exists only when the certificate type is <code>AMAZON_ISSUED</code>.</p>"]
#[serde(rename="DomainValidationOptions")]
pub domain_validation_options: Option<DomainValidationList>,
#[doc="<p>The reason the certificate request failed. This value exists only when the certificate status is <code>FAILED</code>. For more information, see <a href=\"http://docs.aws.amazon.com/acm/latest/userguide/troubleshooting.html#troubleshooting-failed\">Certificate Request Failed</a> in the <i>AWS Certificate Manager User Guide</i>.</p>"]
#[serde(rename="FailureReason")]
pub failure_reason: Option<FailureReason>,
#[doc="<p>The date and time at which the certificate was imported. This value exists only when the certificate type is <code>IMPORTED</code>.</p>"]
#[serde(rename="ImportedAt")]
pub imported_at: Option<TStamp>,
#[doc="<p>A list of ARNs for the AWS resources that are using the certificate. A certificate can be used by multiple AWS resources.</p>"]
#[serde(rename="InUseBy")]
pub in_use_by: Option<InUseList>,
#[doc="<p>The time at which the certificate was issued. This value exists only when the certificate type is <code>AMAZON_ISSUED</code>.</p>"]
#[serde(rename="IssuedAt")]
pub issued_at: Option<TStamp>,
#[doc="<p>The name of the certificate authority that issued and signed the certificate.</p>"]
#[serde(rename="Issuer")]
pub issuer: Option<String>,
#[doc="<p>The algorithm that was used to generate the key pair (the public and private key).</p>"]
#[serde(rename="KeyAlgorithm")]
pub key_algorithm: Option<KeyAlgorithm>,
#[doc="<p>The time after which the certificate is not valid.</p>"]
#[serde(rename="NotAfter")]
pub not_after: Option<TStamp>,
#[doc="<p>The time before which the certificate is not valid.</p>"]
#[serde(rename="NotBefore")]
pub not_before: Option<TStamp>,
#[doc="<p>The reason the certificate was revoked. This value exists only when the certificate status is <code>REVOKED</code>.</p>"]
#[serde(rename="RevocationReason")]
pub revocation_reason: Option<RevocationReason>,
#[doc="<p>The time at which the certificate was revoked. This value exists only when the certificate status is <code>REVOKED</code>.</p>"]
#[serde(rename="RevokedAt")]
pub revoked_at: Option<TStamp>,
#[doc="<p>The serial number of the certificate.</p>"]
#[serde(rename="Serial")]
pub serial: Option<String>,
#[doc="<p>The algorithm that was used to sign the certificate.</p>"]
#[serde(rename="SignatureAlgorithm")]
pub signature_algorithm: Option<String>,
#[doc="<p>The status of the certificate.</p>"]
#[serde(rename="Status")]
pub status: Option<CertificateStatus>,
#[doc="<p>The name of the entity that is associated with the public key contained in the certificate.</p>"]
#[serde(rename="Subject")]
pub subject: Option<String>,
#[doc="<p>One or more domain names (subject alternative names) included in the certificate. This list contains the domain names that are bound to the public key that is contained in the certificate. The subject alternative names include the canonical domain name (CN) of the certificate and additional domain names that can be used to connect to the website.</p>"]
#[serde(rename="SubjectAlternativeNames")]
pub subject_alternative_names: Option<DomainList>,
#[doc="<p>The source of the certificate. For certificates provided by ACM, this value is <code>AMAZON_ISSUED</code>. For certificates that you imported with <a>ImportCertificate</a>, this value is <code>IMPORTED</code>. ACM does not provide <a href=\"http://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html\">managed renewal</a> for imported certificates. For more information about the differences between certificates that you import and those that ACM provides, see <a href=\"http://docs.aws.amazon.com/acm/latest/userguide/import-certificate.html\">Importing Certificates</a> in the <i>AWS Certificate Manager User Guide</i>.</p>"]
#[serde(rename="Type")]
pub type_: Option<CertificateType>,
            }
            
pub type CertificateStatus = String;
pub type CertificateStatuses = Vec<CertificateStatus>;
#[doc="<p>This structure is returned in the response object of <a>ListCertificates</a> action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CertificateSummary {
                #[doc="<p>Amazon Resource Name (ARN) of the certificate. This is of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Option<Arn>,
#[doc="<p>Fully qualified domain name (FQDN), such as www.example.com or example.com, for the certificate.</p>"]
#[serde(rename="DomainName")]
pub domain_name: Option<DomainNameString>,
            }
            
pub type CertificateSummaryList = Vec<CertificateSummary>;
pub type CertificateType = String;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteCertificateRequest {
                #[doc="<p>String that contains the ARN of the ACM Certificate to be deleted. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Arn,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeCertificateRequest {
                #[doc="<p>String that contains an ACM Certificate ARN. The ARN must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Arn,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeCertificateResponse {
                #[doc="<p>Contains a <a>CertificateDetail</a> structure that lists the fields of an ACM Certificate.</p>"]
#[serde(rename="Certificate")]
pub certificate: Option<CertificateDetail>,
            }
            
pub type DomainList = Vec<DomainNameString>;
pub type DomainNameString = String;
#[doc="<p>Structure that contains the domain name, the base validation domain to which validation email is sent, and the email addresses used to validate the domain identity.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DomainValidation {
                #[doc="<p>Fully Qualified Domain Name (FQDN) of the form <code>www.example.com or </code> <code>example.com</code>.</p>"]
#[serde(rename="DomainName")]
pub domain_name: DomainNameString,
#[doc="<p>The base validation domain that acts as the suffix of the email addresses that are used to send the emails.</p>"]
#[serde(rename="ValidationDomain")]
pub validation_domain: Option<DomainNameString>,
#[doc="<p>A list of contact address for the domain registrant.</p>"]
#[serde(rename="ValidationEmails")]
pub validation_emails: Option<ValidationEmailList>,
            }
            
pub type DomainValidationList = Vec<DomainValidation>;
#[doc="<p>This structure is used in the request object of the <a>RequestCertificate</a> action.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DomainValidationOption {
                #[doc="<p>Fully Qualified Domain Name (FQDN) of the certificate being requested.</p>"]
#[serde(rename="DomainName")]
pub domain_name: DomainNameString,
#[doc="<p>The domain to which validation email is sent. This is the base validation domain that will act as the suffix of the email addresses. This must be the same as the <code>DomainName</code> value or a superdomain of the <code>DomainName</code> value. For example, if you requested a certificate for <code>site.subdomain.example.com</code> and specify a <b>ValidationDomain</b> of <code>subdomain.example.com</code>, ACM sends email to the domain registrant, technical contact, and administrative contact in WHOIS for the base domain and the following five addresses:</p> <ul> <li> <p>admin@subdomain.example.com</p> </li> <li> <p>administrator@subdomain.example.com</p> </li> <li> <p>hostmaster@subdomain.example.com</p> </li> <li> <p>postmaster@subdomain.example.com</p> </li> <li> <p>webmaster@subdomain.example.com</p> </li> </ul>"]
#[serde(rename="ValidationDomain")]
pub validation_domain: DomainNameString,
            }
            
pub type DomainValidationOptionList = Vec<DomainValidationOption>;
pub type FailureReason = String;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetCertificateRequest {
                #[doc="<p>String that contains a certificate ARN in the following format:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Arn,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetCertificateResponse {
                #[doc="<p>String that contains the ACM Certificate represented by the ARN specified at input.</p>"]
#[serde(rename="Certificate")]
pub certificate: Option<CertificateBody>,
#[doc="<p>The certificate chain that contains the root certificate issued by the certificate authority (CA).</p>"]
#[serde(rename="CertificateChain")]
pub certificate_chain: Option<CertificateChain>,
            }
            
pub type IdempotencyToken = String;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ImportCertificateRequest {
                #[doc="<p>The certificate to import. It must meet the following requirements:</p> <ul> <li> <p>Must be PEM-encoded.</p> </li> <li> <p>Must contain a 1024-bit or 2048-bit RSA public key.</p> </li> <li> <p>Must be valid at the time of import. You cannot import a certificate before its validity period begins (the certificate's <code>NotBefore</code> date) or after it expires (the certificate's <code>NotAfter</code> date).</p> </li> </ul>"]
#[serde(rename="Certificate")]
#[serde(
                            deserialize_with="rusoto::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="rusoto::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
pub certificate: CertificateBodyBlob,
#[doc="<p>The <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Name (ARN)</a> of an imported certificate to replace. To import a new certificate, omit this field.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Option<Arn>,
#[doc="<p>The certificate chain. It must be PEM-encoded.</p>"]
#[serde(rename="CertificateChain")]
#[serde(
                            deserialize_with="rusoto::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="rusoto::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
pub certificate_chain: Option<CertificateChainBlob>,
#[doc="<p>The private key that matches the public key in the certificate. It must meet the following requirements:</p> <ul> <li> <p>Must be PEM-encoded.</p> </li> <li> <p>Must be unencrypted. You cannot import a private key that is protected by a password or passphrase.</p> </li> </ul>"]
#[serde(rename="PrivateKey")]
#[serde(
                            deserialize_with="rusoto::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="rusoto::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
pub private_key: PrivateKeyBlob,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ImportCertificateResponse {
                #[doc="<p>The <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Name (ARN)</a> of the imported certificate.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Option<Arn>,
            }
            
pub type InUseList = Vec<String>;
pub type KeyAlgorithm = String;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListCertificatesRequest {
                #[doc="<p>The status or statuses on which to filter the list of ACM Certificates.</p>"]
#[serde(rename="CertificateStatuses")]
pub certificate_statuses: Option<CertificateStatuses>,
#[doc="<p>Use this parameter when paginating results to specify the maximum number of items to return in the response. If additional items exist beyond the number you specify, the <code>NextToken</code> element is sent in the response. Use this <code>NextToken</code> value in a subsequent request to retrieve additional items.</p>"]
#[serde(rename="MaxItems")]
pub max_items: Option<MaxItems>,
#[doc="<p>Use this parameter only when paginating results and only in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextToken</code> from the response you just received.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListCertificatesResponse {
                #[doc="<p>A list of ACM Certificates.</p>"]
#[serde(rename="CertificateSummaryList")]
pub certificate_summary_list: Option<CertificateSummaryList>,
#[doc="<p>When the list is truncated, this value is present and contains the value to use for the <code>NextToken</code> parameter in a subsequent pagination request.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListTagsForCertificateRequest {
                #[doc="<p>String that contains the ARN of the ACM Certificate for which you want to list the tags. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Arn,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListTagsForCertificateResponse {
                #[doc="<p>The key-value pairs that define the applied tags.</p>"]
#[serde(rename="Tags")]
pub tags: Option<TagList>,
            }
            
pub type MaxItems = i64;
pub type NextToken = String;
pub type PrivateKeyBlob = Vec<u8>;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RemoveTagsFromCertificateRequest {
                #[doc="<p>String that contains the ARN of the ACM Certificate with one or more tags that you want to remove. This must be of the form:</p> <p> <code>arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Arn,
#[doc="<p>The key-value pair that defines the tag to remove.</p>"]
#[serde(rename="Tags")]
pub tags: TagList,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RequestCertificateRequest {
                #[doc="<p>Fully qualified domain name (FQDN), such as www.example.com, of the site you want to secure with an ACM Certificate. Use an asterisk (*) to create a wildcard certificate that protects several sites in the same domain. For example, *.example.com protects www.example.com, site.example.com, and images.example.com.</p>"]
#[serde(rename="DomainName")]
pub domain_name: DomainNameString,
#[doc="<p>The base validation domain that will act as the suffix of the email addresses that are used to send the emails. This must be the same as the <code>Domain</code> value or a superdomain of the <code>Domain</code> value. For example, if you requested a certificate for <code>test.example.com</code> and specify <b>DomainValidationOptions</b> of <code>example.com</code>, ACM sends email to the domain registrant, technical contact, and administrative contact in WHOIS and the following five addresses:</p> <ul> <li> <p>admin@example.com</p> </li> <li> <p>administrator@example.com</p> </li> <li> <p>hostmaster@example.com</p> </li> <li> <p>postmaster@example.com</p> </li> <li> <p>webmaster@example.com</p> </li> </ul>"]
#[serde(rename="DomainValidationOptions")]
pub domain_validation_options: Option<DomainValidationOptionList>,
#[doc="<p>Customer chosen string that can be used to distinguish between calls to <code>RequestCertificate</code>. Idempotency tokens time out after one hour. Therefore, if you call <code>RequestCertificate</code> multiple times with the same idempotency token within one hour, ACM recognizes that you are requesting only one certificate and will issue only one. If you change the idempotency token for each call, ACM recognizes that you are requesting multiple certificates.</p>"]
#[serde(rename="IdempotencyToken")]
pub idempotency_token: Option<IdempotencyToken>,
#[doc="<p>Additional FQDNs to be included in the Subject Alternative Name extension of the ACM Certificate. For example, add the name www.example.net to a certificate for which the <code>DomainName</code> field is www.example.com if users can reach your site by using either name.</p>"]
#[serde(rename="SubjectAlternativeNames")]
pub subject_alternative_names: Option<DomainList>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RequestCertificateResponse {
                #[doc="<p>String that contains the ARN of the issued certificate. This must be of the form:</p> <p> <code>arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Option<Arn>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ResendValidationEmailRequest {
                #[doc="<p>String that contains the ARN of the requested certificate. The certificate ARN is generated and returned by the <a>RequestCertificate</a> action as soon as the request is made. By default, using this parameter causes email to be sent to all top-level domains you specified in the certificate request.</p> <p>The ARN must be of the form:</p> <p> <code>arn:aws:acm:us-east-1:123456789012:certificate/12345678-1234-1234-1234-123456789012</code> </p>"]
#[serde(rename="CertificateArn")]
pub certificate_arn: Arn,
#[doc="<p>The Fully Qualified Domain Name (FQDN) of the certificate that needs to be validated.</p>"]
#[serde(rename="Domain")]
pub domain: DomainNameString,
#[doc="<p>The base validation domain that will act as the suffix of the email addresses that are used to send the emails. This must be the same as the <code>Domain</code> value or a superdomain of the <code>Domain</code> value. For example, if you requested a certificate for <code>site.subdomain.example.com</code> and specify a <b>ValidationDomain</b> of <code>subdomain.example.com</code>, ACM sends email to the domain registrant, technical contact, and administrative contact in WHOIS and the following five addresses:</p> <ul> <li> <p>admin@subdomain.example.com</p> </li> <li> <p>administrator@subdomain.example.com</p> </li> <li> <p>hostmaster@subdomain.example.com</p> </li> <li> <p>postmaster@subdomain.example.com</p> </li> <li> <p>webmaster@subdomain.example.com</p> </li> </ul>"]
#[serde(rename="ValidationDomain")]
pub validation_domain: DomainNameString,
            }
            
pub type RevocationReason = String;
pub type TStamp = f64;
#[doc="<p>A key-value pair that identifies or specifies metadata about an ACM resource.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Tag {
                #[doc="<p>The key of the tag.</p>"]
#[serde(rename="Key")]
pub key: TagKey,
#[doc="<p>The value of the tag.</p>"]
#[serde(rename="Value")]
pub value: Option<TagValue>,
            }
            
pub type TagKey = String;
pub type TagList = Vec<Tag>;
pub type TagValue = String;
pub type ValidationEmailList = Vec<String>;
/// Errors returned by AddTagsToCertificate
                #[derive(Debug, PartialEq)]
                pub enum AddTagsToCertificateError {
                    
///<p>The specified certificate cannot be found in the caller's account, or the caller's account cannot be found.</p>
ResourceNotFound(String),
///<p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
InvalidTag(String),
///<p>The request contains too many tags. Try the request again with fewer tags.</p>
TooManyTags(String),
///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
InvalidArn(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AddTagsToCertificateError {
                    pub fn from_body(body: &str) -> AddTagsToCertificateError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "TooManyTagsException" => AddTagsToCertificateError::TooManyTags(String::from(error_message)),"ResourceNotFoundException" => AddTagsToCertificateError::ResourceNotFound(String::from(error_message)),"InvalidArnException" => AddTagsToCertificateError::InvalidArn(String::from(error_message)),"InvalidTagException" => AddTagsToCertificateError::InvalidTag(String::from(error_message)),"ValidationException" => AddTagsToCertificateError::Validation(error_message.to_string()),_ => AddTagsToCertificateError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AddTagsToCertificateError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for AddTagsToCertificateError {
                    fn from(err: serde_json::error::Error) -> AddTagsToCertificateError {
                        AddTagsToCertificateError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for AddTagsToCertificateError {
                    fn from(err: CredentialsError) -> AddTagsToCertificateError {
                        AddTagsToCertificateError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for AddTagsToCertificateError {
                    fn from(err: HttpDispatchError) -> AddTagsToCertificateError {
                        AddTagsToCertificateError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for AddTagsToCertificateError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for AddTagsToCertificateError {
                    fn description(&self) -> &str {
                        match *self {
                            AddTagsToCertificateError::InvalidTag(ref cause) => cause,AddTagsToCertificateError::ResourceNotFound(ref cause) => cause,AddTagsToCertificateError::TooManyTags(ref cause) => cause,AddTagsToCertificateError::InvalidArn(ref cause) => cause,AddTagsToCertificateError::Validation(ref cause) => cause,AddTagsToCertificateError::Credentials(ref err) => err.description(),AddTagsToCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),AddTagsToCertificateError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteCertificate
                #[derive(Debug, PartialEq)]
                pub enum DeleteCertificateError {
                    
///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
InvalidArn(String),
///<p>The certificate is in use by another AWS service in the caller's account. Remove the association and try again.</p>
ResourceInUse(String),
///<p>The specified certificate cannot be found in the caller's account, or the caller's account cannot be found.</p>
ResourceNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteCertificateError {
                    pub fn from_body(body: &str) -> DeleteCertificateError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidArnException" => DeleteCertificateError::InvalidArn(String::from(error_message)),"ResourceInUseException" => DeleteCertificateError::ResourceInUse(String::from(error_message)),"ResourceNotFoundException" => DeleteCertificateError::ResourceNotFound(String::from(error_message)),"ValidationException" => DeleteCertificateError::Validation(error_message.to_string()),_ => DeleteCertificateError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteCertificateError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteCertificateError {
                    fn from(err: serde_json::error::Error) -> DeleteCertificateError {
                        DeleteCertificateError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeleteCertificateError {
                    fn from(err: CredentialsError) -> DeleteCertificateError {
                        DeleteCertificateError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteCertificateError {
                    fn from(err: HttpDispatchError) -> DeleteCertificateError {
                        DeleteCertificateError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteCertificateError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteCertificateError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteCertificateError::InvalidArn(ref cause) => cause,DeleteCertificateError::ResourceInUse(ref cause) => cause,DeleteCertificateError::ResourceNotFound(ref cause) => cause,DeleteCertificateError::Validation(ref cause) => cause,DeleteCertificateError::Credentials(ref err) => err.description(),DeleteCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteCertificateError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeCertificate
                #[derive(Debug, PartialEq)]
                pub enum DescribeCertificateError {
                    
///<p>The specified certificate cannot be found in the caller's account, or the caller's account cannot be found.</p>
ResourceNotFound(String),
///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
InvalidArn(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeCertificateError {
                    pub fn from_body(body: &str) -> DescribeCertificateError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ResourceNotFoundException" => DescribeCertificateError::ResourceNotFound(String::from(error_message)),"InvalidArnException" => DescribeCertificateError::InvalidArn(String::from(error_message)),"ValidationException" => DescribeCertificateError::Validation(error_message.to_string()),_ => DescribeCertificateError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeCertificateError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeCertificateError {
                    fn from(err: serde_json::error::Error) -> DescribeCertificateError {
                        DescribeCertificateError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeCertificateError {
                    fn from(err: CredentialsError) -> DescribeCertificateError {
                        DescribeCertificateError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeCertificateError {
                    fn from(err: HttpDispatchError) -> DescribeCertificateError {
                        DescribeCertificateError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeCertificateError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeCertificateError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeCertificateError::InvalidArn(ref cause) => cause,DescribeCertificateError::ResourceNotFound(ref cause) => cause,DescribeCertificateError::Validation(ref cause) => cause,DescribeCertificateError::Credentials(ref err) => err.description(),DescribeCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeCertificateError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetCertificate
                #[derive(Debug, PartialEq)]
                pub enum GetCertificateError {
                    
///<p>The specified certificate cannot be found in the caller's account, or the caller's account cannot be found.</p>
ResourceNotFound(String),
///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
InvalidArn(String),
///<p>The certificate request is in process and the certificate in your account has not yet been issued.</p>
RequestInProgress(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetCertificateError {
                    pub fn from_body(body: &str) -> GetCertificateError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ResourceNotFoundException" => GetCertificateError::ResourceNotFound(String::from(error_message)),"InvalidArnException" => GetCertificateError::InvalidArn(String::from(error_message)),"RequestInProgressException" => GetCertificateError::RequestInProgress(String::from(error_message)),"ValidationException" => GetCertificateError::Validation(error_message.to_string()),_ => GetCertificateError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetCertificateError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetCertificateError {
                    fn from(err: serde_json::error::Error) -> GetCertificateError {
                        GetCertificateError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetCertificateError {
                    fn from(err: CredentialsError) -> GetCertificateError {
                        GetCertificateError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetCertificateError {
                    fn from(err: HttpDispatchError) -> GetCertificateError {
                        GetCertificateError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetCertificateError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetCertificateError {
                    fn description(&self) -> &str {
                        match *self {
                            GetCertificateError::RequestInProgress(ref cause) => cause,GetCertificateError::ResourceNotFound(ref cause) => cause,GetCertificateError::InvalidArn(ref cause) => cause,GetCertificateError::Validation(ref cause) => cause,GetCertificateError::Credentials(ref err) => err.description(),GetCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetCertificateError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ImportCertificate
                #[derive(Debug, PartialEq)]
                pub enum ImportCertificateError {
                    
///<p>The specified certificate cannot be found in the caller's account, or the caller's account cannot be found.</p>
ResourceNotFound(String),
///<p>An ACM limit has been exceeded. For example, you may have input more domains than are allowed or you've requested too many certificates for your account. See the exception message returned by ACM to determine which limit you have violated. For more information about ACM limits, see the <a href="http://docs.aws.amazon.com/acm/latest/userguide/acm-limits.html">Limits</a> topic.</p>
LimitExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ImportCertificateError {
                    pub fn from_body(body: &str) -> ImportCertificateError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ResourceNotFoundException" => ImportCertificateError::ResourceNotFound(String::from(error_message)),"LimitExceededException" => ImportCertificateError::LimitExceeded(String::from(error_message)),"ValidationException" => ImportCertificateError::Validation(error_message.to_string()),_ => ImportCertificateError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ImportCertificateError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ImportCertificateError {
                    fn from(err: serde_json::error::Error) -> ImportCertificateError {
                        ImportCertificateError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ImportCertificateError {
                    fn from(err: CredentialsError) -> ImportCertificateError {
                        ImportCertificateError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ImportCertificateError {
                    fn from(err: HttpDispatchError) -> ImportCertificateError {
                        ImportCertificateError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ImportCertificateError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ImportCertificateError {
                    fn description(&self) -> &str {
                        match *self {
                            ImportCertificateError::ResourceNotFound(ref cause) => cause,ImportCertificateError::LimitExceeded(ref cause) => cause,ImportCertificateError::Validation(ref cause) => cause,ImportCertificateError::Credentials(ref err) => err.description(),ImportCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ImportCertificateError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListCertificates
                #[derive(Debug, PartialEq)]
                pub enum ListCertificatesError {
                    /// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListCertificatesError {
                    pub fn from_body(body: &str) -> ListCertificatesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ValidationException" => ListCertificatesError::Validation(error_message.to_string()),_ => ListCertificatesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListCertificatesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListCertificatesError {
                    fn from(err: serde_json::error::Error) -> ListCertificatesError {
                        ListCertificatesError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListCertificatesError {
                    fn from(err: CredentialsError) -> ListCertificatesError {
                        ListCertificatesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListCertificatesError {
                    fn from(err: HttpDispatchError) -> ListCertificatesError {
                        ListCertificatesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListCertificatesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListCertificatesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListCertificatesError::Validation(ref cause) => cause,ListCertificatesError::Credentials(ref err) => err.description(),ListCertificatesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListCertificatesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTagsForCertificate
                #[derive(Debug, PartialEq)]
                pub enum ListTagsForCertificateError {
                    
///<p>The specified certificate cannot be found in the caller's account, or the caller's account cannot be found.</p>
ResourceNotFound(String),
///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
InvalidArn(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTagsForCertificateError {
                    pub fn from_body(body: &str) -> ListTagsForCertificateError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidArnException" => ListTagsForCertificateError::InvalidArn(String::from(error_message)),"ResourceNotFoundException" => ListTagsForCertificateError::ResourceNotFound(String::from(error_message)),"ValidationException" => ListTagsForCertificateError::Validation(error_message.to_string()),_ => ListTagsForCertificateError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListTagsForCertificateError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListTagsForCertificateError {
                    fn from(err: serde_json::error::Error) -> ListTagsForCertificateError {
                        ListTagsForCertificateError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListTagsForCertificateError {
                    fn from(err: CredentialsError) -> ListTagsForCertificateError {
                        ListTagsForCertificateError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTagsForCertificateError {
                    fn from(err: HttpDispatchError) -> ListTagsForCertificateError {
                        ListTagsForCertificateError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTagsForCertificateError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTagsForCertificateError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTagsForCertificateError::ResourceNotFound(ref cause) => cause,ListTagsForCertificateError::InvalidArn(ref cause) => cause,ListTagsForCertificateError::Validation(ref cause) => cause,ListTagsForCertificateError::Credentials(ref err) => err.description(),ListTagsForCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTagsForCertificateError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RemoveTagsFromCertificate
                #[derive(Debug, PartialEq)]
                pub enum RemoveTagsFromCertificateError {
                    
///<p>One or both of the values that make up the key-value pair is not valid. For example, you cannot specify a tag value that begins with <code>aws:</code>.</p>
InvalidTag(String),
///<p>The specified certificate cannot be found in the caller's account, or the caller's account cannot be found.</p>
ResourceNotFound(String),
///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
InvalidArn(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RemoveTagsFromCertificateError {
                    pub fn from_body(body: &str) -> RemoveTagsFromCertificateError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidArnException" => RemoveTagsFromCertificateError::InvalidArn(String::from(error_message)),"ResourceNotFoundException" => RemoveTagsFromCertificateError::ResourceNotFound(String::from(error_message)),"InvalidTagException" => RemoveTagsFromCertificateError::InvalidTag(String::from(error_message)),"ValidationException" => RemoveTagsFromCertificateError::Validation(error_message.to_string()),_ => RemoveTagsFromCertificateError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RemoveTagsFromCertificateError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RemoveTagsFromCertificateError {
                    fn from(err: serde_json::error::Error) -> RemoveTagsFromCertificateError {
                        RemoveTagsFromCertificateError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RemoveTagsFromCertificateError {
                    fn from(err: CredentialsError) -> RemoveTagsFromCertificateError {
                        RemoveTagsFromCertificateError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RemoveTagsFromCertificateError {
                    fn from(err: HttpDispatchError) -> RemoveTagsFromCertificateError {
                        RemoveTagsFromCertificateError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RemoveTagsFromCertificateError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RemoveTagsFromCertificateError {
                    fn description(&self) -> &str {
                        match *self {
                            RemoveTagsFromCertificateError::InvalidTag(ref cause) => cause,RemoveTagsFromCertificateError::InvalidArn(ref cause) => cause,RemoveTagsFromCertificateError::ResourceNotFound(ref cause) => cause,RemoveTagsFromCertificateError::Validation(ref cause) => cause,RemoveTagsFromCertificateError::Credentials(ref err) => err.description(),RemoveTagsFromCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RemoveTagsFromCertificateError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RequestCertificate
                #[derive(Debug, PartialEq)]
                pub enum RequestCertificateError {
                    
///<p>One or more values in the <a>DomainValidationOption</a> structure is incorrect.</p>
InvalidDomainValidationOptions(String),
///<p>An ACM limit has been exceeded. For example, you may have input more domains than are allowed or you've requested too many certificates for your account. See the exception message returned by ACM to determine which limit you have violated. For more information about ACM limits, see the <a href="http://docs.aws.amazon.com/acm/latest/userguide/acm-limits.html">Limits</a> topic.</p>
LimitExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RequestCertificateError {
                    pub fn from_body(body: &str) -> RequestCertificateError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidDomainValidationOptionsException" => RequestCertificateError::InvalidDomainValidationOptions(String::from(error_message)),"LimitExceededException" => RequestCertificateError::LimitExceeded(String::from(error_message)),"ValidationException" => RequestCertificateError::Validation(error_message.to_string()),_ => RequestCertificateError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RequestCertificateError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RequestCertificateError {
                    fn from(err: serde_json::error::Error) -> RequestCertificateError {
                        RequestCertificateError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RequestCertificateError {
                    fn from(err: CredentialsError) -> RequestCertificateError {
                        RequestCertificateError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RequestCertificateError {
                    fn from(err: HttpDispatchError) -> RequestCertificateError {
                        RequestCertificateError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RequestCertificateError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RequestCertificateError {
                    fn description(&self) -> &str {
                        match *self {
                            RequestCertificateError::InvalidDomainValidationOptions(ref cause) => cause,RequestCertificateError::LimitExceeded(ref cause) => cause,RequestCertificateError::Validation(ref cause) => cause,RequestCertificateError::Credentials(ref err) => err.description(),RequestCertificateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RequestCertificateError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ResendValidationEmail
                #[derive(Debug, PartialEq)]
                pub enum ResendValidationEmailError {
                    
///<p>One or more values in the <a>DomainValidationOption</a> structure is incorrect.</p>
InvalidDomainValidationOptions(String),
///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
InvalidArn(String),
///<p>The specified certificate cannot be found in the caller's account, or the caller's account cannot be found.</p>
ResourceNotFound(String),
///<p>Processing has reached an invalid state. For example, this exception can occur if the specified domain is not using email validation, or the current certificate status does not permit the requested operation. See the exception message returned by ACM to determine which state is not valid.</p>
InvalidState(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ResendValidationEmailError {
                    pub fn from_body(body: &str) -> ResendValidationEmailError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidStateException" => ResendValidationEmailError::InvalidState(String::from(error_message)),"InvalidArnException" => ResendValidationEmailError::InvalidArn(String::from(error_message)),"ResourceNotFoundException" => ResendValidationEmailError::ResourceNotFound(String::from(error_message)),"InvalidDomainValidationOptionsException" => ResendValidationEmailError::InvalidDomainValidationOptions(String::from(error_message)),"ValidationException" => ResendValidationEmailError::Validation(error_message.to_string()),_ => ResendValidationEmailError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ResendValidationEmailError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ResendValidationEmailError {
                    fn from(err: serde_json::error::Error) -> ResendValidationEmailError {
                        ResendValidationEmailError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ResendValidationEmailError {
                    fn from(err: CredentialsError) -> ResendValidationEmailError {
                        ResendValidationEmailError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ResendValidationEmailError {
                    fn from(err: HttpDispatchError) -> ResendValidationEmailError {
                        ResendValidationEmailError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ResendValidationEmailError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ResendValidationEmailError {
                    fn description(&self) -> &str {
                        match *self {
                            ResendValidationEmailError::InvalidDomainValidationOptions(ref cause) => cause,ResendValidationEmailError::InvalidArn(ref cause) => cause,ResendValidationEmailError::ResourceNotFound(ref cause) => cause,ResendValidationEmailError::InvalidState(ref cause) => cause,ResendValidationEmailError::Validation(ref cause) => cause,ResendValidationEmailError::Credentials(ref err) => err.description(),ResendValidationEmailError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ResendValidationEmailError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// A client for the ACM API.
        pub struct AcmClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> AcmClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  AcmClient {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }

        

                #[doc="<p>Adds one or more tags to an ACM Certificate. Tags are labels that you can use to identify and organize your AWS resources. Each tag consists of a <code>key</code> and an optional <code>value</code>. You specify the certificate on input by its Amazon Resource Name (ARN). You specify the tag by using a key-value pair.</p> <p>You can apply a tag to just one certificate if you want to identify a specific characteristic of that certificate, or you can apply the same tag to multiple certificates if you want to filter for a common relationship among those certificates. Similarly, you can apply the same tag to multiple resources if you want to specify a relationship among those resources. For example, you can add the same tag to an ACM Certificate and an Elastic Load Balancing load balancer to indicate that they are both used by the same website. For more information, see <a href=\"http://docs.aws.amazon.com/acm/latest/userguide/tags.html\">Tagging ACM Certificates</a>.</p> <p>To remove one or more tags, use the <a>RemoveTagsFromCertificate</a> action. To view all of the tags that have been applied to the certificate, use the <a>ListTagsForCertificate</a> action.</p>"]
                pub fn add_tags_to_certificate(&self, input: &AddTagsToCertificateRequest)  -> Result<(), AddTagsToCertificateError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.AddTagsToCertificate");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(AddTagsToCertificateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes an ACM Certificate and its associated private key. If this action succeeds, the certificate no longer appears in the list of ACM Certificates that can be displayed by calling the <a>ListCertificates</a> action or be retrieved by calling the <a>GetCertificate</a> action. The certificate will not be available for use by other AWS services.</p> <note> <p>You cannot delete an ACM Certificate that is being used by another AWS service. To delete a certificate that is in use, the certificate association must first be removed.</p> </note>"]
                pub fn delete_certificate(&self, input: &DeleteCertificateRequest)  -> Result<(), DeleteCertificateError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.DeleteCertificate");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(DeleteCertificateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns a list of the fields contained in the specified ACM Certificate. For example, this action returns the certificate status, a flag that indicates whether the certificate is associated with any other AWS service, and the date at which the certificate request was created. You specify the ACM Certificate on input by its Amazon Resource Name (ARN).</p>"]
                pub fn describe_certificate(&self, input: &DescribeCertificateRequest)  -> Result<DescribeCertificateResponse, DescribeCertificateError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.DescribeCertificate");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeCertificateResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeCertificateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves an ACM Certificate and certificate chain for the certificate specified by an ARN. The chain is an ordered list of certificates that contains the root certificate, intermediate certificates of subordinate CAs, and the ACM Certificate. The certificate and certificate chain are base64 encoded. If you want to decode the certificate chain to see the individual certificate fields, you can use OpenSSL.</p> <note> <p>Currently, ACM Certificates can be used only with Elastic Load Balancing and Amazon CloudFront.</p> </note>"]
                pub fn get_certificate(&self, input: &GetCertificateRequest)  -> Result<GetCertificateResponse, GetCertificateError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.GetCertificate");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetCertificateResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetCertificateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Imports an SSL/TLS certificate into AWS Certificate Manager (ACM) to use with <a href=\"http://docs.aws.amazon.com/acm/latest/userguide/acm-services.html\">ACM's integrated AWS services</a>.</p> <note> <p>ACM does not provide <a href=\"http://docs.aws.amazon.com/acm/latest/userguide/acm-renewal.html\">managed renewal</a> for certificates that you import.</p> </note> <p>For more information about importing certificates into ACM, including the differences between certificates that you import and those that ACM provides, see <a href=\"http://docs.aws.amazon.com/acm/latest/userguide/import-certificate.html\">Importing Certificates</a> in the <i>AWS Certificate Manager User Guide</i>.</p> <p>To import a certificate, you must provide the certificate and the matching private key. When the certificate is not self-signed, you must also provide a certificate chain. You can omit the certificate chain when importing a self-signed certificate.</p> <p>The certificate, private key, and certificate chain must be PEM-encoded. For more information about converting these items to PEM format, see <a href=\"http://docs.aws.amazon.com/acm/latest/userguide/import-certificate.html#import-certificate-troubleshooting\">Importing Certificates Troubleshooting</a> in the <i>AWS Certificate Manager User Guide</i>.</p> <p>To import a new certificate, omit the <code>CertificateArn</code> field. Include this field only when you want to replace a previously imported certificate.</p> <p>This operation returns the <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Name (ARN)</a> of the imported certificate.</p>"]
                pub fn import_certificate(&self, input: &ImportCertificateRequest)  -> Result<ImportCertificateResponse, ImportCertificateError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.ImportCertificate");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ImportCertificateResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ImportCertificateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves a list of ACM Certificates and the domain name for each. You can optionally filter the list to return only the certificates that match the specified status.</p>"]
                pub fn list_certificates(&self, input: &ListCertificatesRequest)  -> Result<ListCertificatesResponse, ListCertificatesError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.ListCertificates");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListCertificatesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListCertificatesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the tags that have been applied to the ACM Certificate. Use the certificate ARN to specify the certificate. To add a tag to an ACM Certificate, use the <a>AddTagsToCertificate</a> action. To delete a tag, use the <a>RemoveTagsFromCertificate</a> action.</p>"]
                pub fn list_tags_for_certificate(&self, input: &ListTagsForCertificateRequest)  -> Result<ListTagsForCertificateResponse, ListTagsForCertificateError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.ListTagsForCertificate");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListTagsForCertificateResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListTagsForCertificateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Remove one or more tags from an ACM Certificate. A tag consists of a key-value pair. If you do not specify the value portion of the tag when calling this function, the tag will be removed regardless of value. If you specify a value, the tag is removed only if it is associated with the specified value.</p> <p>To add tags to a certificate, use the <a>AddTagsToCertificate</a> action. To view all of the tags that have been applied to a specific ACM Certificate, use the <a>ListTagsForCertificate</a> action.</p>"]
                pub fn remove_tags_from_certificate(&self, input: &RemoveTagsFromCertificateRequest)  -> Result<(), RemoveTagsFromCertificateError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.RemoveTagsFromCertificate");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(RemoveTagsFromCertificateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Requests an ACM Certificate for use with other AWS services. To request an ACM Certificate, you must specify the fully qualified domain name (FQDN) for your site. You can also specify additional FQDNs if users can reach your site by using other names. For each domain name you specify, email is sent to the domain owner to request approval to issue the certificate. After receiving approval from the domain owner, the ACM Certificate is issued. For more information, see the <a href=\"http://docs.aws.amazon.com/acm/latest/userguide/\">AWS Certificate Manager User Guide</a>.</p>"]
                pub fn request_certificate(&self, input: &RequestCertificateRequest)  -> Result<RequestCertificateResponse, RequestCertificateError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.RequestCertificate");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RequestCertificateResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RequestCertificateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Resends the email that requests domain ownership validation. The domain owner or an authorized representative must approve the ACM Certificate before it can be issued. The certificate can be approved by clicking a link in the mail to navigate to the Amazon certificate approval website and then clicking <b>I Approve</b>. However, the validation email can be blocked by spam filters. Therefore, if you do not receive the original mail, you can request that the mail be resent within 72 hours of requesting the ACM Certificate. If more than 72 hours have elapsed since your original request or since your last attempt to resend validation mail, you must request a new certificate.</p>"]
                pub fn resend_validation_email(&self, input: &ResendValidationEmailRequest)  -> Result<(), ResendValidationEmailError> {
                    let mut request = SignedRequest::new("POST", "acm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "CertificateManager.ResendValidationEmail");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(())
                        }
                        _ => Err(ResendValidationEmailError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                
}

            #[cfg(test)]
            mod protocol_tests {
                
            }
            
