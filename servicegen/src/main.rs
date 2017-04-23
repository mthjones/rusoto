#[macro_use]
extern crate clap;
extern crate inflector;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod botocore;
mod serialization;

use std::fs;
use std::io::Write;
use std::path::Path;

use clap::{Arg, App};
use botocore::{Service, LoadError};

fn main() {
    let matches = App::new("Rusoto Service Crate Generator")
                    .version(crate_version!())
                    .author(crate_authors!())
                    .about(crate_description!())
                    .arg(Arg::with_name("services")
                        .long("services")
                        .use_delimiter(true)
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("out_dir")
                        .long("outdir")
                        .short("o")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("version")
                        .long("version")
                        .short("v")
                        .takes_value(true)
                        .required(true))
                    .get_matches();

    let out_dir = Path::new(matches.value_of("out_dir").unwrap());

    if out_dir.exists() {
        if out_dir.is_dir() {
            println!("Output directory {} already exists.", out_dir.display());
        }
        if out_dir.is_file() {
            println!("Given output directory {} is a file.", out_dir.display());
        }
        return;
    }

    fs::create_dir(out_dir).expect("Unable to create output directory");
    
    let services: Vec<Result<Service, LoadError>> = matches.values_of("services").unwrap().map(|s| {
        let mut service_parts = s.splitn(2, "@");
        let name = service_parts.next().expect(&format!("Invalid service value {}. Must be in format name@version.", s));
        let version = service_parts.next().expect(&format!("Invalid service value {}. Must be in format name@version.", s));
        
        Service::load(name, version)
    }).collect();

    for service in services {
        if let Err(e) = service {
            println!("Failed to load service: {}", e);
            continue;
        }
        
        let service = service.unwrap();

        let crate_dir = out_dir.join(service.metadata.endpoint_prefix);

        fs::create_dir(&crate_dir).expect(&format!("Unable to create directory at {}", crate_dir.display()));

        println!("Loaded {} @ {}", service.metadata.service_full_name, service.metadata.api_version);
    }
}
