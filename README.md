# WIP ⚠️
# deps-parser

This project is designed to generate a comprehensive dependency graph of packages that are from your registry, by parsing package-lock.json files spread across multiple subdirectories. It's especially beneficial for large-scale projects with interdependent packages, providing insights into which packages may need dependencies update.

Upon execution, It produces 2 files:
- **aggregated-packages.json**: the aggregate of your "package-lock.json" files, filtered to display package dependencies by registry, leveraging the resolved property.
- **links-and-nodes.json**: Contains the necessary links and nodes, enabling you to visually represent the dependency graph.

For a hands-on illustration, refer to the example folder.

## Usage

```
cargo run -- -p /path/to/start/analyzing/from -s https://private-registry/
```

Parameters:

**-p or --path**: Designates the root path containing your packages. The tool recursively searches for package-lock.json files in every subdirectory. If not specified, it defaults to the current directory.

**-f or --filter**: The string to filter dependencies, example: your registry. Dependencies without a resolved property containing this string will be filtered out. This parameter is mandatory.
