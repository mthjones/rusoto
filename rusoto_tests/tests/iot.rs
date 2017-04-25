#![cfg(feature = "iot")]

extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_iot;

use rusoto_iot::{IotClient, ListThingsRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_list_things() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = IotClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListThingsRequest::default();

    client.list_things(&request).unwrap();
}
