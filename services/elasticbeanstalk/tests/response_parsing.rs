
                extern crate rusoto_elasticbeanstalk;
                extern crate rusoto_mock;
                extern crate rusoto_core;

                use rusoto_elasticbeanstalk::*;
                use rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                
        #[test]
        fn test_parse_elasticbeanstalk_check_dns_availability() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-check-dns-availability.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CheckDNSAvailabilityMessage::default();
            let result = client.check_dns_availability(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_create_application_version() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-create-application-version.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateApplicationVersionMessage::default();
            let result = client.create_application_version(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_create_application() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-create-application.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateApplicationMessage::default();
            let result = client.create_application(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_create_configuration_template() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-create-configuration-template.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateConfigurationTemplateMessage::default();
            let result = client.create_configuration_template(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_create_environment() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-create-environment.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateEnvironmentMessage::default();
            let result = client.create_environment(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_create_storage_location() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-create-storage-location.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.create_storage_location();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_delete_application() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-delete-application.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DeleteApplicationMessage::default();
            let result = client.delete_application(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_describe_application_versions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-describe-application-versions.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeApplicationVersionsMessage::default();
            let result = client.describe_application_versions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_describe_applications() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-describe-applications.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeApplicationsMessage::default();
            let result = client.describe_applications(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_describe_configuration_options() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-describe-configuration-options.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeConfigurationOptionsMessage::default();
            let result = client.describe_configuration_options(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_describe_environments() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-describe-environments.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeEnvironmentsMessage::default();
            let result = client.describe_environments(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_describe_events() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-describe-events.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeEventsMessage::default();
            let result = client.describe_events(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_list_available_solution_stacks() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-list-available-solution-stacks.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.list_available_solution_stacks();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_retrieve_environment_info() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-retrieve-environment-info.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = RetrieveEnvironmentInfoMessage::default();
            let result = client.retrieve_environment_info(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_terminate_environment() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-terminate-environment.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = TerminateEnvironmentMessage::default();
            let result = client.terminate_environment(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_update_application_version() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-update-application-version.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = UpdateApplicationVersionMessage::default();
            let result = client.update_application_version(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elasticbeanstalk_update_application() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elasticbeanstalk-update-application.xml"));
            let client = ElasticBeanstalkClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = UpdateApplicationMessage::default();
            let result = client.update_application(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }