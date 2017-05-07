
//! Amazon EC2 Container Service
//!
//! If you're using the service, you're probably looking for [EcsClient](struct.EcsClient.html) and [Ecs](trait.Ecs.html).

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
            