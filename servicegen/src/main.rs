#[macro_use]
extern crate clap;
extern crate rusoto_codegen;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod cargo;

use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::path::Path;

use clap::{Arg, App};

use rusoto_codegen::botocore::Service;

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

    let version = matches.value_of("version").unwrap();
    
    let services: Vec<io::Result<Service>> = matches.values_of("services").unwrap().map(|s| {
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

        let crate_dir = out_dir.join(&service.metadata.endpoint_prefix);

        println!("Generating crate for {} @ {}...", service.metadata.service_full_name, service.metadata.api_version);

        fs::create_dir(&crate_dir).expect(&format!("Unable to create directory at {}", crate_dir.display()));

        let mut cargo_manifest = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(crate_dir.join("Cargo.toml"))
            .expect("Unable to write Cargo.toml");
        
        let manifest = cargo::Manifest {
            package: cargo::Metadata {
                authors: Some(vec![
                    "Anthony DiMarco <ocramida@gmail.com>".into(),
                    "Jimmy Cuadra <jimmy@jimmycuadra.com>".into(),
                    "Matthew Mayer <matthewkmayer@gmail.com>".into(),
                    "Nikita Pekin <contact@nikitapek.in>".into()
                ]),
                description: Some(format!("AWS SDK for Rust - {}", &service.metadata.service_full_name)),
                documentation: Some("http://rusoto.github.io/rusoto/rusoto/index.html".into()),
                keywords: Some(vec!["AWS".into(), "Amazon".into(), service.metadata.service_full_name.clone(), service.metadata.endpoint_prefix.clone()]),
                license: Some("MIT".into()),
                name: format!("rusoto_{}", &service.metadata.endpoint_prefix),
                readme: Some("README.md".into()),
                repository: Some("https://github.com/rusoto/rusoto".into()),
                version: version.into(),
                homepage: Some("https://www.rusoto.org/".into()),
                ..cargo::Metadata::default()
            },
            dependencies: vec![
                ("chrono".to_owned(), cargo::Dependency::Simple("0.2.21".into())),
                ("hyper".to_owned(), cargo::Dependency::Simple("0.10.0".into())),
                ("hyper-native-tls".to_owned(), cargo::Dependency::Simple("0.2.1".into())),
                ("lazy_static".to_owned(), cargo::Dependency::Simple("0.2.1".into())),
                ("log".to_owned(), cargo::Dependency::Simple("0.3.6".into())),
                ("md5".to_owned(), cargo::Dependency::Simple("0.3.2".into())),
                ("regex".to_owned(), cargo::Dependency::Simple("0.2.1".into())),
                ("ring".to_owned(), cargo::Dependency::Simple("0.7".into())),
                ("rustc-serialize".to_owned(), cargo::Dependency::Simple("0.3.19".into())),
                ("serde".to_owned(), cargo::Dependency::Simple("0.9.5".into())),
                ("serde_derive".to_owned(), cargo::Dependency::Simple("0.9.5".into())),
                ("serde_json".to_owned(), cargo::Dependency::Simple("0.9.4".into())),
                ("time".to_owned(), cargo::Dependency::Simple("0.1.35".into())),
                ("url".to_owned(), cargo::Dependency::Simple("1.2.0".into())),
                ("xml-rs".to_owned(), cargo::Dependency::Simple("0.3".into())),
                ("rusoto".to_owned(), cargo::Dependency::Extended {
                    path: Some("../..".into()),
                    version: None,
                    optional: None,
                    default_features: None,
                }),
                ("rusoto_credential".to_owned(), cargo::Dependency::Extended {
                    path: Some("../../credential".into()),
                    version: None,
                    optional: None,
                    default_features: None,
                }),
            ].into_iter().collect(),
            ..cargo::Manifest::default()
        };

        let extern_crates = manifest.dependencies.iter().map(|(k, _)| {
            if k == "xml-rs" {
                return "extern crate xml;".into();
            }
            let safe_name = k.replace("-", "_");
            let use_macro = k == "serde_derive" || k == "log" || k == "lazy_static";
            if use_macro {
                return format!("#[macro_use]\nextern crate {};", safe_name);
            }
            format!("extern crate {};", safe_name)
        }).collect::<Vec<String>>().join("\n");

        cargo_manifest.write_all(toml::to_string(&manifest).unwrap().as_bytes()).unwrap();

        let src_dir = crate_dir.join("src");

        fs::create_dir(&src_dir).expect(&format!("Unable to create directory at {}", src_dir.display()));

        let lib_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(src_dir.join("lib.rs"))
            .expect("Unable to write lib.rs");
        
        let mut writer = BufWriter::new(lib_file);

        writer.write_all(extern_crates.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();

        rusoto_codegen::generator::generate_source2(&service, &mut writer).unwrap();
    }
}
