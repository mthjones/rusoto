use toml;

use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Manifest {
    pub package: Metadata,
    pub badges: Option<HashMap<String, Badge>>,
    #[serde(rename="build-dependencies")]
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub build_dependencies: HashMap<String, Dependency>,
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub dependencies: HashMap<String, Dependency>,
    #[serde(rename="dev-dependencies")]
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub dev_dependencies: HashMap<String, Dependency>,
    pub features: Option<HashMap<String, Vec<String>>>
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Metadata {
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<String>,
    pub name: String,
    pub readme: Option<String>,
    pub repository: Option<String>,
    pub version: String,
    pub homepage: Option<String>
}

#[derive(Debug, Clone, Serialize)]
pub struct Badge {
    pub repository: String,
    pub branch: String
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Extended {
        version: Option<String>,
        path: Option<String>,
        optional: Option<bool>,
        #[serde(rename="default-features")]
        default_features: Option<bool>
    }
}