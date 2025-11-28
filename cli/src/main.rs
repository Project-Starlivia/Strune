use std::fs;
use std::path::Path;
use serde::Deserialize;
use anyhow::Result;

use loader::load_nodes_from_file;
use render::render_node_page;
use operation::{fill_dependents, NodeWithDependents};

fn main() -> Result<()> {
    let nodes: Vec<NodeWithDependents> = load_nodes_from_file("content/sample.json")?;
    println!("nodes: {}", nodes.len());
    let nodes = fill_dependents(nodes);
    println!("nodes: {}", nodes.len());
    fs::create_dir_all("dist")?;


    for node in &nodes {
        let html = render_node_page(node)?;
        let filename = format!("{}.html", node.label);
        let path = Path::new("dist").join(filename);
        fs::write(&path, html)?;
        println!("-> {:?}", path);
    }
/*
*/
    Ok(())
}