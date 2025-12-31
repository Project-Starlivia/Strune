use std::fs;
use std::path::Path;

/// Helper function to recursively copy a directory
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), std::io::Error> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Copy public directory if it exists
pub fn copy_public_directory(input_path: &Path, output_dir: &Path) -> Result<(), String> {
    if input_path.exists() {
        let dist_public = output_dir.join("public");
        copy_dir_all(input_path, &dist_public)
            .map_err(|e| format!("Failed to copy public directory: {}", e))?;
        println!("Copied public directory from {} to dist/public", input_path.display());
    }
    Ok(())
}

/// Prepare output directory (clear if exists, then create)
pub fn prepare_output_dir(output_dir: &Path) -> Result<(), String> {
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)
            .map_err(|e| format!("Failed to remove output directory: {}", e))?;
    }
    fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;
    Ok(())
}
