# Strune
<p align="center">
  <img src="./logo.svg" alt="logo" width="120">
</p>

Strune is a **simple, directional knowledge structure**.
It is primarily intended to function as a word book or a lightweight linked reference, but how you use it is entirely up to you.

[日本語](./README_JA.md)
## Structure

The core data structure is defined in `strune_core/src/node.rs`:

```rust
pub struct Node<T = Value> {
    pub label: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub options: T,
}
```

- **label**: The name of the knowledge node. This is the primary identifier.
- **description**: A short explanation of the concept, typically within 200 characters.
- **dependencies**: A list of parent concepts from the perspective of this node. Contains labels (strings) of related nodes.
- **options**: A freely extensible field for user-defined metadata. Generic type `T` allows for type-safe custom fields (defaults to `serde_json::Value`).

Note that dependencies is not merely a relational link.
Interpretation is ultimately up to the user, but by default it represents elements that compose or contain this knowledge.

Examples:
- `Unity` would have `GameEngine` listed in its dependencies.
- `Blender` and `FBX` would list each other in their dependencies fields.

## Isn’t this ambiguous?
Yes. Knowledge rarely fits into clean parent–child relationships.
Strune intentionally embraces this ambiguity and aims to visualize loosely directional conceptual structures, even if they don’t map perfectly.

Its core strength lies in its simplicity and directionality, and the expressive power that emerges from them.
# Packages

This project is organized as a Rust workspace monorepo.

> ## ⚠️ Beta Notice
> The following components are still in beta and may undergo breaking API changes.

## strune_core

**Location**: `strune_core/`

The core data structure library for Strune. This package defines the fundamental `Node<T>` type that all other packages depend on.

**Features**:
- Generic `Node<T>` struct with support for type-safe custom options
- Serde-based JSON serialization and deserialization
- `format_options<T>()` method for converting between different option types
- Display trait implementation for debugging

**Dependencies**: `serde`, `serde_json`

## loader

**Location**: `crates/loader/`

A library for loading Strune nodes from various file formats into the core `Node` structure.

**Features**:
- **Markdown Parser** (`markdown.rs`) – Converts extended Markdown files into Strune nodes
  - Supports hierarchical heading structure (`#` = label, `##` = sections)
  - Configurable parsing with custom description, dependencies, and options handlers
- **JSON Loader** (`json.rs`) – Direct JSON file parsing

**Core Functions**:
- `load_nodes_from_markdown<T>(path)` – Load nodes from Markdown with default parsers
- `load_nodes_from_markdown_detail<T>()` – Load with custom parsing functions
- `trim_text()` – Text normalization helper
- `list_text_to_array()` – Converts Markdown lists to string arrays

### Markdown Syntax

Writing raw JSON is tedious, so Strune introduces a minimal Markdown-based syntax:

```markdown
# <label>
## description
<description>
## dependencies
- <dependence01>
- <dependence02>
or
- <dependence01>
- <dependence02>
## options
<options>
```

All fields other than label are optional.

**Dependencies**: `strune_core`, `regex`, `serde`, `serde_json`, `once_cell`, `thiserror`

## operation

**Location**: `crates/operation/`

Provides operations for analyzing and manipulating Strune node graphs. This package introduces trait-based extensibility for adding computed fields to nodes.

**Features**:
- **Dependents** (`dependents.rs`) – Reverse dependency calculation
  - `HasDependents` / `MaybeDependents` traits for accessing reverse dependencies
  - `fill_dependents()` function that computes which nodes depend on each node
  - Macro support for easy trait implementation
- **Slug** (`slug.rs`) – URL-friendly identifier generation
  - `HasSlug` / `MaybeSlug` traits for accessing slug fields
  - `label_slug_map()` creates a mapping from labels to slugs
  - Fallback to label if no custom slug is defined

**Core Functions**:
- `fill_dependents<T>(nodes)` – Populates reverse dependency information
- `label_slug_map<T>(nodes)` – Generates label-to-slug mapping

**Dependencies**: `strune_core`

## tera-render

**Location**: `crates/tera-render/`

A rendering engine that generates static HTML sites from Strune nodes using the [Tera](https://keats.github.io/tera/) template engine.

**Features**:
- Template-based HTML generation with Tera
- Automatic dependency and dependent link generation
- Slug-based URL routing
- Configurable branding and base paths

**Core Components**:
- `ingwaz.rs` – Main rendering logic
  - `render()` – Generates HTML files for all nodes
  - `render_node_page()` – Renders individual node pages
  - `RenderNode` – Helper struct for link generation

**Template Context Variables**:
- `current_node` – The node being rendered
- `dependencies` – List of parent nodes with links
- `dependents` – List of child nodes with links
- `base_path`, `brand_logo`, `brand_title` – Site configuration

**Dependencies**: `strune_core`, `operation`, `tera`, `serde`, `serde_json`

## cli

**Location**: `cli/`

Command-line interface for Strune. This is the main executable that ties all packages together.

**Features**:
- Load Markdown files containing Strune nodes
- Calculate reverse dependencies automatically
- Generate static HTML site with templating
- Copy public assets to output directory
- Clear and rebuild output directory

**Current Workflow**:
1. Load nodes from `content/sample.md`
2. Fill reverse dependencies using `operation::fill_dependents()`
3. Render all nodes to `dist/` directory using Tera templates
4. Copy `public/` assets to `dist/public/`

**Dependencies**: `strune_core`, `loader`, `operation`, `tera-render` (aliased as `render`), `serde`, `serde_json`, `anyhow`
