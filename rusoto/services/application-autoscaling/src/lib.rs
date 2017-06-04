
//! Application Auto Scaling
//!
//! If you're using the service, you're probably looking for [ApplicationAutoScalingClient](struct.ApplicationAutoScalingClient.html) and [ApplicationAutoScaling](trait.ApplicationAutoScaling.html).

extern crate hyper;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            