use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const STRING_TO_MATCH: &str = "STRING_TO_MATCH_IN_RESOLVED";
const OUTPUT_FILE: &str = "deps-graph.json";

#[derive(Debug, Deserialize)]
struct PackageLock {
    dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    version: String,
    resolved: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Output {
    deps: Vec<OutputDependency>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OutputDependency {
    name: String,
    version: String,
}

fn process_package_lock(path: &std::path::Path) -> Result<HashMap<String, Output>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let package_lock: PackageLock = serde_json::from_str(&data)?;

    let filtered_deps: Vec<OutputDependency> = package_lock.dependencies.into_iter()
        .filter_map(|(name, dep)| {
            if dep.resolved.as_ref().map_or(false, |resolved| resolved.contains(STRING_TO_MATCH)) {
                Some(OutputDependency { name, version: dep.version })
            } else {
                None
            }
        })
        .collect();

    let output = Output { deps: filtered_deps };
    
    let service_name = path.parent().unwrap().file_name().unwrap().to_string_lossy().to_string();

    let mut result = HashMap::new();
    result.insert(service_name, output);

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir("./")?;  // Read the current directory

    let mut existing_data = match fs::read_to_string("deps-graph.json") {
        Ok(content) => serde_json::from_str::<HashMap<String, Output>>(&content)?,
        Err(_) => HashMap::new(),
    };

    for path in paths {
        let path = path?.path();
        if path.is_dir() {
            let package_lock_path = path.join("package-lock.json");
            if package_lock_path.exists() {
                let new_data = process_package_lock(&package_lock_path)?;
                existing_data.extend(new_data);
            }
        }
    }

    fs::write(OUTPUT_FILE, serde_json::to_string_pretty(&existing_data)?)?;
    Ok(())
}