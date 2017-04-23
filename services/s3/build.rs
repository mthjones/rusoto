extern crate rusoto_codegen;

use std::env;
use std::path::Path;

use rusoto_codegen::{Service, generate, generate_user_agent_vars};

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir).to_owned();

    generate(Service::new("s3", "2006-03-01"), &out_path);

    generate_user_agent_vars(&out_path);
}
