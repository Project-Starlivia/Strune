use std::collections::HashMap;
use std::path::Path;
use regex::Regex;
use once_cell::sync::Lazy;
use serde_json::{Map, Value};
use thiserror::Error;

use strune_core::node::Node;

#[derive(Debug, Clone, Default)]
pub struct RawNode {
    title: String,
    level: usize,
    text: String,
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid data: {0}")]
    InvalidData(String),
}

static HEADING_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(#{1,6})\s+(.*)$").unwrap());

pub fn load_nodes_from_markdown_detail<T, Desc, Deps, Opt>(path: impl AsRef<Path>, parse_description: Desc, parse_dependencies: Deps, parse_options: Opt) -> Result<Vec<Node<T>>, LoadError>
where
    T: serde::de::DeserializeOwned,
    Desc: Fn(&str) -> String,
    Deps: Fn(&str) -> Vec<String>,
    Opt: Fn(&[RawNode]) -> Result<T, serde_json::Error>,
{
    let content = std::fs::read_to_string(path)?
        .trim_start_matches('\u{feff}') // BOMの対策
        .to_string();

    let mut raw_nodes: Vec<RawNode> = Vec::new();
    let mut first_text = true;

    for line in content.lines() {
        if let Some(caps) = HEADING_RE.captures(line) {
            let level = caps[1].len();
            let title = caps[2].to_string();
            raw_nodes.push(RawNode { title, level, text: "".to_string() });
            first_text = true;
        } else {
            if raw_nodes.is_empty() { continue; }
            let last = raw_nodes.last_mut().unwrap();

            if first_text {
                first_text = false;
            }else{
            last.text.push('\n');
            }

            last.text.push_str(&line);
        }
    }

    let mut nodes: Vec<Node<T>> = Vec::new();
    let mut current_node: Option<&mut Node<T>> = None;
    let mut tmp_options: HashMap<String, Vec<RawNode>> = HashMap::new();

    for raw in raw_nodes {
        match raw.level {
            1 => {
                nodes.push(Node {
                    label: raw.title,
                    description: raw.text,
                    dependencies: Vec::new(),
                    options: parse_options(&[])?,
                });
                current_node = nodes.last_mut();
            }
            2 => {
                let Some(node) = current_node.as_deref_mut() else {
                    continue;
                };
                match raw.title.as_str() {
                    "description" => node.description = parse_description(&raw.text),
                    "dependencies" => node.dependencies = parse_dependencies(&raw.text),
                    _ => continue
                }
            }
            _ => {
                let Some(node) = current_node.as_deref_mut() else {
                    continue;
                };
                tmp_options.entry(node.label.clone()).or_default().push(raw);
            }
        }
    }

    for (label, raw_nodes) in tmp_options {
        let node = nodes.iter_mut().find(|n| n.label == label).unwrap();
        node.options = parse_options(&raw_nodes)?;
    }

    Ok(nodes)
}

pub fn trim_text(text: &str) -> String {
    text.trim().to_string()
}

pub fn list_text_to_array(text: &str) -> Vec<String> {
    text
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else if let Some(rest) = line.strip_prefix("- ") {
                if rest.is_empty() { return None; }
                Some(rest.to_string())
            } else {
                Some(line.to_string())
            }
        })
        .collect()
}

pub fn raw_node_to_value<T>(raw_nodes: &[RawNode]) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned,
{
    let mut map = Map::new();
    for raw in raw_nodes {
        map.insert(raw.title.clone(), Value::String(raw.text.trim().to_string()));
    }
    serde_json::from_value(Value::Object(map))
}


pub fn load_nodes_from_markdown<T>(path: impl AsRef<Path>) -> Result<Vec<Node<T>>, LoadError>
where
    T: serde::de::DeserializeOwned,
{
    load_nodes_from_markdown_detail(path, trim_text, list_text_to_array, |raw| raw_node_to_value::<T>(raw))
}