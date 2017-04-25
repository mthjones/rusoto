#![cfg(feature = "devicefarm")]

extern crate rusoto_core;
extern crate rusoto_datapipeline;

use rusoto_devicefarm::{DeviceFarmClient, ListDevicesRequest};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
pub fn should_list_devices() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DeviceFarmClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = ListDevicesRequest::default();

    client.list_devices(&request).unwrap();
}
