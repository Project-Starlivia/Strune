use std::collections::HashMap;
use core::node::Node;

pub trait MaybeDependents {
    fn dependents(&self) -> Option<&[String]>;
    fn dependents_mut(&mut self) -> &mut Option<Vec<String>>;
}

pub trait HasDependents {
    fn dependents(&self) -> &[String];
    fn dependents_mut(&mut self) -> &mut Vec<String>;
}

#[macro_export]
macro_rules! impl_has_dependents_detail {
    (<$($gen:tt),*> $ty:ty, $field:ident) => {
        $crate::impl_has!(
            <$($gen),*> $ty, $field;
            $crate::HasDependents,
            dependents -> &[String],
            dependents_mut -> &mut Vec<String>
        );
    };
}

#[macro_export]
macro_rules! impl_has_dependents {
    ($name:ident) => {
        $crate::impl_has_dependents_detail!(<T> $name<T>, dependents);
    };
}

#[macro_export]
macro_rules! impl_maybe_dependents_detail {
    (<$($gen:tt),*> $ty:ty, $field:ident) => {
        $crate::impl_maybe!(
            <$($gen),*> $ty, $field;
            $crate::MaybeDependents,
            dependents -> Option<&[String]>,
            dependents_mut -> &mut Option<Vec<String>>
        );
    };
}


#[macro_export]
macro_rules! impl_maybe_dependents {
    ($name:ident) => {
        $crate::impl_maybe_dependents_detail!(<T> $name<T>, dependents);
    };
}

pub fn fill_dependents<T>(mut nodes: Vec<Node<T>>) -> Vec<Node<T>>
where
    T: MaybeDependents,
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
        *slot = Some(deps);
    }

    nodes
}