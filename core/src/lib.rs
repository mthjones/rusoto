#![crate_name = "rusoto_core"]
#![crate_type = "lib"]
#![cfg_attr(feature = "unstable", feature(proc_macro))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity, used_underscore_binding, ptr_arg, suspicious_else_formatting))]
#![allow(dead_code)]
// #![cfg_attr(not(feature = "unstable"), deny(warnings))]

//! Rusoto is an [AWS](https://aws.amazon.com/) SDK for Rust.
//! A high level overview is available in `README.md` at https://github.com/rusoto/rusoto.

extern crate chrono;
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate md5;
extern crate regex;
extern crate ring;
extern crate rusoto_credential;
extern crate rustc_serialize;
extern crate serde;
extern crate time;
extern crate url;
extern crate xml;

pub use region::{ParseRegionError, Region};
pub use rusoto_credential::{AwsCredentials, ChainProvider, ContainerProvider, CredentialsError,
                            EnvironmentProvider, InstanceMetadataProvider, ProfileProvider,
                            ProvideAwsCredentials, DefaultCredentialsProvider,
                            DefaultCredentialsProviderSync, claims,
                            AutoRefreshingProviderSync, AutoRefreshingProvider,
                            BaseAutoRefreshingProvider};
pub use request::{DispatchSignedRequest, HttpResponse, HttpDispatchError, TlsError};
pub use signature::SignedRequest;
pub use request::default_tls_client;
pub use region::default_region;

pub mod param;
pub mod region;
pub mod request;
pub mod xmlerror;
pub mod xmlutil;
pub mod serialization;
#[macro_use]
pub mod signature;