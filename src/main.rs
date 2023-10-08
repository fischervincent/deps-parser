use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::collections::HashMap;
use deps_parser::{Output, extract_dependencies_from_package_lock};
use serde_json;

const DEPS_GRAPH_FILE: &str = "deps-graph.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <STRING_TO_MATCH>", args[0]);
        std::process::exit(1);
    }

    let string_to_match = &args[1];

    let paths = fs::read_dir("../")?;
    
    let mut aggregated_data: HashMap<String, Output> = HashMap::new();

    // Check if deps-graph.json already exists and read its content.
    if Path::new(DEPS_GRAPH_FILE).exists() {
        let mut content = String::new();
        File::open(DEPS_GRAPH_FILE)?.read_to_string(&mut content)?;
        aggregated_data = serde_json::from_str(&content)?;
    }

    for path in paths {
        let path = path?.path();
        if path.is_dir() {
            let package_lock_path = path.join("package-lock.json");
            if package_lock_path.exists() {
                let mut file_content = String::new();
                File::open(&package_lock_path)?.read_to_string(&mut file_content)?;
                                
                let extracted_data = extract_dependencies_from_package_lock(&file_content, &string_to_match)?;
                
                // Merge the extracted_data into aggregated_data.
                aggregated_data.extend(extracted_data);
            }
        }
    }

    // Write the aggregated_data back to deps-graph.json.
    let output = serde_json::to_string_pretty(&aggregated_data)?;
    File::create(DEPS_GRAPH_FILE)?.write_all(output.as_bytes())?;

    println!("Processed dependencies written to {}", DEPS_GRAPH_FILE);

    Ok(())
}
