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

extern crate chrono;
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;
extern crate reqwest;
extern crate retry;
extern crate ring;
extern crate rustc_serialize;
extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate url;
#[cfg(feature="xml")]
extern crate xml;

pub mod credential;
pub mod param;
pub mod region;
pub mod request;
#[cfg(feature="xml")]
pub mod xmlerror;
#[cfg(feature="xml")]
pub mod xmlutil;
pub mod serialization;
#[macro_use]
pub mod signature;

pub use region::{ParseRegionError, Region};
pub use credential::{AwsCredentials, ChainProvider, ContainerProvider, CredentialsError,
                            EnvironmentProvider, InstanceMetadataProvider, ProfileProvider,
                            ProvideAwsCredentials, DefaultCredentialsProvider,
                            DefaultCredentialsProviderSync, claims,
                            AutoRefreshingProviderSync, AutoRefreshingProvider,
                            BaseAutoRefreshingProvider};
pub use request::{DispatchSignedRequest, HttpResponse, HttpDispatchError, TlsError};
pub use signature::SignedRequest;
pub use request::default_tls_client;
pub use region::default_region;
