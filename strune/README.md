# Strune

A simple, directional knowledge structure library for building linked references and word books.

## Overview

Strune provides a minimal yet powerful data structure for representing knowledge nodes with dependencies and custom metadata. It's designed to help you build knowledge graphs, word books, and static documentation sites with ease.

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

### Loading Nodes from Markdown

```rust
use strune::{Node, load_nodes_from_markdown};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Use default Value type
let nodes: Vec<Node<Value>> = load_nodes_from_markdown("content/sample.md")?;

// Or use custom options type
#[derive(Serialize, Deserialize)]
struct MyOptions {
    slug: Option<String>,
    tags: Vec<String>,
}

let nodes: Vec<Node<MyOptions>> = load_nodes_from_markdown("content/sample.md")?;
```

### Calculating Reverse Dependencies

```rust
use strune::{fill_dependents, impl_maybe_dependents};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Options {
    dependents: Option<Vec<String>>,
}

impl_maybe_dependents!(Options);

let nodes = load_nodes_from_markdown("content/sample.md")?;
let nodes_with_dependents = fill_dependents(nodes);
```

### Rendering to Static HTML

```rust
use strune::{render, fill_dependents, impl_maybe_dependents, impl_maybe_slug};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Options {
    slug: Option<String>,
    dependents: Option<Vec<String>>,
}

impl_maybe_slug!(Options);
impl_maybe_dependents!(Options);

let nodes = load_nodes_from_markdown("content/sample.md")?;
let nodes = fill_dependents(nodes);

render(
    "templates/**/*",
    "dist",
    "",
    &nodes
)?;
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

### Traits

- `MaybeDependents`: Opt-in trait for nodes with reverse dependency tracking
- `MaybeSlug`: Opt-in trait for nodes with custom URL slugs
- `HasDependents` / `HasSlug`: Required field variants

## Examples

See the [examples directory](https://github.com/your-username/Strune/tree/main/examples) for complete usage examples.

## Documentation

Full API documentation is available at [docs.rs/strune](https://docs.rs/strune).

## License

MIT License - see [LICENSE](../LICENSE) for details.

## Links

- [Repository](https://github.com/your-username/Strune)
- [Issues](https://github.com/your-username/Strune/issues)
- [Changelog](https://github.com/your-username/Strune/blob/main/CHANGELOG.md)
