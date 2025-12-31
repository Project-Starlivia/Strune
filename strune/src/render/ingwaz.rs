use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Serialize;
use serde_json::Value;
use tera::{Tera, Context};
use pulldown_cmark::{Parser, Options, html};

use crate::core::Node;
use crate::operation::{MaybeDependents, MaybeSlug, label_slug_map};
use crate::loader::yaml::load_config_from_yaml;

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

pub const DEFAULT_CONFIG: &str = "strune/config/ingwaz_default.yml";

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
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

    // Convert description from Markdown to HTML
    let description_html = markdown_to_html(&node.description);
    context.insert("description_html", &description_html);

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
    config_path: impl AsRef<Path>,
    nodes: &[Node<T>],
) -> Result<(), tera::Error>
where
    T: Serialize + MaybeSlug + MaybeDependents,
{
    let tera = Tera::new(template_dir)?;
    fs::create_dir_all(&output_dir).map_err(|e| tera::Error::msg(e.to_string()))?;

    let mut ctx = Context::new();

    // Load config from YAML or use default
    let config = load_config_from_yaml(config_path).map_err(|e| tera::Error::msg(e.to_string()))?;

    if let Value::Object(config_map) = config {
        ctx.insert("config", &Value::Object(config_map));
    } else {
        return Err(tera::Error::msg("config must be an object"));
    }

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
        println!("Rendering:{}", node.to_string());
        let file_link = render_nodes.get(&node.label).unwrap().link.clone();
        let path = output_dir.as_ref().join(format!("{file_link}.html"));

        let html = render_node_page::<T>(&tera, ctx.clone(), node, &render_nodes)?;

        fs::write(path, html).map_err(|e| tera::Error::msg(e.to_string()))?;
    }
    Ok(())
}
