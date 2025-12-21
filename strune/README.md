# Strune

A simple, directional knowledge structure library for building linked references and word books.

more about: https://project-starlivia.github.io/Strune/

## Features

- **Core Data Structure**: Generic `Node<T>` with label, description, dependencies, and customizable options
- **Markdown & JSON Loaders**: Parse knowledge nodes from Markdown or JSON files
- **Graph Operations**: Calculate reverse dependencies (dependents) and slug mappings
- **Static Site Generation**: Render knowledge graphs to HTML using Tera templates
- **Type-Safe**: Leverages Rust's type system with generic options support

## Installation

Add Strune to your `Cargo.toml`:

```toml
[dependencies]
strune = "0.1.0"
```

## Quick Start

Loading Nodes from Markdown
```rust
// define custom options
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MyOpts<T>
{
    #[serde(flatten)]
    pub base: T,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub dependents: Option<Vec<String>>,
}

impl_maybe_slug!(MyOpts);
impl_maybe_dependents!(MyOpts);

fn main() -> Result<()> {
    // Load nodes from Markdown file
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let nodes: Vec<Node<MyOpts<Value>>> = load_nodes_from_markdown(base_path.join("content/sample.md"))?;

    // Calculate dependents
    let nodes = fill_dependents(nodes);

    println!("nodes: {}", nodes.len());

    // Clear dist directory
    let dist_path = base_path.join("dist");
    if dist_path.exists() {
        fs::remove_dir_all(&dist_path)?;
    }
    fs::create_dir_all(&dist_path)?;

    // Render static site
    render(
        "strune/templates/**/*.html",
        &dist_path,
        "/projects/Strune/sample/dist/",
        nodes.as_slice(),
    )
    .map_err(|e| {
        eprintln!("render error: {:?}", e);
        e
    })?;

    // Copy public directory to dist/public
    let public_path = base_path.join("public");
    if public_path.exists() {
        let dist_public = dist_path.join("public");
        copy_dir_all(public_path, &dist_public)?;
        println!("Copied public directory to dist/public");
    }

    Ok(())
}
```

## Markdown Syntax

Strune uses an extended Markdown syntax for defining knowledge nodes:

```markdown
# NodeLabel
## description
A short explanation of this concept.

## dependencies
- ParentConcept1
- ParentConcept2

## options
Additional metadata as needed
```

All fields except the label (h1) are optional.

## Core Concepts

### Node Structure

```rust
pub struct Node<T = Value> {
    pub label: String,           // Node identifier
    pub description: String,     // Explanation text
    pub dependencies: Vec<String>, // Parent concepts
    pub options: T,              // Custom metadata
}
```

### Dependencies

Dependencies represent directional relationships. A node's dependencies are concepts that compose or contain it:

- `Unity` → depends on `GameEngine`
- `Blender` ↔ `FBX` (mutual dependencies)

## Sample

See the [sample directory](https://github.com/your-username/Strune/tree/main/sample) for complete usage examples.

https://project-starlivia.github.io/Strune/

## License

MIT License - see [LICENSE](../LICENSE) for details.
