#![crate_name = "rusoto"]
#![crate_type = "lib"]
#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity, used_underscore_binding, ptr_arg, suspicious_else_formatting))]
#![allow(dead_code)]
// #![cfg_attr(not(feature = "unstable"), deny(warnings))]

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

extern crate chrono;
extern crate hyper;
extern crate rusoto_core;
#[cfg(test)]
extern crate rusoto_mock;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate xml;

pub use rusoto_core::*;

#[cfg(feature = "acm")]
pub mod acm;
#[cfg(feature = "autoscaling")]
pub mod autoscaling;
#[allow(unused_imports)]
#[cfg(feature = "cloudformation")]
pub mod cloudformation;
#[cfg(feature = "cloudfront")]
pub mod cloudfront;
#[cfg(feature = "cloudhsm")]
pub mod cloudhsm;
#[cfg(feature = "cloudsearch")]
pub mod cloudsearch;
#[cfg(feature = "cloudtrail")]
pub mod cloudtrail;
#[cfg(feature = "cloudwatch")]
pub mod cloudwatch;
#[cfg(feature = "codecommit")]
pub mod codecommit;
#[cfg(feature = "codedeploy")]
pub mod codedeploy;
#[cfg(feature = "codepipeline")]
pub mod codepipeline;
#[cfg(feature = "cognito-identity")]
pub mod cognitoidentity;
#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "datapipeline")]
pub mod datapipeline;
#[cfg(feature = "devicefarm")]
pub mod devicefarm;
#[cfg(feature = "directconnect")]
pub mod directconnect;
#[cfg(feature = "ds")]
pub mod ds;
#[cfg(feature = "dynamodb")]
pub mod dynamodb;
#[cfg(feature = "dynamodbstreams")]
pub mod dynamodbstreams;
#[cfg(feature = "ecr")]
pub mod ecr;
#[cfg(feature = "ecs")]
pub mod ecs;
#[cfg(feature = "emr")]
pub mod emr;
#[cfg(feature = "elasticache")]
pub mod elasticache;
#[cfg(feature = "elasticbeanstalk")]
pub mod elasticbeanstalk;
#[cfg(feature = "elastictranscoder")]
pub mod elastictranscoder;
#[cfg(feature = "elb")]
pub mod elb;
#[cfg(feature = "elbv2")]
pub mod elbv2;
#[cfg(feature = "events")]
pub mod events;
#[cfg(feature = "firehose")]
pub mod firehose;
#[cfg(feature = "iam")]
pub mod iam;
#[cfg(feature = "importexport")]
pub mod importexport;
#[cfg(feature = "inspector")]
pub mod inspector;
#[cfg(feature = "iot")]
pub mod iot;
#[cfg(feature = "kinesis")]
pub mod kinesis;
#[cfg(feature = "kms")]
pub mod kms;
#[cfg(feature = "lambda")]
pub mod lambda;
#[cfg(feature = "logs")]
pub mod logs;
#[cfg(feature = "machinelearning")]
pub mod machinelearning;
#[cfg(feature = "marketplacecommerceanalytics")]
pub mod marketplacecommerceanalytics;
#[cfg(feature = "opsworks")]
pub mod opsworks;
#[cfg(feature = "redshift")]
pub mod redshift;
#[cfg(feature = "rds")]
pub mod rds;
#[cfg(feature = "route53")]
pub mod route53;
#[cfg(feature = "route53domains")]
pub mod route53domains;
#[cfg(feature = "sdb")]
pub mod sdb;
#[cfg(feature = "ses")]
pub mod ses;
#[cfg(feature = "sns")]
pub mod sns;
#[cfg(feature = "sqs")]
pub mod sqs;
#[cfg(feature = "ssm")]
pub mod ssm;
#[cfg(feature = "storagegateway")]
pub mod storagegateway;
#[cfg(feature = "sts")]
pub mod sts;
#[cfg(feature = "swf")]
pub mod swf;
#[cfg(feature = "waf")]
pub mod waf;
#[cfg(feature = "workspaces")]
pub mod workspaces;

/*
#[cfg(feature = "gamelift")]
pub mod gamelift;
#[cfg(feature = "support")]
pub mod support;
*/
