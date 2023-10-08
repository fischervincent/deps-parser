use std::env;
use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const OUTPUT_FILE: &str = "deps-graph.json";

#[derive(Debug, Deserialize)]
struct PackageLock {
    dependencies: Option<HashMap<String, Dependency>>,
    packages: Option<HashMap<String, Dependency>>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    version: Option<String>,
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

fn process_package_lock(path: &std::path::Path, string_to_match: &str) -> Result<HashMap<String, Output>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let package_lock: Result<PackageLock, _> = serde_json::from_str(&data);

    match package_lock {
        Ok(lock) => {
            let mut all_deps = Vec::new();

            if let Some(deps) = lock.dependencies {
                all_deps.extend(deps.into_iter());
            }

            if let Some(packages) = lock.packages {
                // Extracting package name from the key instead
                all_deps.extend(packages.into_iter().map(|(name, dep)| (name, dep)));
            }

            let filtered_deps: Vec<OutputDependency> = all_deps.into_iter()
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

            let output = Output { deps: filtered_deps };

            let service_name = path.parent().unwrap().file_name().unwrap().to_string_lossy().to_string();
            let mut result = HashMap::new();
            result.insert(service_name, output);

            Ok(result)
        },
        Err(e) => {
            eprintln!("Error processing {}: {:?}", path.display(), e);
            Ok(HashMap::new()) // Return an empty map for this file
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <STRING_TO_MATCH>", args[0]);
        std::process::exit(1);
    }

    let string_to_match = &args[1];

    let paths = fs::read_dir("../")?;

    let mut existing_data = match fs::read_to_string("deps-graph.json") {
        Ok(content) => serde_json::from_str::<HashMap<String, Output>>(&content)?,
        Err(_) => HashMap::new(),
    };

    for path in paths {
        let path = path?.path();
        if path.is_dir() {
            let package_lock_path = path.join("package-lock.json");
            if package_lock_path.exists() {
                let new_data = process_package_lock(&package_lock_path, string_to_match)?;
                existing_data.extend(new_data);
            }
        }
    }

    fs::write(OUTPUT_FILE, serde_json::to_string_pretty(&existing_data)?)?;
    Ok(())
}