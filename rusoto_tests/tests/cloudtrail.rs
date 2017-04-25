#![cfg(feature = "cloudtrail")]

extern crate rusoto_core;
extern crate rusoto_cloudtrail;

use rusoto_cloudtrail::{CloudTrailClient, DescribeTrailsRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_describe_trails() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudTrailClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeTrailsRequest::default();

    client.describe_trails(&request).unwrap();
}
