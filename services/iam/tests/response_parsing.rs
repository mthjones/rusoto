
                extern crate rusoto_iam;
                extern crate rusoto_mock;
                extern crate rusoto_core;

                use rusoto_iam::*;
                use rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                
        #[test]
        fn test_parse_iam_create_virtual_mfa_device() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-create-virtual-mfa-device.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateVirtualMFADeviceRequest::default();
            let result = client.create_virtual_mfa_device(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_get_account_summary() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-get-account-summary.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.get_account_summary();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_get_group() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-get-group.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetGroupRequest::default();
            let result = client.get_group(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_get_user_policy() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-get-user-policy.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetUserPolicyRequest::default();
            let result = client.get_user_policy(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_get_user() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-get-user.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetUserRequest::default();
            let result = client.get_user(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_access_keys() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-access-keys.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListAccessKeysRequest::default();
            let result = client.list_access_keys(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_account_aliases() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-account-aliases.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListAccountAliasesRequest::default();
            let result = client.list_account_aliases(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-groups.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListGroupsRequest::default();
            let result = client.list_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_instance_profiles() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-instance-profiles.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListInstanceProfilesRequest::default();
            let result = client.list_instance_profiles(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_mfa_devices() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-mfa-devices.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListMFADevicesRequest::default();
            let result = client.list_mfa_devices(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_roles() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-roles.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListRolesRequest::default();
            let result = client.list_roles(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_server_certificates() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-server-certificates.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListServerCertificatesRequest::default();
            let result = client.list_server_certificates(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_signing_certificates() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-signing-certificates.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListSigningCertificatesRequest::default();
            let result = client.list_signing_certificates(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_users() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-users.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListUsersRequest::default();
            let result = client.list_users(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_iam_list_virtual_mfa_devices() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/iam-list-virtual-mfa-devices.xml"));
            let client = IamClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListVirtualMFADevicesRequest::default();
            let result = client.list_virtual_mfa_devices(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }