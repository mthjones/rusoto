
                extern crate rusoto_s3;
                extern crate rusoto_mock;
                extern crate rusoto_core;

                use rusoto_s3::*;
                use rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                
        #[test]
        fn test_parse_s3_get_bucket_acl() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/s3-get-bucket-acl.xml"));
            let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetBucketAclRequest::default();
            let result = client.get_bucket_acl(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_s3_get_bucket_location() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/s3-get-bucket-location.xml"));
            let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetBucketLocationRequest::default();
            let result = client.get_bucket_location(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_s3_get_bucket_logging() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/s3-get-bucket-logging.xml"));
            let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetBucketLoggingRequest::default();
            let result = client.get_bucket_logging(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_s3_get_bucket_policy() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/s3-get-bucket-policy.xml"));
            let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetBucketPolicyRequest::default();
            let result = client.get_bucket_policy(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_s3_list_buckets() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/s3-list-buckets.xml"));
            let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            
            let result = client.list_buckets();
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_s3_list_multipart_uploads() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/s3-list-multipart-uploads.xml"));
            let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListMultipartUploadsRequest::default();
            let result = client.list_multipart_uploads(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_s3_list_object_versions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/s3-list-object-versions.xml"));
            let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListObjectVersionsRequest::default();
            let result = client.list_object_versions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_s3_list_objects() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/s3-list-objects.xml"));
            let client = S3Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListObjectsRequest::default();
            let result = client.list_objects(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }