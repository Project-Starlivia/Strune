use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use serde_json::Value;

use strune_core::node::Node;
use render::render_simple;
use operation::{impl_maybe_dependents, impl_maybe_slug};
use loader::json::load_nodes_from_json;
use loader::markdown::load_nodes_from_markdown;
use operation::dependents::fill_dependents;

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
    let nodes: Vec<Node<MyOpts<Value>>> = load_nodes_from_markdown("content/sample.md")?;


    let nodes = fill_dependents(nodes);

    for node in nodes.iter() {
        println!("{:?}", node.to_string());
    }
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