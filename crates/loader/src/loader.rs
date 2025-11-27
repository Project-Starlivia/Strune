use std::path::Path;
use core::node::Node;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct RawNode {
    label: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    dependencies: Vec<String>,
}

pub fn load_nodes_from_file<P: AsRef<Path>>(file: P) -> anyhow::Result<Vec<Node>> {
    let path = file.as_ref();

    if path.extension().and_then(|s| s.to_str()) != Some("json") {
        anyhow::bail!("JSONファイルではありません: {:?}", path);
    }

    let content = std::fs::read_to_string(path)?;

    let raws: Vec<RawNode> = serde_json::from_str(&content)?;

    let nodes = raws
        .into_iter()
        .map(|raw| Node {
            label: raw.label,
            description: raw.description,
            dependencies: raw.dependencies,
        })
        .collect();

    Ok(nodes)
}