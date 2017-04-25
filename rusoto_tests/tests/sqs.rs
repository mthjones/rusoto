#![cfg(feature = "sqs")]

extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_sqs::{SqsClient, ListQueuesRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn list_queues() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let sqs = SqsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let request = ListQueuesRequest { ..Default::default() };

    let result = sqs.list_queues(&request).unwrap();
    println!("{:#?}", result);
}
