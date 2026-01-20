//! Binary lists and copying for stage3.
//!
//! Contains the complete list of binaries needed for an installed system.

use anyhow::Result;

use crate::binary::{copy_binary_with_libs, copy_bash, copy_sbin_binary_with_libs};
use crate::context::BuildContext;

/// Coreutils and essential user binaries.
const COREUTILS: &[&str] = &[
    // File operations
    "ls",
    "cat",
    "cp",
    "mv",
    "rm",
    "mkdir",
    "rmdir",
    "touch",
    "chmod",
    "chown",
    "ln",
    "readlink",
    "dirname",
    "basename",
    "realpath",
    "stat",
    "file",
    // Text processing
    "echo",
    "printf",
    "head",
    "tail",
    "wc",
    "sort",
    "uniq",
    "cut",
    "tr",
    "diff",
    "tee",
    "yes",
    // Search/find
    "grep",
    "find",
    "xargs",
    "which",
    // System info
    "pwd",
    "uname",
    "date",
    "env",
    "printenv",
    "id",
    "whoami",
    "groups",
    "hostname",
    // Process
    "sleep",
    "kill",
    "ps",
    "pgrep",
    "pkill",
    "nice",
    "nohup",
    // Compression
    "gzip",
    "gunzip",
    "xz",
    "unxz",
    "bzip2",
    "bunzip2",
    // Archive
    "tar",
    "cpio",
    // Text editors (basic)
    "vi",
    "vim",
    // Shell utilities
    "true",
    "false",
    "test",
    "expr",
    "seq",
    // Disk utilities
    "df",
    "du",
    "sync",
    // More text processing
    "sed",
    "awk",
    "gawk",
    // User
    "su",
    "sudo",
    "passwd",
    // Network
    "ping",
    "curl",
    "wget",
    // Systemd control
    "systemctl",
    "journalctl",
    "timedatectl",
    "hostnamectl",
    "localectl",
    "loginctl",
    "bootctl",
];

/// Sbin utilities (system administration).
const SBIN_UTILS: &[&str] = &[
    // Filesystem
    "mount",
    "umount",
    "fsck",
    "fsck.ext4",
    "e2fsck",
    "mkfs.ext4",
    "mke2fs",
    "mkfs.fat",
    "mkfs.vfat",
    // Disk management
    "blkid",
    "fdisk",
    "sfdisk",
    "parted",
    "partprobe",
    "wipefs",
    "lsblk",
    // System control
    "reboot",
    "shutdown",
    "poweroff",
    "halt",
    // Hardware
    "hwclock",
    "lspci",
    "lsusb",
    // Kernel modules
    "insmod",
    "rmmod",
    "modprobe",
    "lsmod",
    "depmod",
    // Boot
    "chroot",
    "pivot_root",
    // Library cache
    "ldconfig",
    // User management
    "useradd",
    "userdel",
    "usermod",
    "groupadd",
    "groupdel",
    "groupmod",
    "chpasswd",
    // Network
    "ip",
    "ss",
    "ifconfig",
    "route",
    // System
    "sysctl",
    "losetup",
    // Time
    "chronyd",
    // SELinux (if present)
    "getenforce",
    "setenforce",
];

/// Systemd binaries to copy.
const SYSTEMD_BINARIES: &[&str] = &[
    "systemd-executor",
    "systemd-shutdown",
    "systemd-sulogin-shell",
    "systemd-cgroups-agent",
    "systemd-journald",
    "systemd-modules-load",
    "systemd-sysctl",
    "systemd-tmpfiles",
    "systemd-timedated",
    "systemd-hostnamed",
    "systemd-localed",
    "systemd-logind",
    "systemd-networkd",
    "systemd-resolved",
    "systemd-udevd",
    "systemd-fsck",
    "systemd-remount-fs",
    "systemd-vconsole-setup",
    "systemd-random-seed",
];

/// Copy all coreutils binaries.
pub fn copy_coreutils(ctx: &BuildContext) -> Result<()> {
    println!("Copying coreutils binaries...");

    let mut copied = 0;
    for binary in COREUTILS {
        if copy_binary_with_libs(ctx, binary, "usr/bin")? {
            copied += 1;
        }
    }

    println!("  Copied {}/{} coreutils binaries", copied, COREUTILS.len());
    Ok(())
}

/// Copy all sbin utilities.
pub fn copy_sbin_utils(ctx: &BuildContext) -> Result<()> {
    println!("Copying sbin utilities...");

    let mut copied = 0;
    for binary in SBIN_UTILS {
        if copy_sbin_binary_with_libs(ctx, binary)? {
            copied += 1;
        }
    }

    println!("  Copied {}/{} sbin utilities", copied, SBIN_UTILS.len());
    Ok(())
}

/// Copy bash shell.
pub fn copy_shell(ctx: &BuildContext) -> Result<()> {
    println!("Copying bash shell...");
    copy_bash(ctx)?;
    println!("  Copied bash");
    Ok(())
}

/// Copy systemd binaries and libraries.
pub fn copy_systemd_binaries(ctx: &BuildContext) -> Result<()> {
    println!("Copying systemd binaries...");

    // Copy main systemd binary
    let systemd_src = ctx.source.join("usr/lib/systemd/systemd");
    let systemd_dst = ctx.staging.join("usr/lib/systemd/systemd");
    if systemd_src.exists() {
        std::fs::create_dir_all(systemd_dst.parent().unwrap())?;
        std::fs::copy(&systemd_src, &systemd_dst)?;
        crate::binary::make_executable(&systemd_dst)?;
        println!("  Copied systemd");
    }

    // Copy helper binaries
    for binary in SYSTEMD_BINARIES {
        let src = ctx.source.join("usr/lib/systemd").join(binary);
        let dst = ctx.staging.join("usr/lib/systemd").join(binary);
        if src.exists() {
            std::fs::copy(&src, &dst)?;
            crate::binary::make_executable(&dst)?;
        }
    }

    // Copy systemd private libraries
    let systemd_lib_src = ctx.source.join("usr/lib64/systemd");
    if systemd_lib_src.exists() {
        std::fs::create_dir_all(ctx.staging.join("usr/lib64/systemd"))?;
        for entry in std::fs::read_dir(&systemd_lib_src)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("libsystemd-") && name_str.ends_with(".so") {
                let dst = ctx.staging.join("usr/lib64/systemd").join(&name);
                std::fs::copy(entry.path(), &dst)?;
            }
        }
    }

    // Create /sbin/init -> /usr/lib/systemd/systemd symlink
    let init_link = ctx.staging.join("usr/sbin/init");
    if !init_link.exists() && !init_link.is_symlink() {
        std::os::unix::fs::symlink("/usr/lib/systemd/systemd", &init_link)?;
    }

    println!("  Copied {} systemd binaries", SYSTEMD_BINARIES.len());
    Ok(())
}

/// Copy agetty and login binaries for getty/console.
pub fn copy_login_binaries(ctx: &BuildContext) -> Result<()> {
    println!("Copying login binaries...");

    let login_binaries = ["agetty", "login", "sulogin", "nologin"];

    for binary in login_binaries {
        copy_sbin_binary_with_libs(ctx, binary)?;
    }

    println!("  Copied login binaries");
    Ok(())
}
