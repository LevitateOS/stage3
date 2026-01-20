//! Stage3 tarball builder implementation.
//!
//! Builds a complete rootfs tarball for LevitateOS installation.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::context::BuildContext;
use crate::rootfs::{binaries, etc, filesystem, pam, recipe, systemd};

/// Builder for stage3 tarballs.
pub struct Stage3Builder {
    /// Source directory containing Rocky rootfs
    source_dir: PathBuf,
    /// Output directory for the tarball
    output_dir: PathBuf,
    /// Optional path to recipe binary
    recipe_binary: Option<PathBuf>,
}

impl Stage3Builder {
    pub fn new(source_dir: impl AsRef<Path>, output_dir: impl AsRef<Path>) -> Self {
        Self {
            source_dir: source_dir.as_ref().to_path_buf(),
            output_dir: output_dir.as_ref().to_path_buf(),
            recipe_binary: None,
        }
    }

    /// Set the path to the recipe binary.
    pub fn with_recipe(mut self, recipe_binary: impl AsRef<Path>) -> Self {
        self.recipe_binary = Some(recipe_binary.as_ref().to_path_buf());
        self
    }

    /// Build the stage3 tarball.
    pub fn build(&self) -> Result<PathBuf> {
        println!("Building stage3 tarball...");
        println!("  Source: {}", self.source_dir.display());
        println!("  Output: {}", self.output_dir.display());

        // Validate source directory
        if !self.source_dir.exists() {
            anyhow::bail!(
                "Source directory does not exist: {}",
                self.source_dir.display()
            );
        }

        // Create output directory
        fs::create_dir_all(&self.output_dir)?;

        // Create staging directory
        let staging_dir = self.output_dir.join("staging");
        if staging_dir.exists() {
            fs::remove_dir_all(&staging_dir)?;
        }
        fs::create_dir_all(&staging_dir)?;

        // Create build context
        let mut ctx = BuildContext::new(
            self.source_dir.clone(),
            staging_dir.clone(),
            self.output_dir.clone(),
        );

        if let Some(ref recipe_path) = self.recipe_binary {
            ctx = ctx.with_recipe(recipe_path.clone());
        }

        // Build the rootfs
        self.build_rootfs(&ctx)?;

        // Create the tarball
        let tarball_path = self.create_tarball(&staging_dir)?;

        // Clean up staging directory
        println!("Cleaning up staging directory...");
        fs::remove_dir_all(&staging_dir)?;

        println!("Stage3 tarball created: {}", tarball_path.display());
        Ok(tarball_path)
    }

    /// Build the complete rootfs in staging directory.
    fn build_rootfs(&self, ctx: &BuildContext) -> Result<()> {
        println!("\n=== Building rootfs ===\n");

        // 1. Create FHS directory structure
        filesystem::create_fhs_structure(&ctx.staging)?;

        // 2. Create symlinks (must be after dirs but before binaries)
        filesystem::create_symlinks(&ctx.staging)?;

        // 3. Copy shell (bash) first
        binaries::copy_shell(ctx)?;

        // 4. Copy coreutils binaries
        binaries::copy_coreutils(ctx)?;

        // 5. Copy sbin utilities
        binaries::copy_sbin_utils(ctx)?;

        // 6. Copy systemd binaries and setup
        binaries::copy_systemd_binaries(ctx)?;
        binaries::copy_login_binaries(ctx)?;

        // 7. Copy systemd units
        systemd::copy_systemd_units(ctx)?;
        systemd::copy_dbus_symlinks(ctx)?;

        // 8. Set up systemd services
        systemd::setup_getty(ctx)?;
        systemd::setup_serial_console(ctx)?;
        systemd::setup_networkd(ctx)?;
        systemd::set_default_target(ctx)?;
        systemd::setup_dbus(ctx)?;

        // 9. Copy udev rules and tmpfiles
        systemd::copy_udev_rules(ctx)?;
        systemd::copy_tmpfiles(ctx)?;
        systemd::copy_sysctl(ctx)?;

        // 10. Create /etc configuration files
        etc::create_etc_files(ctx)?;
        etc::copy_timezone_data(ctx)?;
        etc::copy_locales(ctx)?;

        // 11. Set up PAM
        pam::setup_pam(ctx)?;
        pam::copy_pam_modules(ctx)?;
        pam::create_security_config(ctx)?;

        // 12. Copy recipe package manager
        recipe::copy_recipe(ctx)?;
        recipe::setup_recipe_config(ctx)?;

        println!("\n=== Rootfs build complete ===\n");
        Ok(())
    }

    /// Create the tarball from the staging directory.
    fn create_tarball(&self, staging: &Path) -> Result<PathBuf> {
        println!("Creating tarball...");

        let tarball_path = self.output_dir.join("levitateos-stage3.tar.xz");

        // Use tar command for better compatibility and performance
        let status = Command::new("tar")
            .args([
                "-cJf",
                tarball_path.to_str().unwrap(),
                "-C",
                staging.to_str().unwrap(),
                ".",
            ])
            .status()
            .context("Failed to run tar command")?;

        if !status.success() {
            anyhow::bail!("tar command failed with status: {}", status);
        }

        // Print tarball size
        let metadata = fs::metadata(&tarball_path)?;
        let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
        println!("  Tarball size: {:.2} MB", size_mb);

        Ok(tarball_path)
    }
}

/// List contents of an existing tarball.
pub fn list_tarball(path: &Path) -> Result<()> {
    println!("Contents of {}:", path.display());

    let status = Command::new("tar")
        .args(["-tJf", path.to_str().unwrap()])
        .status()
        .context("Failed to run tar command")?;

    if !status.success() {
        anyhow::bail!("tar command failed with status: {}", status);
    }

    Ok(())
}

/// Verify tarball contents.
pub fn verify_tarball(path: &Path) -> Result<()> {
    println!("Verifying {}...", path.display());

    // Check essential files exist in tarball
    let essential_files = [
        "./usr/bin/bash",
        "./usr/bin/sh",
        "./usr/sbin/init",
        "./etc/passwd",
        "./etc/shadow",
        "./etc/os-release",
        "./usr/lib/systemd/systemd",
    ];

    let output = Command::new("tar")
        .args(["-tJf", path.to_str().unwrap()])
        .output()
        .context("Failed to run tar command")?;

    if !output.status.success() {
        anyhow::bail!("tar command failed");
    }

    let contents = String::from_utf8_lossy(&output.stdout);
    let mut missing = Vec::new();

    for file in essential_files {
        if !contents.contains(file) {
            missing.push(file);
        }
    }

    if missing.is_empty() {
        println!("  All essential files present");
        Ok(())
    } else {
        println!("  Missing files:");
        for file in &missing {
            println!("    - {}", file);
        }
        anyhow::bail!("Tarball verification failed: missing essential files");
    }
}
