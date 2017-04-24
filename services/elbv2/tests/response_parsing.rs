
                extern crate rusoto_elbv2;
                extern crate rusoto_mock;
                extern crate rusoto;

                use rusoto_elbv2::*;
                use rusoto_mock::*;
                use rusoto::Region as rusoto_region;

                
        #[test]
        fn test_parse_elb_describe_load_balancers() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elb-describe-load-balancers.xml"));
            let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeLoadBalancersInput::default();
            let result = client.describe_load_balancers(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }