use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Serialize;
use tera::{Tera, Context};

use core::node::Node;
use operation::slug::{label_slug_map, MaybeSlug};

#[derive(Serialize)]
struct TmplOptions<'a, O: Serialize> {
    #[serde(flatten)]
    inner: &'a O,
    stem_map: HashMap<String,String>,
}

pub fn render<T, O>(
    template_dir: &str,
    template_path: &str,
    output_dir: impl AsRef<Path>,
    nodes: &[Node<T>],
    options: O,
) -> Result<(), tera::Error>
where
    T: Serialize + MaybeSlug,
    O: Serialize + Default,
{
    let tera = Tera::new(template_dir)?;
    fs::create_dir_all(&output_dir).map_err(|e| tera::Error::msg(e.to_string()))?;

    let opt = TmplOptions {
        inner: &options,
        stem_map: label_slug_map(nodes),
    };

    let mut ctx = Context::new();
    ctx.insert("options", &opt);
    for node in nodes {
        ctx.insert("node", node);

        let file_stem = opt.stem_map.get(&node.label).unwrap();
        let path = output_dir.as_ref().join(format!("{file_stem}.html"));

        let html = tera.render(template_path, &ctx)?;
        fs::write(path, html).map_err(|e| tera::Error::msg(e.to_string()))?;
    }
    Ok(())
}

pub fn render_simple<T>(
    template_dir: &str,
    template_path: &str,
    output_dir: impl AsRef<Path>,
    nodes: &[Node<T>],
) -> Result<(), tera::Error>
where
    T: Serialize + MaybeSlug,
{
    render::<T, ()>(template_dir, template_path, output_dir, nodes, ())
}