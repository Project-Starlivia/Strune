use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use core::node::Node;

pub trait WithDependents {
    fn dependents(&self) -> &Vec<String>;
    fn dependents_mut(&mut self) -> &mut Vec<String>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionsWithDependents<T> {
    #[serde(flatten)]
    pub base: T,
    #[serde(default)]
    pub dependents: Vec<String>,
}

impl<T> WithDependents for OptionsWithDependents<T> {
    fn dependents(&self) -> &Vec<String> {
        &self.dependents
    }

    fn dependents_mut(&mut self) -> &mut Vec<String> {
        &mut self.dependents
    }
}

pub type NodeWithDependents<T = Value> = Node<OptionsWithDependents<T>>;

pub fn fill_dependents<T>(mut nodes: Vec<Node<T>>) -> Vec<Node<T>>
where
    T: WithDependents,
{
    let mut reverse: HashMap<String, Vec<String>> = HashMap::new();

    for node in &nodes {
        for dep in &node.dependencies {
            reverse.entry(dep.clone()).or_default().push(node.label.clone());
        }
    }
    
    for node in &mut nodes {
        let deps = reverse.remove(&node.label).unwrap_or_default();
        let slot = node.options.dependents_mut();

        slot.clear();
        slot.extend(deps);
    }

    nodes
}