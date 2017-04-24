
                extern crate rusoto_ses;
                extern crate rusoto_mock;
                extern crate rusoto_core;

                use rusoto_ses::*;
                use rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                
        #[test]
        fn test_parse_ses_delete_identity() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-delete-identity.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DeleteIdentityRequest::default();
            let result = client.delete_identity(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_get_identity_dkim_attributes() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-get-identity-dkim-attributes.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetIdentityDkimAttributesRequest::default();
            let result = client.get_identity_dkim_attributes(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_get_identity_notification_attributes() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-get-identity-notification-attributes.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetIdentityNotificationAttributesRequest::default();
            let result = client.get_identity_notification_attributes(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_get_identity_verification_attributes() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-get-identity-verification-attributes.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetIdentityVerificationAttributesRequest::default();
            let result = client.get_identity_verification_attributes(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_get_send_quota() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-get-send-quota.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.get_send_quota();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_get_send_statistics() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-get-send-statistics.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.get_send_statistics();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_list_identities() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-list-identities.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListIdentitiesRequest::default();
            let result = client.list_identities(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_send_email() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-send-email.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = SendEmailRequest::default();
            let result = client.send_email(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_send_raw_email() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-send-raw-email.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = SendRawEmailRequest::default();
            let result = client.send_raw_email(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_set_identity_dkim_enabled() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-set-identity-dkim-enabled.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = SetIdentityDkimEnabledRequest::default();
            let result = client.set_identity_dkim_enabled(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_verify_domain_dkim() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-verify-domain-dkim.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = VerifyDomainDkimRequest::default();
            let result = client.verify_domain_dkim(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ses_verify_domain_identity() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ses-verify-domain-identity.xml"));
            let client = SesClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = VerifyDomainIdentityRequest::default();
            let result = client.verify_domain_identity(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }