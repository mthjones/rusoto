
                extern crate rusoto_ec2;
                extern crate rusoto_mock;
                extern crate rusoto_core;

                use rusoto_ec2::*;
                use rusoto_mock::*;
                use rusoto_core::Region as rusoto_region;

                
        #[test]
        fn test_parse_ec_2_allocate_address() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-allocate-address.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AllocateAddressRequest::default();
            let result = client.allocate_address(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_assign_private_ip_addresses() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-assign-private-ip-addresses.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AssignPrivateIpAddressesRequest::default();
            let result = client.assign_private_ip_addresses(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_associate_address() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-associate-address.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AssociateAddressRequest::default();
            let result = client.associate_address(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_associate_route_table() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-associate-route-table.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AssociateRouteTableRequest::default();
            let result = client.associate_route_table(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_attach_volume() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-attach-volume.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AttachVolumeRequest::default();
            let result = client.attach_volume(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_attach_vpn_gateway() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-attach-vpn-gateway.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = AttachVpnGatewayRequest::default();
            let result = client.attach_vpn_gateway(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_bundle_instance() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-bundle-instance.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = BundleInstanceRequest::default();
            let result = client.bundle_instance(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_cancel_bundle_task() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-cancel-bundle-task.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CancelBundleTaskRequest::default();
            let result = client.cancel_bundle_task(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_cancel_reserved_instances_listing() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-cancel-reserved-instances-listing.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CancelReservedInstancesListingRequest::default();
            let result = client.cancel_reserved_instances_listing(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_cancel_spot_instance_requests() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-cancel-spot-instance-requests.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CancelSpotInstanceRequestsRequest::default();
            let result = client.cancel_spot_instance_requests(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_confirm_product_instance() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-confirm-product-instance.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ConfirmProductInstanceRequest::default();
            let result = client.confirm_product_instance(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_copy_snapshot() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-copy-snapshot.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CopySnapshotRequest::default();
            let result = client.copy_snapshot(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_customer_gateway() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-customer-gateway.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateCustomerGatewayRequest::default();
            let result = client.create_customer_gateway(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_dhcp_options() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-dhcp-options.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateDhcpOptionsRequest::default();
            let result = client.create_dhcp_options(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_instance_export_task() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-instance-export-task.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateInstanceExportTaskRequest::default();
            let result = client.create_instance_export_task(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_key_pair() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-key-pair.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateKeyPairRequest::default();
            let result = client.create_key_pair(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_network_acl() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-network-acl.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateNetworkAclRequest::default();
            let result = client.create_network_acl(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_network_interface() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-network-interface.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateNetworkInterfaceRequest::default();
            let result = client.create_network_interface(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_reserved_instances_listing() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-reserved-instances-listing.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateReservedInstancesListingRequest::default();
            let result = client.create_reserved_instances_listing(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_route_table() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-route-table.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateRouteTableRequest::default();
            let result = client.create_route_table(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_snapshot() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-snapshot.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateSnapshotRequest::default();
            let result = client.create_snapshot(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_spot_datafeed_subscription() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-spot-datafeed-subscription.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateSpotDatafeedSubscriptionRequest::default();
            let result = client.create_spot_datafeed_subscription(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_subnet() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-subnet.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateSubnetRequest::default();
            let result = client.create_subnet(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_volume() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-volume.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateVolumeRequest::default();
            let result = client.create_volume(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_vpc() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-vpc.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateVpcRequest::default();
            let result = client.create_vpc(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_create_vpn_gateway() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-create-vpn-gateway.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = CreateVpnGatewayRequest::default();
            let result = client.create_vpn_gateway(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_delete_internet_gateway() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-delete-internet-gateway.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DeleteInternetGatewayRequest::default();
            let result = client.delete_internet_gateway(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_account_attributes() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-account-attributes.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeAccountAttributesRequest::default();
            let result = client.describe_account_attributes(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_addresses() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-addresses.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeAddressesRequest::default();
            let result = client.describe_addresses(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_availability_zones() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-availability-zones.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeAvailabilityZonesRequest::default();
            let result = client.describe_availability_zones(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_bundle_tasks() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-bundle-tasks.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeBundleTasksRequest::default();
            let result = client.describe_bundle_tasks(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_customer_gateways() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-customer-gateways.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeCustomerGatewaysRequest::default();
            let result = client.describe_customer_gateways(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_dhcp_options() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-dhcp-options.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeDhcpOptionsRequest::default();
            let result = client.describe_dhcp_options(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_export_tasks() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-export-tasks.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeExportTasksRequest::default();
            let result = client.describe_export_tasks(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_instance_attribute() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-instance-attribute.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeInstanceAttributeRequest::default();
            let result = client.describe_instance_attribute(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_instance_status() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-instance-status.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeInstanceStatusRequest::default();
            let result = client.describe_instance_status(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-instances.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeInstancesRequest::default();
            let result = client.describe_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_internet_gateways() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-internet-gateways.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeInternetGatewaysRequest::default();
            let result = client.describe_internet_gateways(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_key_pairs() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-key-pairs.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeKeyPairsRequest::default();
            let result = client.describe_key_pairs(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_network_acls() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-network-acls.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeNetworkAclsRequest::default();
            let result = client.describe_network_acls(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_network_interfaces() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-network-interfaces.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeNetworkInterfacesRequest::default();
            let result = client.describe_network_interfaces(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_placement_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-placement-groups.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribePlacementGroupsRequest::default();
            let result = client.describe_placement_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_regions() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-regions.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeRegionsRequest::default();
            let result = client.describe_regions(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_reserved_instances_offerings() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-reserved-instances-offerings.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeReservedInstancesOfferingsRequest::default();
            let result = client.describe_reserved_instances_offerings(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_reserved_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-reserved-instances.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeReservedInstancesRequest::default();
            let result = client.describe_reserved_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_route_tables() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-route-tables.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeRouteTablesRequest::default();
            let result = client.describe_route_tables(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_security_groups() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-security-groups.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeSecurityGroupsRequest::default();
            let result = client.describe_security_groups(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_snapshots() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-snapshots.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeSnapshotsRequest::default();
            let result = client.describe_snapshots(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_spot_instance_requests() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-spot-instance-requests.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeSpotInstanceRequestsRequest::default();
            let result = client.describe_spot_instance_requests(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_spot_price_history() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-spot-price-history.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeSpotPriceHistoryRequest::default();
            let result = client.describe_spot_price_history(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_subnets() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-subnets.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeSubnetsRequest::default();
            let result = client.describe_subnets(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_tags() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-tags.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeTagsRequest::default();
            let result = client.describe_tags(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_volume_status() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-volume-status.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeVolumeStatusRequest::default();
            let result = client.describe_volume_status(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_volumes() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-volumes.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeVolumesRequest::default();
            let result = client.describe_volumes(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_vpcs() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-vpcs.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeVpcsRequest::default();
            let result = client.describe_vpcs(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_vpn_connections() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-vpn-connections.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeVpnConnectionsRequest::default();
            let result = client.describe_vpn_connections(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_describe_vpn_gateways() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-describe-vpn-gateways.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DescribeVpnGatewaysRequest::default();
            let result = client.describe_vpn_gateways(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_detach_network_interface() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-detach-network-interface.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DetachNetworkInterfaceRequest::default();
            let result = client.detach_network_interface(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_detach_volume() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-detach-volume.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = DetachVolumeRequest::default();
            let result = client.detach_volume(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_get_password_data() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-get-password-data.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = GetPasswordDataRequest::default();
            let result = client.get_password_data(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_import_instance() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-import-instance.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ImportInstanceRequest::default();
            let result = client.import_instance(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_import_key_pair() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-import-key-pair.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ImportKeyPairRequest::default();
            let result = client.import_key_pair(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_import_volume() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-import-volume.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ImportVolumeRequest::default();
            let result = client.import_volume(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_modify_snapshot_attribute() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-modify-snapshot-attribute.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ModifySnapshotAttributeRequest::default();
            let result = client.modify_snapshot_attribute(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_monitor_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-monitor-instances.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = MonitorInstancesRequest::default();
            let result = client.monitor_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_register_image() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-register-image.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = RegisterImageRequest::default();
            let result = client.register_image(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_replace_network_acl_association() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-replace-network-acl-association.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = ReplaceNetworkAclAssociationRequest::default();
            let result = client.replace_network_acl_association(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_request_spot_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-request-spot-instances.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = RequestSpotInstancesRequest::default();
            let result = client.request_spot_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_run_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-run-instances.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = RunInstancesRequest::default();
            let result = client.run_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_start_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-start-instances.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = StartInstancesRequest::default();
            let result = client.start_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_stop_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-stop-instances.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = StopInstancesRequest::default();
            let result = client.stop_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }


        #[test]
        fn test_parse_ec_2_unmonitor_instances() {
            let mock = MockRequestDispatcher::with_status(200)
                .with_body(include_str!("responses/ec2-unmonitor-instances.xml"));
            let client = Ec2Client::new(mock, MockCredentialsProvider, rusoto_region::UsEast1);
            let request = UnmonitorInstancesRequest::default();
            let result = client.unmonitor_instances(&request);
            assert!(result.is_ok(), "parse error: {:?}", result);
        }