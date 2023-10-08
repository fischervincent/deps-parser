# WIP ⚠️
# deps-parser

Generates a dependency graph from package-lock.json files across subdirectories.

## Usage

```
cargo run -- -p /path/to/start/analyzing/from -s https://private-registry/
```

Parameters:

-p or --path: The path where your packages are. It will look for package-lock.json files in every subdirectories. Defaults to the current directory.
-f or --filter: The string pattern to match private packages. It will filter out dependencies that does not have a resolved property that contains this filter. This argument is required.
