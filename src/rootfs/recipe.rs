//! Recipe package manager integration.
//!
//! Copies the recipe binary into the stage3 tarball.

use anyhow::{Context, Result};
use std::fs;

use crate::binary::make_executable;
use crate::context::BuildContext;

/// Copy recipe binary to the stage3.
pub fn copy_recipe(ctx: &BuildContext) -> Result<()> {
    println!("Copying recipe package manager...");

    // Check if recipe binary path is configured
    let recipe_path = match &ctx.recipe_binary {
        Some(path) => path.clone(),
        None => {
            // Default location
            let default_path = std::path::PathBuf::from("../recipe/target/release/recipe");
            if default_path.exists() {
                default_path
            } else {
                println!("  Warning: recipe binary not found, skipping");
                return Ok(());
            }
        }
    };

    if !recipe_path.exists() {
        println!("  Warning: recipe binary not found at {:?}, skipping", recipe_path);
        return Ok(());
    }

    // Copy to /usr/bin/recipe
    let dest = ctx.staging.join("usr/bin/recipe");
    fs::copy(&recipe_path, &dest)
        .with_context(|| format!("Failed to copy recipe from {:?}", recipe_path))?;
    make_executable(&dest)?;

    println!("  Copied recipe to /usr/bin/recipe");
    Ok(())
}

/// Create recipe configuration directory.
pub fn setup_recipe_config(ctx: &BuildContext) -> Result<()> {
    println!("Setting up recipe configuration...");

    // Create recipe directories
    let recipe_dirs = [
        "etc/recipe",
        "var/lib/recipe",
        "var/cache/recipe",
    ];

    for dir in recipe_dirs {
        fs::create_dir_all(ctx.staging.join(dir))?;
    }

    // Create basic recipe configuration
    fs::write(
        ctx.staging.join("etc/recipe/recipe.conf"),
        r#"# Recipe package manager configuration

# Repository URL (set during installation)
# repository = "https://packages.levitateos.org"

# Cache directory
cache_dir = "/var/cache/recipe"

# Database directory
db_dir = "/var/lib/recipe"
"#,
    )?;

    println!("  Created recipe configuration");
    Ok(())
}
