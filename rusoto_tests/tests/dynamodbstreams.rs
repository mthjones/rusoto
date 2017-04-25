#![cfg(feature = "dynamodbstreams")]

extern crate rusoto_core;
extern crate rusoto_dynamodbstreams;

use rusoto_dynamodbstreams::{DynamoDbStreamsClient, ListStreamsInput};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_list_streams() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        DynamoDbStreamsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListStreamsInput::default();

    client.list_streams(&request).unwrap();
}
