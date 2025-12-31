use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use serde_json::Value;

use strune::{Node, render, impl_maybe_dependents, impl_maybe_slug, load_nodes_from_markdown, fill_dependents, DEFAULT_CONFIG};

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
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let nodes: Vec<Node<MyOpts<Value>>> = load_nodes_from_markdown(base_path.join("content/sample.md"))?;

    let nodes = fill_dependents(nodes);

    println!("nodes: {}", nodes.len());


    // Determine config path based on environment
    let config_path = if std::env::var("STRUNE_ENV").ok().as_deref() == Some("dev") {
        base_path.join("config/development.yml")
    } else {
        PathBuf::from(DEFAULT_CONFIG)
    };
    println!("Using config path: {}", config_path.display());

    let dist_path = base_path.join("dist");
    let public_path = base_path.join("public");

    render(
        "strune/templates/**/*.html",
        &dist_path,
        &public_path,
        &config_path,
        nodes.as_slice(),
    )
    .map_err(|e| {
        eprintln!("render error: {:?}", e);
        e
    })?;

    Ok(())
}