use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct PackageLock {
    name: String,
    version: String,
    dependencies: Option<HashMap<String, DependencyInFile>>,
    packages: Option<HashMap<String, DependencyInFile>>,
}

#[derive(Debug, Deserialize)]
struct DependencyInFile {
    version: Option<String>,
    resolved: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimplifiedPackage {
    pub deps: Vec<SimplifiedDependency>,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimplifiedDependency {
    pub name: String,
    pub version: String,
}

pub fn extract_dependencies_from_package_lock(file_content: &str, string_to_match: &str) -> Result<HashMap<String, SimplifiedPackage>, Box<dyn std::error::Error>> {
    let package_data: PackageLock = serde_json::from_str(file_content)?;
    let version = package_data.version;
    let mut all_deps: Vec<(String, DependencyInFile)> = Vec::new();

    if let Some(deps) = package_data.dependencies {
        all_deps.extend(deps.into_iter());
    } else if let Some(packages) = package_data.packages {
        all_deps.extend(
            packages.into_iter()
                    .map(|(key, dep)| (key.trim_start_matches("node_modules/").to_string(), dep))
        );
    }
    let filtered_deps: Vec<SimplifiedDependency> = all_deps
        .into_iter()
        .filter_map(|(name, dep)| {
            if let Some(version) = dep.version {
                if dep.resolved.as_ref().map_or(false, |resolved| resolved.contains(string_to_match)) {
                    Some(SimplifiedDependency { name, version })
                } else {
                    None
                }
            } else {
                None
            }
    })
        .collect();
    
    let mut result = HashMap::new();
    result.insert(package_data.name, SimplifiedPackage { version, deps: filtered_deps });

    Ok(result)
}
