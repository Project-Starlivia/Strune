use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node<T = Value> {
    pub label: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub options: T,
}

impl Node<Value> {
    pub fn format_options<T>(&self) -> Result<Node<T>, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        let opts: T = serde_json::from_value(self.options.clone())?;

        Ok(Node {
            label: self.label.clone(),
            description: self.description.clone(),
            dependencies: self.dependencies.clone(),
            options: opts,
        })
    }
}

impl<T> fmt::Display for Node<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let options = serde_json::to_string_pretty(&self.options)
            .unwrap_or_else(|_| "\"<failed to serialize options>\"".to_string());

        write!(
            f,
            "Node {{ label: {}, description: {}, dependencies: {:?}, options: {} }}",
            self.label, self.description, self.dependencies, options
        )
    }
}