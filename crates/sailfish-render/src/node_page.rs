use core::node::Node;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path="node.stpl")]
struct NodeTemplate<'a> {
    node: &'a Node
}

pub fn render_node_page(node: &Node) -> Result<String, sailfish::RenderError> {
    NodeTemplate { node }.render_once()
}