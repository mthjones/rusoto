
                extern crate rusoto_rds;
                extern crate rusoto_mock;
                extern crate rusoto;

                use rusoto_rds::*;
                use rusoto_mock::*;
                use rusoto::Region as rusoto_region;

                
        #[test]
        fn test_parse_rds_describe_db_engine_versions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-db-engine-versions.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeDBEngineVersionsMessage::default();
            let result = client.describe_db_engine_versions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_db_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-db-instances.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeDBInstancesMessage::default();
            let result = client.describe_db_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_db_parameter_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-db-parameter-groups.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeDBParameterGroupsMessage::default();
            let result = client.describe_db_parameter_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_db_security_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-db-security-groups.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeDBSecurityGroupsMessage::default();
            let result = client.describe_db_security_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_db_snapshots() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-db-snapshots.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeDBSnapshotsMessage::default();
            let result = client.describe_db_snapshots(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_db_subnet_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-db-subnet-groups.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeDBSubnetGroupsMessage::default();
            let result = client.describe_db_subnet_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_event_categories() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-event-categories.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeEventCategoriesMessage::default();
            let result = client.describe_event_categories(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_event_subscriptions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-event-subscriptions.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeEventSubscriptionsMessage::default();
            let result = client.describe_event_subscriptions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_events() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-events.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeEventsMessage::default();
            let result = client.describe_events(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_option_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-option-groups.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeOptionGroupsMessage::default();
            let result = client.describe_option_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_reserved_db_instances_offerings() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-reserved-db-instances-offerings.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeReservedDBInstancesOfferingsMessage::default();
            let result = client.describe_reserved_db_instances_offerings(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_rds_describe_reserved_db_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/rds-describe-reserved-db-instances.xml"));
            let client = RdsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeReservedDBInstancesMessage::default();
            let result = client.describe_reserved_db_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }