use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use serde_json::Value;

use strune_core::node::Node;
use render::ingwaz::render;
use operation::{impl_maybe_dependents, impl_maybe_slug};
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

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let nodes: Vec<Node<MyOpts<Value>>> = load_nodes_from_markdown("content/sample.md")?;

    let nodes = fill_dependents(nodes);

    for node in nodes.iter() {
        println!("{:?}", node.to_string());
    }
    println!("nodes: {}", nodes.len());

    // Clear dist directory
    let dist_path = Path::new("dist");
    if dist_path.exists() {
        fs::remove_dir_all(dist_path)?;
    }
    fs::create_dir_all(dist_path)?;

    render(
        "crates/tera-render/templates/**/*.html",
        "dist",
        "/Strune",
        nodes.as_slice(),
    )
    .map_err(|e| {
        eprintln!("render error: {:?}", e);
        e
    })?;

    // Copy public directory to dist/public
    let public_path = Path::new("public");
    if public_path.exists() {
        let dist_public = dist_path.join("public");
        copy_dir_all(public_path, &dist_public)?;
        println!("Copied public directory to dist/public");
    }

    Ok(())
}