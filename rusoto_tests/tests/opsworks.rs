#![cfg(feature = "opsworks")]

extern crate rusoto_core;
extern crate rusoto_opsworks;

use rusoto_opsworks::{OpsWorksClient, DescribeStacksRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_describe_stacks() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = OpsWorksClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeStacksRequest::default();

    client.describe_stacks(&request).unwrap();
}

#[test]
fn should_describe_my_user_profile() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = OpsWorksClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    client.describe_my_user_profile().unwrap();
}
