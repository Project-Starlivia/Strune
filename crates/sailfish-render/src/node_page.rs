use core::node::Node;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "node.stpl")]
struct NodeTemplate<'a, T> {
    node: &'a Node<T>,
}

pub fn render_node_page<T>(node: &Node<T>) -> Result<String, sailfish::RenderError> {
    NodeTemplate { node }.render_once()
}