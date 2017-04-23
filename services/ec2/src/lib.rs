//! Amazon EC2 Container Service
//!
//! If you're using the service, you're probably looking for [Ec2Client](struct.Ec2Client.html).

extern crate rusoto_core;
extern crate hyper;
extern crate md5;
extern crate xml;

use rusoto_core::*;

include!(concat!(env!("OUT_DIR"), "/ec2.rs"));
