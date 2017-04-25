#![cfg(feature = "firehose")]

extern crate rusoto_core;
extern crate rusoto_firehose;

use rusoto_firehose::{KinesisFirehoseClient, ListDeliveryStreamsInput};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_list_delivery_streams() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        KinesisFirehoseClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListDeliveryStreamsInput::default();

    client.list_delivery_streams(&request).unwrap();
}
