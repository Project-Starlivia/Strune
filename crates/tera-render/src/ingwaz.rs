use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Serialize;
use serde_json::Value;
use tera::{Tera, Context};
use operation::MaybeDependents;
use strune_core::node::Node;
use operation::slug::{label_slug_map, MaybeSlug};

#[derive(Serialize)]
pub struct RenderNode{
    label: String,
    link: String,
}


impl RenderNode {
    pub fn new_with_map(label: String, map: &HashMap<String, String>) -> Self {
        let link = map.get(&label).unwrap().clone();
        Self{label, link}
    }
}
fn render_node_page<T>(
    tera: &Tera,
    mut context: Context,
    node: &Node<T>,
    render_nodes: &HashMap<String, RenderNode>
) -> Result<String, tera::Error>
where T: Serialize + MaybeDependents
{
    context.insert("title", &node.label);

    context.insert("current_node", &node);

    let dependencies: Vec<&RenderNode> = node
        .dependencies
        .iter()
        .map(|d| render_nodes.get(d).unwrap())
        .collect();
    context.insert("dependencies", &dependencies);

    let dependents: Vec<&RenderNode> = node
        .options
        .dependents()
        .into_iter()
        .flatten()
        .map(|s| render_nodes.get(s).unwrap())
        .collect();
    context.insert("dependents", &dependents);

    tera.render("ingwaz/node_page.html", &context)
}

pub fn render<T>(
    template_dir: &str,
    output_dir: impl AsRef<Path>,
    nodes: &[Node<T>],
) -> Result<(), tera::Error>
where
    T: Serialize + MaybeSlug + MaybeDependents,
{
    let tera = Tera::new(template_dir)?;
    fs::create_dir_all(&output_dir).map_err(|e| tera::Error::msg(e.to_string()))?;

    let mut ctx = Context::new();

    ctx.insert("page_logo_path", "https://raw.githubusercontent.com/Project-Starlivia/Strune/refs/heads/main/logo.svg");
    ctx.insert("page_title", "Strune");
    let header_links: Vec<Value> = Vec::new();
    ctx.insert("header_links", &header_links);

    let slug_map = label_slug_map(nodes);
    let render_nodes: HashMap<String, RenderNode> =
        nodes.iter()
            .map(|n| {
                let key = n.label.clone();
                let value = RenderNode::new_with_map(key.clone(), &slug_map);
                (key, value)
            })
            .collect();

    for node in nodes {
        let file_link = render_nodes.get(&node.label).unwrap().link.clone();
        let path = output_dir.as_ref().join(format!("{file_link}.html"));

        let html = render_node_page::<T>(&tera, ctx.clone(), node, &render_nodes)?;

        fs::write(path, html).map_err(|e| tera::Error::msg(e.to_string()))?;
    }
    Ok(())
}
