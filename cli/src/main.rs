use std::fs;
use std::path::Path;
use anyhow::Result;
use loader::load_nodes_from_file;
use render::render_node_page;

fn main() -> Result<()> {
    let nodes = load_nodes_from_file("content/sample.json")?;
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