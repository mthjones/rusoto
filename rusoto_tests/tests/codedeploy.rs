#![cfg(feature = "codedeploy")]

extern crate rusoto_core;
extern crate rusoto_codedeploy;

use rusoto_codedeploy::{CodeDeployClient, ListApplicationsInput};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_list_applications() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeDeployClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListApplicationsInput::default();

    client.list_applications(&request).unwrap();
}
