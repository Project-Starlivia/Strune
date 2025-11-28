use core::node::Node;
use sailfish::TemplateOnce;
use operation::NodeWithDependents;

#[derive(TemplateOnce)]
#[template(path = "node.stpl")]
struct NodeTemplate<'a> {
    node: &'a NodeWithDependents,
}

pub fn render_node_page(node: &NodeWithDependents) -> Result<String, sailfish::RenderError> {
    NodeTemplate { node }.render_once()
}