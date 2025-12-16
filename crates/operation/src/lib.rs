pub mod dependents;
pub use dependents::{HasDependents, MaybeDependents};

pub mod slug;
pub use slug::{HasSlug, MaybeSlug};

#[macro_export]
macro_rules! impl_has {
    (<$($gen:tt),*> $ty:ty, $field:ident;
     $trait:path, $get:ident -> $get_ret:ty, $get_mut:ident -> $mut_ret:ty) => {
        impl<$($gen),*> $trait for $ty {
            fn $get(&self) -> $get_ret { &self.$field }
            fn $get_mut(&mut self) -> $mut_ret { &mut self.$field }
        }
    };
}

#[macro_export]
macro_rules! impl_maybe {
    (<$($gen:tt),*> $ty:ty, $field:ident;
     $trait:path, $get:ident -> $get_ret:ty, $get_mut:ident -> $mut_ret:ty) => {
        impl<$($gen),*> $trait for $ty {
            fn $get(&self) -> $get_ret { self.$field.as_deref() }
            fn $get_mut(&mut self) -> $mut_ret { &mut self.$field }
        }
    };
}