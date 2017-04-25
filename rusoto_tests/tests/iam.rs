#![cfg(feature = "iam")]

extern crate rusoto_core;
extern crate rusoto_iam;

use rusoto_iam::{IamClient, GetUserRequest, ListUsersRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn get_user() {
    let credentials = DefaultCredentialsProvider::new().unwrap();

    let iam = IamClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    // http://docs.aws.amazon.com/IAM/latest/APIReference/Welcome.html
    let request = GetUserRequest { ..Default::default() };
    iam.get_user(&request).unwrap();
}

#[test]
fn list_users() {
    let credentials = DefaultCredentialsProvider::new().unwrap();

    let iam = IamClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    // http://docs.aws.amazon.com/IAM/latest/APIReference/Welcome.html
    let request = ListUsersRequest { ..Default::default() };
    iam.list_users(&request).unwrap();
}
