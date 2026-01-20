//! Stage3 tarball builder library.
//!
//! This crate builds the stage3 tarball that gets extracted during installation.
//! The stage3 contains everything needed for a bootable LevitateOS system
//! (except the kernel, which is installed separately).
//!
//! ## Components
//!
//! - **binaries**: Coreutils, sbin utilities, systemd binaries
//! - **etc**: System configuration files (passwd, shadow, fstab, etc.)
//! - **systemd**: Unit files for installed system boot
//! - **pam**: Real PAM authentication (not permissive like live)
//! - **recipe**: Package manager integration

pub mod binary;
pub mod builder;
pub mod context;
pub mod rootfs;

pub use builder::Stage3Builder;
pub use context::BuildContext;
