
                extern crate rusoto_cloudfront;
                extern crate rusoto_mock;
                extern crate rusoto;

                use rusoto_cloudfront::*;
                use rusoto_mock::*;
                use rusoto::Region as rusoto_region;

                
        #[test]
        fn test_parse_cloudfront_get_cloud_front_origin_access_identity() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudfront-get-cloud-front-origin-access-identity.xml"));
            let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetCloudFrontOriginAccessIdentityRequest::default();
            let result = client.get_cloud_front_origin_access_identity(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudfront_get_distribution() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudfront-get-distribution.xml"));
            let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetDistributionRequest::default();
            let result = client.get_distribution(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudfront_get_invalidation() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudfront-get-invalidation.xml"));
            let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetInvalidationRequest::default();
            let result = client.get_invalidation(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudfront_get_streaming_distribution() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudfront-get-streaming-distribution.xml"));
            let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetStreamingDistributionRequest::default();
            let result = client.get_streaming_distribution(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudfront_list_cloud_front_origin_access_identities() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudfront-list-cloud-front-origin-access-identities.xml"));
            let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListCloudFrontOriginAccessIdentitiesRequest::default();
            let result = client.list_cloud_front_origin_access_identities(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudfront_list_distributions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudfront-list-distributions.xml"));
            let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListDistributionsRequest::default();
            let result = client.list_distributions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudfront_list_invalidations() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudfront-list-invalidations.xml"));
            let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListInvalidationsRequest::default();
            let result = client.list_invalidations(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_cloudfront_list_streaming_distributions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/cloudfront-list-streaming-distributions.xml"));
            let client = CloudFrontClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListStreamingDistributionsRequest::default();
            let result = client.list_streaming_distributions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }