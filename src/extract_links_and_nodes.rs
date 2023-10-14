use serde::{Serialize};
use std::collections::{HashMap};

use crate::SimplifiedPackage;

#[derive(Debug, Serialize, PartialEq)]
pub struct Link {
    pub source: String,
    pub target: String,
    pub version: String,  // Moved the version here
}

#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub no_package: Option<bool>
}

pub fn extract_links_and_nodes(aggregated_data: &std::collections::HashMap<String, SimplifiedPackage>)
    -> (Vec<Link>, Vec<Node>) 
{
    let mut links = Vec::new();
    let mut node_map: HashMap<String, Node> = HashMap::new();

    // Collect all unique nodes
    for (service_name, service) in aggregated_data.iter() {
        node_map.entry(service_name.clone())
            .or_insert_with(|| Node { 
                id: service_name.clone(), 
                name: service_name.clone(), 
                version: Some(service.version.clone()), 
                no_package: None
            });
    }

    for (service_name, service) in aggregated_data.iter() {
        for dep in &service.deps {
            links.push(Link {
                source: dep.name.clone(),
                target: service_name.clone(),
                version: dep.version.clone(), 
            });
            node_map.entry(dep.name.clone())
                .or_insert_with(|| Node {
                    id: dep.name.clone(),
                    name: dep.name.clone(),
                    version: None,
                    no_package: Some(true),
                });
        }
    }

    let nodes = node_map.into_iter().map(|(_, v)| v).collect();

    (links, nodes)
}
