//! Stage3 tarball builder implementation.

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Builder for stage3 tarballs
pub struct Stage3Builder {
    /// Source directory containing files to package
    source_dir: PathBuf,
    /// Output directory for the tarball
    output_dir: PathBuf,
}

impl Stage3Builder {
    pub fn new(source_dir: impl AsRef<Path>, output_dir: impl AsRef<Path>) -> Self {
        Self {
            source_dir: source_dir.as_ref().to_path_buf(),
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }

    /// Build the stage3 tarball
    pub fn build(&self) -> Result<PathBuf> {
        // For now, this is a placeholder
        // The actual implementation will:
        // 1. Create a minimal rootfs structure
        // 2. Copy binaries from the Rocky rootfs
        // 3. Set up systemd services for installed system
        // 4. Package as tar.xz

        let tarball_path = self.output_dir.join("levitateos-stage3.tar.xz");

        // TODO: Implement actual build logic
        // This will be developed iteratively as install-tests identifies
        // what's needed in the stage3

        Ok(tarball_path)
    }
}
