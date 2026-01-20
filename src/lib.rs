//! Stage3 tarball builder library.
//!
//! This crate builds the stage3 tarball that gets extracted during installation.
//! The stage3 contains everything needed for a bootable LevitateOS system.

pub mod builder;

pub use builder::Stage3Builder;
