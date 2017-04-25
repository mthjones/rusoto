#![cfg(feature = "cloudhsm")]

extern crate rusoto_core;
extern crate rusoto_cloudhsm;

use rusoto_cloudhsm::{CloudHsmClient, ListHapgsRequest, ListHsmsRequest, ListLunaClientsRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_list_hapgs() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListHapgsRequest::default();

    client.list_hapgs(&request).unwrap();
}

#[test]
fn should_list_hsms() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListHsmsRequest::default();

    client.list_hsms(&request).unwrap();
}
#[test]
fn should_list_luna_clients() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListLunaClientsRequest::default();

    client.list_luna_clients(&request).unwrap();
}
