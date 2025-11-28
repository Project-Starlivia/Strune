use std::fs;
use std::path::Path;
use anyhow::Result;
use loader::load_nodes_from_file;
use render::render_node_page;
use serde::Deserialize;

use core::node::Node;

#[derive(Debug, Deserialize)]
struct MyOptions {
    #[serde(rename = "updatedAt")]
    updated_at: String,
    #[serde(default)]
    tags: Vec<String>,
}
fn main() -> Result<()> {
    let nodes = load_nodes_from_file::<MyOptions>("content/sample.json")?;
    println!("nodes: {}", nodes.len());

    fs::create_dir_all("dist")?;

    for node in &nodes {
        let html = render_node_page(node)?;
        let filename = format!("{}.html", node.label);
        let path = Path::new("dist").join(filename);
        fs::write(&path, html)?;
        println!("-> {:?}", path);
    }

    Ok(())
}