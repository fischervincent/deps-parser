use std::collections::HashMap;

use deps_parser::{Output, OutputDependency, extract_links_and_nodes, Node, Link};

#[test]
fn test_extract_links_and_nodes() {
    // Mock data
    let mut mock_data = HashMap::new();
    mock_data.insert("service1".to_string(), Output {
        version: "1.1.0".to_string(),
        deps: vec![
            OutputDependency { name: "private-lib1".to_string(), version: "1.0.0".to_string() },
            OutputDependency { name: "private-lib2".to_string(), version: "2.0.0".to_string() },
        ]
    });
    mock_data.insert("service2".to_string(), Output {
        version: "2.1.0".to_string(),
        deps: vec![
            OutputDependency { name: "private-lib2".to_string(), version: "1.0.0".to_string() }
        ]
    });
    mock_data.insert("private-lib1".to_string(), Output {
        version: "2.5.0".to_string(),
        deps: vec![
            OutputDependency { name: "private-lib2".to_string(), version: "2.0.0".to_string() },
        ]
    });

    let (links, nodes) = extract_links_and_nodes(&mock_data);
    print!("nodes: {:?}", nodes);
    print!("links: {:?}", links);

    // Test the links
    assert_eq!(links.len(), 4);
    assert!(links.contains(&Link { source: "private-lib1".to_string(), target: "service1".to_string(), version: "1.0.0".to_string() }));
    assert!(links.contains(&Link { source: "private-lib2".to_string(), target: "service1".to_string(), version: "2.0.0".to_string() }));
    assert!(links.contains(&Link { source: "private-lib2".to_string(), target: "service2".to_string(), version: "1.0.0".to_string() }));
    assert!(links.contains(&Link { source: "private-lib2".to_string(), target: "private-lib1".to_string(), version: "2.0.0".to_string() }));

    // Test the nodes
    assert_eq!(nodes.len(), 4);
    assert!(nodes.contains(&Node { id: "service1".to_string(), name: "service1".to_string(), version: Some("1.1.0".to_string()), no_package: None }));
    assert!(nodes.contains(&Node { id: "service2".to_string(), name: "service2".to_string(), version: Some("2.1.0".to_string()), no_package: None }));
    assert!(nodes.contains(&Node { id: "private-lib1".to_string(), name: "private-lib1".to_string(), version: Some("2.5.0".to_string()), no_package: None }));
    assert!(nodes.contains(&Node { id: "private-lib2".to_string(), name: "private-lib2".to_string(), version: None, no_package: Some(true) }));
}
