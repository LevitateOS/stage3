//! Stage3 tarball builder for LevitateOS.
//!
//! Builds a minimal but complete rootfs tarball containing:
//! - Base system binaries (bash, coreutils, systemd)
//! - System configuration (/etc files)
//! - PAM authentication
//! - Package manager (recipe)
//!
//! Note: Kernel is NOT included - it's installed separately in Phase 5.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use stage3::builder::{list_tarball, verify_tarball, Stage3Builder};

#[derive(Parser)]
#[command(name = "stage3")]
#[command(about = "Build stage3 tarball for LevitateOS installation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Build the stage3 tarball
    Build {
        /// Source directory containing Rocky rootfs
        #[arg(short, long)]
        source: PathBuf,

        /// Output directory for the tarball
        #[arg(short, long, default_value = "output")]
        output: PathBuf,

        /// Path to recipe binary (optional)
        #[arg(short, long)]
        recipe: Option<PathBuf>,
    },

    /// List contents of an existing tarball
    List {
        /// Path to tarball
        path: PathBuf,
    },

    /// Verify tarball contains essential files
    Verify {
        /// Path to tarball
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            source,
            output,
            recipe,
        } => {
            let mut builder = Stage3Builder::new(&source, &output);

            if let Some(recipe_path) = recipe {
                builder = builder.with_recipe(recipe_path);
            }

            let tarball_path = builder.build()?;
            println!("\nBuild complete: {}", tarball_path.display());
        }
        Commands::List { path } => {
            list_tarball(&path)?;
        }
        Commands::Verify { path } => {
            verify_tarball(&path)?;
        }
    }

    Ok(())
}
