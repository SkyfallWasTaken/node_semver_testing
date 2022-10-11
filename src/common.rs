use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Empty {}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Dependency {
    pub versions: HashMap<String, Empty>,
    pub dependencies: Option<HashMap<Empty, String>>,
    pub dev_dependencies: Option<HashMap<Empty, String>>,
}

#[derive(Deserialize, Serialize)]
pub struct PackageNames {
    pub count: u64,
    pub names: Vec<String>,
}
