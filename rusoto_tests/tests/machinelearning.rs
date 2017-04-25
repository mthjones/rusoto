#![cfg(feature = "machinelearning")]

extern crate rusoto_core;
extern crate rusoto_machinelearning;

use rusoto_machinelearning::{MachineLearningClient, DescribeDataSourcesInput,
                              DescribeBatchPredictionsInput, DescribeEvaluationsInput};
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};

#[test]
fn should_describe_batch_predictions() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        MachineLearningClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeBatchPredictionsInput::default();

    client.describe_batch_predictions(&request).unwrap();
}
#[test]
fn should_describe_data_sources() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        MachineLearningClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeDataSourcesInput::default();

    client.describe_data_sources(&request).unwrap();
}
#[test]
fn should_describe_evaluations() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        MachineLearningClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let request = DescribeEvaluationsInput::default();

    client.describe_evaluations(&request).unwrap();
}
