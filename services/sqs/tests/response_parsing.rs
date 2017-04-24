
                extern crate rusoto_sqs;
                extern crate rusoto_mock;
                extern crate rusoto;

                use rusoto_sqs::*;
                use rusoto_mock::*;
                use rusoto::Region as rusoto_region;

                
        #[test]
        fn test_parse_sqs_add_permission() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-add-permission.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AddPermissionRequest::default();
            let result = client.add_permission(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sqs_change_message_visibility_batch() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-change-message-visibility-batch.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ChangeMessageVisibilityBatchRequest::default();
            let result = client.change_message_visibility_batch(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sqs_create_queue() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-create-queue.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateQueueRequest::default();
            let result = client.create_queue(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sqs_delete_message_batch() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-delete-message-batch.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DeleteMessageBatchRequest::default();
            let result = client.delete_message_batch(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sqs_get_queue_attributes() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-get-queue-attributes.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetQueueAttributesRequest::default();
            let result = client.get_queue_attributes(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sqs_get_queue_url() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-get-queue-url.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetQueueUrlRequest::default();
            let result = client.get_queue_url(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sqs_list_queues() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-list-queues.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListQueuesRequest::default();
            let result = client.list_queues(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sqs_receive_message() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-receive-message.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ReceiveMessageRequest::default();
            let result = client.receive_message(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sqs_send_message_batch() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-send-message-batch.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = SendMessageBatchRequest::default();
            let result = client.send_message_batch(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_sqs_send_message() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/sqs-send-message.xml"));
            let client = SqsClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = SendMessageRequest::default();
            let result = client.send_message(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }