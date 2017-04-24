
                extern crate rusoto_sns;
                extern crate rusoto_mock;
                extern crate rusoto_core;

                use rusoto_sns::*;
                use rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                
        #[test]
        fn test_parse_sns_add_permission() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-add-permission.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AddPermissionInput::default();
            let result = client.add_permission(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sns_confirm_subscription() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-confirm-subscription.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ConfirmSubscriptionInput::default();
            let result = client.confirm_subscription(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sns_create_topic() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-create-topic.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateTopicInput::default();
            let result = client.create_topic(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sns_get_subscription_attributes() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-get-subscription-attributes.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetSubscriptionAttributesInput::default();
            let result = client.get_subscription_attributes(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sns_get_topic_attributes() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-get-topic-attributes.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetTopicAttributesInput::default();
            let result = client.get_topic_attributes(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sns_list_subscriptions_by_topic() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-list-subscriptions-by-topic.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListSubscriptionsByTopicInput::default();
            let result = client.list_subscriptions_by_topic(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sns_list_subscriptions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-list-subscriptions.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListSubscriptionsInput::default();
            let result = client.list_subscriptions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sns_list_topics() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-list-topics.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListTopicsInput::default();
            let result = client.list_topics(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sns_publish() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-publish.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = PublishInput::default();
            let result = client.publish(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sns_subscribe() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sns-subscribe.xml"));
            let client = SnsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = SubscribeInput::default();
            let result = client.subscribe(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }