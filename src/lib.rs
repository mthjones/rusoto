#![crate_name = "rusoto"]
#![crate_type = "lib"]
#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity, used_underscore_binding, ptr_arg, suspicious_else_formatting))]
#![allow(dead_code)]
#![cfg_attr(not(feature = "unstable"), deny(warnings))]

//! Rusoto is an [AWS](https://aws.amazon.com/) SDK for Rust.
//! A high level overview is available in `README.md` at https://github.com/rusoto/rusoto.
//!
//! # Example
//!
//! The following code shows a simple example of using Rusoto's DynamoDB API to
//! list the names of all tables in a database.
//!
//! ```rust,ignore
//! use std::default::Default;
//!
//! use rusoto::{DefaultCredentialsProvider, Region};
//! use rusoto::dynamodb::{DynamoDbClient, ListTablesInput};
//!
//! let provider = DefaultCredentialsProvider::new().unwrap();
//! let client = DynamoDbClient::new(provider, Region::UsEast1);
//! let list_tables_input: ListTablesInput = Default::default();
//!
//! match client.list_tables(&list_tables_input) {
//!     Ok(output) => {
//!         match output.table_names {
//!             Some(table_name_list) => {
//!                 println!("Tables in database:");
//!
//!                 for table_name in table_name_list {
//!                     println!("{}", table_name);
//!                 }
//!             },
//!             None => println!("No tables in database!"),
//!         }
//!     },
//!     Err(error) => {
//!         println!("Error: {:?}", error);
//!     },
//! }

extern crate rusoto_core;

pub use rusoto_core::*;

#[cfg(feature = "acm")]
pub extern crate rusoto_acm;
#[cfg(feature = "autoscaling")]
pub extern crate rusoto_autoscaling;
#[allow(unused_imports)]
#[cfg(feature = "cloudformation")]
pub extern crate rusoto_cloudformation;
#[cfg(feature = "cloudfront")]
pub extern crate rusoto_cloudfront;
#[cfg(feature = "cloudhsm")]
pub extern crate rusoto_cloudhsm;
#[cfg(feature = "cloudsearch")]
pub extern crate rusoto_cloudsearch;
#[cfg(feature = "cloudtrail")]
pub extern crate rusoto_cloudtrail;
#[cfg(feature = "cloudwatch")]
pub extern crate rusoto_cloudwatch;
#[cfg(feature = "codecommit")]
pub extern crate rusoto_codecommit;
#[cfg(feature = "codedeploy")]
pub extern crate rusoto_codedeploy;
#[cfg(feature = "codepipeline")]
pub extern crate rusoto_codepipeline;
#[cfg(feature = "cognito-identity")]
pub extern crate rusoto_cognito_identity;
#[cfg(feature = "config")]
pub extern crate rusoto_config;
#[cfg(feature = "datapipeline")]
pub extern crate rusoto_datapipeline;
#[cfg(feature = "devicefarm")]
pub extern crate rusoto_devicefarm;
#[cfg(feature = "directconnect")]
pub extern crate rusoto_directconnect;
#[cfg(feature = "ds")]
pub extern crate rusoto_ds;
#[cfg(feature = "dynamodb")]
pub extern crate rusoto_dynamodb;
#[cfg(feature = "dynamodbstreams")]
pub extern crate rusoto_dynamodbstreams;
#[cfg(feature = "ec2")]
pub extern crate rusoto_ec2;
#[cfg(feature = "ecr")]
pub extern crate rusoto_ecr;
#[cfg(feature = "ecs")]
pub extern crate rusoto_ecs;
#[cfg(feature = "emr")]
pub extern crate rusoto_emr;
#[cfg(feature = "elasticache")]
pub extern crate rusoto_elasticache;
#[cfg(feature = "elasticbeanstalk")]
pub extern crate rusoto_elasticbeanstalk;
#[cfg(feature = "elastictranscoder")]
pub extern crate rusoto_elastictranscoder;
#[cfg(feature = "elb")]
pub extern crate rusoto_elb;
#[cfg(feature = "elbv2")]
pub extern crate rusoto_elbv2;
#[cfg(feature = "events")]
pub extern crate rusoto_events;
#[cfg(feature = "firehose")]
pub extern crate rusoto_firehose;
#[cfg(feature = "iam")]
pub extern crate rusoto_iam;
#[cfg(feature = "importexport")]
pub extern crate rusoto_importexport;
#[cfg(feature = "inspector")]
pub extern crate rusoto_inspector;
#[cfg(feature = "iot")]
pub extern crate rusoto_iot;
#[cfg(feature = "kinesis")]
pub extern crate rusoto_kinesis;
#[cfg(feature = "kms")]
pub extern crate rusoto_kms;
#[cfg(feature = "lambda")]
pub extern crate rusoto_lambda;
#[cfg(feature = "logs")]
pub extern crate rusoto_logs;
#[cfg(feature = "machinelearning")]
pub extern crate rusoto_machinelearning;
#[cfg(feature = "marketplacecommerceanalytics")]
pub extern crate rusoto_marketplacecommerceanalytics;
#[cfg(feature = "opsworks")]
pub extern crate rusoto_opsworks;
#[cfg(feature = "redshift")]
pub extern crate rusoto_redshift;
#[cfg(feature = "rds")]
pub extern crate rusoto_rds;
#[cfg(feature = "route53")]
pub extern crate rusoto_route53;
#[cfg(feature = "route53domains")]
pub extern crate rusoto_route53domains;
#[cfg(feature = "s3")]
pub extern crate rusoto_s3;
#[cfg(feature = "sdb")]
pub extern crate rusoto_sdb;
#[cfg(feature = "ses")]
pub extern crate rusoto_ses;
#[cfg(feature = "sns")]
pub extern crate rusoto_sns;
#[cfg(feature = "sqs")]
pub extern crate rusoto_sqs;
#[cfg(feature = "ssm")]
pub extern crate rusoto_ssm;
#[cfg(feature = "storagegateway")]
pub extern crate rusoto_storagegateway;
#[cfg(feature = "sts")]
pub extern crate rusoto_sts;
#[cfg(feature = "swf")]
pub extern crate rusoto_swf;
#[cfg(feature = "waf")]
pub extern crate rusoto_waf;
#[cfg(feature = "workspaces")]
pub extern crate rusoto_workspaces;

/*
#[cfg(feature = "gamelift")]
pub extern crate rusoto_gamelift;
#[cfg(feature = "support")]
pub extern crate rusoto_support;
*/
