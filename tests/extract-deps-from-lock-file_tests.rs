use std::fs;

use deps_parser::extract_dependencies_from_package_lock;

#[test]
fn given_dependencies_at_root_then_use_dependencies() {
    let content = fs::read_to_string("tests/fixtures/sample_package_lock.json")
        .expect("Failed to read the fixture file");

    let string_to_match = "your_private_registry_url_here.com";
    let result = extract_dependencies_from_package_lock(&content, &string_to_match)
        .expect("Failed to extract dependencies");

    assert!(result.contains_key("some_service_name")); 

    let dependencies = &result["some_service_name"].deps;
    assert_eq!(dependencies.len(), 2);
    assert!(dependencies.iter().any(|dep| dep.name == "private-lib1" && dep.version == "1.0.0"));
    assert!(dependencies.iter().any(|dep| dep.name == "private-lib2" && dep.version == "1.2.1"));
}

#[test]
// for npm 7+ and the use of npm workspace for example
fn given_no_dependencies_at_root_then_use_packages_instead() {
    let content = fs::read_to_string("tests/fixtures/sample_package_lock_no_root_dependencies.json")
        .expect("Failed to read the fixture file");
    let result = extract_dependencies_from_package_lock(&content, "https://private-registry/")
        .expect("Failed to extract dependencies");
    
    assert!(result.contains_key("sample-project")); 
    print!("{:?}", result);
    let dependencies = &result["sample-project"].deps;
    
    assert_eq!(dependencies.len(), 2);
    assert!(dependencies.iter().any(|dep| dep.name == "private-lib1" && dep.version == "1.0.0"));
    assert!(dependencies.iter().any(|dep| dep.name == "private-lib2" && dep.version == "2.0.0"));
}