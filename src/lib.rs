mod extract_links_and_nodes;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct PackageLock {
    name: String,
    version: String,
    dependencies: Option<HashMap<String, Dependency>>,
    packages: Option<HashMap<String, Dependency>>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    version: Option<String>,
    resolved: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    pub deps: Vec<OutputDependency>,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputDependency {
    pub name: String,
    pub version: String,
}

pub fn extract_dependencies_from_package_lock(file_content: &str, string_to_match: &str) -> Result<HashMap<String, Output>, Box<dyn std::error::Error>> {
    let package_data: PackageLock = serde_json::from_str(file_content)?;
    let version = package_data.version;
    let mut all_deps: Vec<(String, Dependency)> = Vec::new();

    if let Some(deps) = package_data.dependencies {
        all_deps.extend(deps.into_iter());
    } else if let Some(packages) = package_data.packages {
        all_deps.extend(
            packages.into_iter()
                    .map(|(key, dep)| (key.trim_start_matches("node_modules/").to_string(), dep))
        );
    }
    let filtered_deps: Vec<OutputDependency> = all_deps
        .into_iter()
        .filter_map(|(name, dep)| {
            if let Some(version) = dep.version {
                if dep.resolved.as_ref().map_or(false, |resolved| resolved.contains(string_to_match)) {
                    Some(OutputDependency { name, version })
                } else {
                    None
                }
            } else {
                None
            }
    })
        .collect();
    
    let mut result = HashMap::new();
    result.insert(package_data.name, Output { version, deps: filtered_deps });

    Ok(result)
}

pub use extract_links_and_nodes::{Link, Node, extract_links_and_nodes};
