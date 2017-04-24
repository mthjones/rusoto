
                extern crate rusoto_autoscaling;
                extern crate rusoto_mock;
                extern crate rusoto;

                use rusoto_autoscaling::*;
                use rusoto_mock::*;
                use rusoto::Region as rusoto_region;

                
        #[test]
        fn test_parse_autoscaling_describe_adjustment_types() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-adjustment-types.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.describe_adjustment_types();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_auto_scaling_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-auto-scaling-groups.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AutoScalingGroupNamesType::default();
            let result = client.describe_auto_scaling_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_auto_scaling_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-auto-scaling-instances.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeAutoScalingInstancesType::default();
            let result = client.describe_auto_scaling_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_auto_scaling_notification_types() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-auto-scaling-notification-types.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.describe_auto_scaling_notification_types();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_launch_configurations() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-launch-configurations.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = LaunchConfigurationNamesType::default();
            let result = client.describe_launch_configurations(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_metric_collection_types() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-metric-collection-types.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.describe_metric_collection_types();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_notification_configurations() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-notification-configurations.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeNotificationConfigurationsType::default();
            let result = client.describe_notification_configurations(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_policies() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-policies.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribePoliciesType::default();
            let result = client.describe_policies(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_scaling_activities() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-scaling-activities.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeScalingActivitiesType::default();
            let result = client.describe_scaling_activities(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_scaling_process_types() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-scaling-process-types.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.describe_scaling_process_types();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_scheduled_actions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-scheduled-actions.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeScheduledActionsType::default();
            let result = client.describe_scheduled_actions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_tags() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-tags.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeTagsType::default();
            let result = client.describe_tags(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_autoscaling_describe_termination_policy_types() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/autoscaling-describe-termination-policy-types.xml"));
            let client = AutoscalingClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.describe_termination_policy_types();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }