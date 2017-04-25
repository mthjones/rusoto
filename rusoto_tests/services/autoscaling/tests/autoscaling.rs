extern crate rusoto_core;
extern crate rusoto_autoscaling;

use rusoto_autoscaling::{AutoscalingClient, AutoScalingGroupNamesType};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_describe_auto_scaling_groups() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        AutoscalingClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = AutoScalingGroupNamesType::default();

    let response = client.describe_auto_scaling_groups(&request).unwrap();
    println!("{:#?}", response);
}
