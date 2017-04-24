
                extern crate rusoto_cloudwatch;
                extern crate rusoto_mock;
                extern crate rusoto_core;

                use rusoto_cloudwatch::*;
                use rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                
        #[test]
        fn test_parse_cloudwatch_describe_alarm_history() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudwatch-describe-alarm-history.xml"));
            let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeAlarmHistoryInput::default();
            let result = client.describe_alarm_history(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudwatch_describe_alarms() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudwatch-describe-alarms.xml"));
            let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeAlarmsInput::default();
            let result = client.describe_alarms(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudwatch_list_metrics() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudwatch-list-metrics.xml"));
            let client = CloudWatchClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListMetricsInput::default();
            let result = client.list_metrics(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }