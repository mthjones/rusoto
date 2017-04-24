
                extern crate rusoto_elb;
                extern crate rusoto_mock;
                extern crate rusoto;

                use rusoto_elb::*;
                use rusoto_mock::*;
                use rusoto::Region as rusoto_region;

                
        #[test]
        fn test_parse_elb_describe_load_balancer_policies() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elb-describe-load-balancer-policies.xml"));
            let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeLoadBalancerPoliciesInput::default();
            let result = client.describe_load_balancer_policies(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elb_describe_load_balancer_policy_types() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elb-describe-load-balancer-policy-types.xml"));
            let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeLoadBalancerPolicyTypesInput::default();
            let result = client.describe_load_balancer_policy_types(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_elb_describe_load_balancers() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/elb-describe-load-balancers.xml"));
            let client = ElbClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeAccessPointsInput::default();
            let result = client.describe_load_balancers(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }