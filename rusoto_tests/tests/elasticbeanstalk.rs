#![cfg(feature = "elasticbeanstalk")]

extern crate rusoto_core;
extern crate rusoto_elasticbeanstalk;

use rusoto_elasticbeanstalk::{ElasticBeanstalkClient, DescribeApplicationsMessage};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_describe_applications() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        ElasticBeanstalkClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeApplicationsMessage::default();

    let result = client.describe_applications(&request).unwrap();
    println!("{:#?}", result);
}
