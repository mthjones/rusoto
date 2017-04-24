
                extern crate rusoto_importexport;
                extern crate rusoto_mock;
                extern crate rusoto;

                use rusoto_importexport::*;
                use rusoto_mock::*;
                use rusoto::Region as rusoto_region;

                
        #[test]
        fn test_parse_importexport_list_jobs() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/importexport-list-jobs.xml"));
            let client = ImportExportClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ListJobsInput::default();
            let result = client.list_jobs(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }