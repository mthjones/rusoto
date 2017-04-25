#![cfg(feature = "ses")]

extern crate rusoto_core;
extern crate rusoto_ses;

use rusoto_ses::SesClient;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_list_verified_email_addresses() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SesClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let result = client.list_verified_email_addresses().unwrap();
    println!("{:#?}", result);
}
