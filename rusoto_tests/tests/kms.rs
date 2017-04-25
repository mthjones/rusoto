#![cfg(feature = "kms")]

extern crate rusoto_core;
extern crate rusoto_kms;

use rusoto_kms::{KmsClient, ListKeysRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_list_keys() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KmsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListKeysRequest::default();

    client.list_keys(&request).unwrap();
}
