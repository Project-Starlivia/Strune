// Module declarations
mod main;
pub mod ingwaz;

// Re-exports from submodules
pub use ingwaz::{render, RenderNode, DEFAULT_CONFIG};
pub use main::{
    copy_dir_all,
    copy_public_directory,
    prepare_output_dir,
};
