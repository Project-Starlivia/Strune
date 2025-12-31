// Core data structures
pub mod core;
pub use core::Node;

// File loaders
pub mod loader;
pub use loader::{
    load_nodes_from_markdown,
    load_nodes_from_markdown_detail,
    load_nodes_from_json,
    load_config_from_yaml,
    trim_text,
    list_text_to_array,
    raw_node_to_value,
};

// Graph operations
pub mod operation;
pub use operation::{
    HasDependents,
    MaybeDependents,
    fill_dependents,
    HasSlug,
    MaybeSlug,
    label_slug_map,
};

// Template rendering
pub mod render;
pub use render::{render, RenderNode, DEFAULT_CONFIG};
