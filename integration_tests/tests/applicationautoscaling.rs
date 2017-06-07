#![cfg(feature = "application-autoscaling")]

extern crate rusoto_core;
extern crate rusoto_application_autoscaling;

use rusoto_application_autoscaling::{ApplicationAutoScaling, ApplicationAutoScalingClient, DescribeScalingActivitiesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_scaling_activities() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ApplicationAutoScalingClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeScalingActivitiesRequest {
    	service_namespace: "appstream".to_owned(),
    	..Default::default()
    };

	let result = client.describe_scaling_activities(&request).unwrap();
	println!("{:#?}", result);
}

