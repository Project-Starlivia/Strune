use std::path::Path;
use core::node::Node;
use serde::Deserialize;
use serde_json::{Value, Map};

#[derive(Debug, Deserialize)]
struct RawNode {
    label: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    pub options: Value
}

impl RawNode {
    pub fn to_node<T>(self) -> Result<Node<T>, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let opt_values = match self.options {
            Value::Null => Value::Object(Map::new()),
            other => other,
        };
        let opts: T = serde_json::from_value(opt_values)?;

        Ok(Node {
            label: self.label.clone(),
            description: self.description.clone(),
            dependencies: self.dependencies.clone(),
            options: opts,
        })
    }
}
pub fn load_nodes_from_file<T>(file: impl AsRef<Path>) -> anyhow::Result<Vec<Node<T>>>
where
    T: serde::de::DeserializeOwned,
{
    let content = std::fs::read_to_string(file)?;
    let raws: Vec<RawNode> = serde_json::from_str(&content)?;

    raws.into_iter()
        .map(|raw| raw.to_node::<T>().map_err(Into::into))
        .collect()
}