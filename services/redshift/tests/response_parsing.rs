
                extern crate rusoto_redshift;
                extern crate rusoto_mock;
                extern crate rusoto_core;

                use rusoto_redshift::*;
                use rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                
        #[test]
        fn test_parse_redshift_authorize_cluster_security_group_ingress() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-authorize-cluster-security-group-ingress.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AuthorizeClusterSecurityGroupIngressMessage::default();
            let result = client.authorize_cluster_security_group_ingress(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_copy_cluster_snapshot() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-copy-cluster-snapshot.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CopyClusterSnapshotMessage::default();
            let result = client.copy_cluster_snapshot(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_create_cluster_parameter_group() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-create-cluster-parameter-group.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateClusterParameterGroupMessage::default();
            let result = client.create_cluster_parameter_group(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_create_cluster_security_group() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-create-cluster-security-group.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateClusterSecurityGroupMessage::default();
            let result = client.create_cluster_security_group(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_create_cluster_snapshot() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-create-cluster-snapshot.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateClusterSnapshotMessage::default();
            let result = client.create_cluster_snapshot(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_create_cluster_subnet_group() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-create-cluster-subnet-group.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateClusterSubnetGroupMessage::default();
            let result = client.create_cluster_subnet_group(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_create_cluster() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-create-cluster.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateClusterMessage::default();
            let result = client.create_cluster(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_delete_cluster_parameter_group() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-delete-cluster-parameter-group.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DeleteClusterParameterGroupMessage::default();
            let result = client.delete_cluster_parameter_group(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_delete_cluster_snapshot() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-delete-cluster-snapshot.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DeleteClusterSnapshotMessage::default();
            let result = client.delete_cluster_snapshot(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_delete_cluster() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-delete-cluster.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DeleteClusterMessage::default();
            let result = client.delete_cluster(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_cluster_parameter_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-cluster-parameter-groups.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeClusterParameterGroupsMessage::default();
            let result = client.describe_cluster_parameter_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_cluster_parameters() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-cluster-parameters.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeClusterParametersMessage::default();
            let result = client.describe_cluster_parameters(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_cluster_security_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-cluster-security-groups.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeClusterSecurityGroupsMessage::default();
            let result = client.describe_cluster_security_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_cluster_snapshots() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-cluster-snapshots.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeClusterSnapshotsMessage::default();
            let result = client.describe_cluster_snapshots(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_cluster_subnet_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-cluster-subnet-groups.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeClusterSubnetGroupsMessage::default();
            let result = client.describe_cluster_subnet_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_cluster_versions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-cluster-versions.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeClusterVersionsMessage::default();
            let result = client.describe_cluster_versions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_clusters() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-clusters.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeClustersMessage::default();
            let result = client.describe_clusters(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_events() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-events.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeEventsMessage::default();
            let result = client.describe_events(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_orderable_cluster_options() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-orderable-cluster-options.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeOrderableClusterOptionsMessage::default();
            let result = client.describe_orderable_cluster_options(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_reserved_node_offerings() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-reserved-node-offerings.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeReservedNodeOfferingsMessage::default();
            let result = client.describe_reserved_node_offerings(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_reserved_nodes() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-reserved-nodes.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeReservedNodesMessage::default();
            let result = client.describe_reserved_nodes(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_describe_resize() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-describe-resize.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeResizeMessage::default();
            let result = client.describe_resize(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_modify_cluster_parameter_group() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-modify-cluster-parameter-group.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ModifyClusterParameterGroupMessage::default();
            let result = client.modify_cluster_parameter_group(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_purchase_reserved_node_offering() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-purchase-reserved-node-offering.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = PurchaseReservedNodeOfferingMessage::default();
            let result = client.purchase_reserved_node_offering(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_reboot_cluster() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-reboot-cluster.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = RebootClusterMessage::default();
            let result = client.reboot_cluster(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_reset_cluster_parameter_group() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-reset-cluster-parameter-group.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ResetClusterParameterGroupMessage::default();
            let result = client.reset_cluster_parameter_group(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_restore_from_cluster_snapshot() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-restore-from-cluster-snapshot.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = RestoreFromClusterSnapshotMessage::default();
            let result = client.restore_from_cluster_snapshot(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_redshift_revoke_cluster_security_group_ingress() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/redshift-revoke-cluster-security-group-ingress.xml"));
            let client = RedshiftClient::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = RevokeClusterSecurityGroupIngressMessage::default();
            let result = client.revoke_cluster_security_group_ingress(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }