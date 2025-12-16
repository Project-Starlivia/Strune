use serde::{Deserialize, Serialize};
use anyhow::Result;
use serde_json::Value;

use loader::load_nodes_from_file;
use render::render_simple;
use operation::{impl_maybe_dependents, impl_maybe_slug};
use core::node::Node;
use operation::dependents::fill_dependents;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct OptionsExt<T> {
    #[serde(flatten)]
    pub base: T,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub dependents: Option<Vec<String>>,
}

impl_maybe_slug!(OptionsExt);
impl_maybe_dependents!(OptionsExt);

fn main() -> Result<()> {
    let nodes: Vec<Node<OptionsExt<Value>>> = load_nodes_from_file("content/sample.json")?;
    let nodes = fill_dependents(nodes);

    println!("nodes: {}", nodes.len());

    render_simple(
        "crates/tera-render/templates/*.html",
        "node.html",
        "dist",
        nodes.as_slice())
    .map_err(|e| {
        eprintln!("render error: {:?}", e);
        e
    })?;

    Ok(())
}