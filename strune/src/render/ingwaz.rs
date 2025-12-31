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

use super::main::{
    copy_public_directory,
    prepare_output_dir,
};

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

/// Build render node map from nodes
fn build_render_node_map<T>(nodes: &[Node<T>]) -> HashMap<String, RenderNode>
where
    T: MaybeSlug,
{
    let slug_map = label_slug_map(nodes);
    nodes.iter()
        .map(|n| {
            let key = n.label.clone();
            let value = RenderNode::new_with_map(key.clone(), &slug_map);
            (key, value)
        })
        .collect()
}

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
    output_dir_path: impl AsRef<Path>,
    public_dir_path: impl AsRef<Path>,
    config_path: impl AsRef<Path>,
    nodes: &[Node<T>],
) -> Result<(), tera::Error>
where
    T: Serialize + MaybeSlug + MaybeDependents,
{
    let output_dir_path = output_dir_path.as_ref();
    let public_dir_path = public_dir_path.as_ref();
    let config_path = config_path.as_ref();

    // 1. Initialize: Prepare output directory and load template engine
    prepare_output_dir(output_dir_path).map_err(|e| tera::Error::msg(e))?;
    let tera = Tera::new(template_dir)?;

    // 2. Load configuration
    let config = load_config_from_yaml(config_path).map_err(|e| tera::Error::msg(e.to_string()))?;
    let config_map = if let Value::Object(map) = config {
        map
    } else {
        return Err(tera::Error::msg("config must be an object"));
    };

    // 3. Copy public directory
    copy_public_directory(public_dir_path, output_dir_path).map_err(|e| tera::Error::msg(e))?;

    // 4. Build template context
    let mut ctx = Context::new();
    ctx.insert("config", &Value::Object(config_map));

    // 5. Build render node map
    let render_nodes = build_render_node_map(nodes);

    // 6. Render HTML pages for each node
    for node in nodes {
        println!("Rendering: {}", node.to_string());
        let file_link = render_nodes.get(&node.label).unwrap().link.clone();
        let path = output_dir_path.join(format!("{}.html", file_link));

        let html = render_node_page::<T>(&tera, ctx.clone(), node, &render_nodes)?;
        fs::write(path, html).map_err(|e| tera::Error::msg(e.to_string()))?;
    }

    Ok(())
}
