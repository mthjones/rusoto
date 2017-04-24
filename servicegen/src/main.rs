#[macro_use]
extern crate clap;
extern crate rayon;
extern crate rusoto_codegen;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod cargo;

use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write, BufWriter};
use std::path::Path;

use rayon::prelude::*;

use clap::{Arg, ArgGroup, App};

use rusoto_codegen::botocore::Service;

#[derive(Deserialize)]
struct ServiceConfig {
    pub version: String
}

fn get_dependencies(service: &Service, core_version: &str) -> BTreeMap<String, cargo::Dependency> {
    let mut dependencies = BTreeMap::new();

    dependencies.insert("hyper".to_owned(), cargo::Dependency::Simple("0.10.0".into()));

    match &service.metadata.protocol[..] {
        "json" => {
            dependencies.insert("serde".to_owned(), cargo::Dependency::Simple("0.9.5".into()));
            dependencies.insert("serde_derive".to_owned(), cargo::Dependency::Simple("0.9.5".into()));
            dependencies.insert("serde_json".to_owned(), cargo::Dependency::Simple("0.9.4".into()));
            dependencies.insert("rusoto".to_owned(), cargo::Dependency::Extended {
                path: Some("../../core".into()),
                version: Some(core_version.to_owned()),
                optional: None,
                default_features: None,
                features: None
            });
        },
        "query" | "ec2" => {
            dependencies.insert("xml-rs".to_owned(), cargo::Dependency::Simple("0.3".into()));
            dependencies.insert("rusoto".to_owned(), cargo::Dependency::Extended {
                path: Some("../../core".into()),
                version: Some(core_version.to_owned()),
                optional: None,
                default_features: None,
                features: Some(vec!["xml".into()])
            });
        },
        "rest-json" => {
            dependencies.insert("log".to_owned(), cargo::Dependency::Simple("0.3.6".into()));
            dependencies.insert("serde".to_owned(), cargo::Dependency::Simple("0.9.5".into()));
            dependencies.insert("serde_derive".to_owned(), cargo::Dependency::Simple("0.9.5".into()));
            dependencies.insert("serde_json".to_owned(), cargo::Dependency::Simple("0.9.4".into()));
            dependencies.insert("rusoto".to_owned(), cargo::Dependency::Extended {
                path: Some("../../core".into()),
                version: Some(core_version.to_owned()),
                optional: None,
                default_features: None,
                features: None
            });
        },
        "rest-xml" => {
            dependencies.insert("xml-rs".to_owned(), cargo::Dependency::Simple("0.3".into()));
            dependencies.insert("rusoto".to_owned(), cargo::Dependency::Extended {
                path: Some("../../core".into()),
                version: Some(core_version.to_owned()),
                optional: None,
                default_features: None,
                features: Some(vec!["xml".into()])
            });
        },
        protocol => panic!("Unknown protocol {}", protocol),
    }

    match &service.metadata.endpoint_prefix[..] {
        "s3" => {
            dependencies.insert("rustc-serialize".to_owned(), cargo::Dependency::Simple("0.3.19".into()));
            dependencies.insert("md5".to_owned(), cargo::Dependency::Simple("0.3.2".into()));
        }
        _ => {}
    };

    dependencies
}

fn main() {
    let matches = App::new("Rusoto Service Crate Generator")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("services_list")
            .long("services")
            .use_delimiter(true)
            .takes_value(true))
        .arg(Arg::with_name("services_config")
            .long("config")
            .short("c")
            .takes_value(true))
        .group(ArgGroup::with_name("services")
            .args(&["services_list", "services_config"])
            .required(true))
        .arg(Arg::with_name("out_dir")
            .long("outdir")
            .short("o")
            .takes_value(true)
            .required(true))
        .get_matches();
    
    let core_manifest_raw = OpenOptions::new()
        .read(true)
        .open("../core/Cargo.toml")
        .and_then(|mut f| {
            let mut buf = String::new();
            f.read_to_string(&mut buf).map(|_| buf)
        })
        .expect("Unable to read core crate's Cargo.toml");
    
    let core_manifest: cargo::Manifest = toml::from_str(&core_manifest_raw).expect("Unable to parse core crate's Cargo.toml");

    let version = &core_manifest.package.version;

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

    let service_versions: Vec<(String, String)> = {
        if let Some(services) = matches.values_of("services_list") {
            services.map(|s| {
                let mut service_parts = s.splitn(2, "@");
                let name = service_parts.next().expect(&format!("Invalid service value {}. Must be in format name@version.", s)).to_owned();
                let version = service_parts.next().expect(&format!("Invalid service value {}. Must be in format name@version.", s)).to_owned();
                (name, version)
            }).collect()
        } else if let Some(services_config) = matches.value_of("services_config") {
            let contents = File::open(services_config).and_then(|mut f| {
                let mut contents = String::new();
                f.read_to_string(&mut contents).map(|_| contents)
            }).expect("Unable to read services configuration file.");

            let parsed: BTreeMap<String, ServiceConfig> = serde_json::from_str(&contents).expect("Unable to parse services configuration file.");
            parsed.iter().map(|(k, v)| (k.clone(), v.version.clone())).collect()
        } else {
            unreachable!("Arg parsing failed. Did not have services config or services list.");
        }
    };

    service_versions.par_iter().for_each(|&(ref name, ref service_version)| {
        let service = Service::load(name, service_version);

        if let Err(ref e) = service {
            println!("Failed to load service {}: {}", name, e);
            return;
        }
        
        let service = service.as_ref().unwrap();

        let crate_dir = out_dir.join(&name);

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
                description: Some(format!("AWS SDK for Rust - {} @ {}", &service.metadata.service_full_name, &service.metadata.api_version)),
                documentation: Some("http://rusoto.github.io/rusoto/rusoto/index.html".into()),
                keywords: Some(vec!["AWS".into(), "Amazon".into(), name.clone(), service.metadata.service_full_name.clone(), service.metadata.endpoint_prefix.clone()]),
                license: Some("MIT".into()),
                name: format!("rusoto_{}", &name),
                readme: None,
                repository: Some("https://github.com/rusoto/rusoto".into()),
                version: version.clone(),
                homepage: Some("https://www.rusoto.org/".into()),
                ..cargo::Metadata::default()
            },
            dependencies: get_dependencies(&service, &version),
            dev_dependencies: vec![
                ("rusoto_mock".to_owned(), cargo::Dependency::Extended {
                    path: Some("../../mock".into()),
                    version: None,
                    optional: None,
                    default_features: None,
                    features: None
                })
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
    });
}
