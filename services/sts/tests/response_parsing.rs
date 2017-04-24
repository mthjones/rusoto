
                extern crate rusoto_sts;
                extern crate rusoto_mock;
                extern crate rusoto_core;

                use rusoto_sts::*;
                use rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                
        #[test]
        fn test_parse_sts_get_session_token() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sts-get-session-token.xml"));
            let client = StsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetSessionTokenRequest::default();
            let result = client.get_session_token(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }