use std::collections::HashMap;
use strune_core::node::Node;

pub trait MaybeSlug {
    fn slug(&self) -> Option<&str>;
    fn slug_mut(&mut self) -> &mut Option<String>;
}

pub trait HasSlug {
    fn slug(&self) -> &str;
    fn slug_mut(&mut self) -> &mut String;
}

#[macro_export]
macro_rules! impl_has_slug_detail {
    (<$($gen:tt),*> $ty:ty, $field:ident) => {
        $crate::impl_has!(
            <$($gen),*> $ty, $field;
            $crate::HasSlug,
            slug -> &str,
            slug_mut -> &mut String
        );
    };
}

#[macro_export]
macro_rules! impl_has_slug {
    ($name:ident) => {
        $crate::impl_has_slug_detail!(<T> $name<T>, slug);
    };
}

#[macro_export]
macro_rules! impl_maybe_slug_detail {
    (<$($gen:tt),*> $ty:ty, $field:ident) => {
        $crate::impl_maybe!(
            <$($gen),*> $ty, $field;
            $crate::MaybeSlug,
            slug -> Option<&str>,
            slug_mut -> &mut Option<String>
        );
    };
}

#[macro_export]
macro_rules! impl_maybe_slug {
    ($name:ident) => {
        $crate::impl_maybe_slug_detail!(<T> $name<T>, slug);
    };
}

pub fn label_slug_map<T>(nodes: &[Node<T>]) -> HashMap<String, String>
where
    T: MaybeSlug,
{
    let mut result = HashMap::new();
    for node in nodes {
        let slug = node
            .options
            .slug()
            .map(str::to_string)
            .unwrap_or_else(|| node.label.clone());

        result.insert(node.label.clone(), slug);
    }
    result
}