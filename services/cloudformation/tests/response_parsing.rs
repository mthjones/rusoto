
                extern crate rusoto_cloudformation;
                extern crate rusoto_mock;
                extern crate rusoto;

                use rusoto_cloudformation::*;
                use rusoto_mock::*;
                use rusoto::Region as rusoto_region;

                
        #[test]
        fn test_parse_cloudformation_describe_stacks() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudformation-describe-stacks.xml"));
            let client = CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeStacksInput::default();
            let result = client.describe_stacks(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudformation_get_template() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudformation-get-template.xml"));
            let client = CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetTemplateInput::default();
            let result = client.get_template(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudformation_list_stacks() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudformation-list-stacks.xml"));
            let client = CloudFormationClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListStacksInput::default();
            let result = client.list_stacks(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }