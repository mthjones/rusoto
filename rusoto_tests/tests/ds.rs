#![cfg(feature = "ds")]

extern crate rusoto_core;
extern crate rusoto_ds;

use rusoto_ds::{DirectoryServiceClient, DescribeTrustsRequest, DescribeDirectoriesRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_describe_trusts() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        DirectoryServiceClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeTrustsRequest::default();

    client.describe_trusts(&request).unwrap();
}

#[test]
fn should_describe_directories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        DirectoryServiceClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeDirectoriesRequest::default();

    client.describe_directories(&request).unwrap();
}
