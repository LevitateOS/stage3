//! Stage3 tarball builder for LevitateOS.
//!
//! Builds a minimal but complete rootfs tarball containing:
//! - Base system binaries (bash, coreutils, systemd)
//! - Boot files (kernel, initramfs for installed system)
//! - System configuration templates
//! - Package manager (recipe)

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "stage-3")]
#[command(about = "Build stage3 tarball for LevitateOS installation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Build the stage3 tarball
    Build {
        /// Output directory
        #[arg(short, long, default_value = "output")]
        output: String,
    },
    /// List contents of an existing tarball
    List {
        /// Path to tarball
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { output } => {
            println!("Building stage3 tarball to {}...", output);
            // TODO: Implement stage3 build
            println!("Stage3 builder not yet implemented");
            println!("For now, use leviso to build the initramfs");
        }
        Commands::List { path } => {
            println!("Listing contents of {}...", path);
            // TODO: List tarball contents
        }
    }

    Ok(())
}
