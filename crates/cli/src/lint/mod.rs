use crate::common::parse_and_chdir;
use anyhow::Result;
use clap::Args;

#[cfg(feature = "dylint-rules")]
use anyhow::{Context, bail};
#[cfg(feature = "dylint-rules")]
use std::collections::BTreeSet;
#[cfg(feature = "dylint-rules")]
use std::io::Write;
use std::path::PathBuf;
#[cfg(feature = "dylint-rules")]
use std::process::Command;

#[derive(Args)]
pub struct LintArgs {
    /// Path to the module workspace root
    #[arg(short = 'p', long, value_parser = parse_and_chdir)]
    pub path: Option<PathBuf>,
    #[arg(long)]
    clippy: bool,
    #[arg(long)]
    dylint: bool,
}

#[cfg(feature = "dylint-rules")]
include!(concat!(env!("OUT_DIR"), "/generated_libs.rs"));

impl LintArgs {
    pub fn run(&self) -> Result<()> {
        if self.dylint {
            run_dylint()?;
        }
        Ok(())
    }
}

#[cfg(feature = "dylint-rules")]
fn embedded_toolchains() -> Result<BTreeSet<String>> {
    LIBS.iter()
        .map(|(filename, _)| {
            let (_, toolchain_and_ext) = filename
                .rsplit_once('@')
                .with_context(|| format!("missing toolchain marker in `{filename}`"))?;
            let (toolchain, _) = toolchain_and_ext
                .rsplit_once('.')
                .with_context(|| format!("missing library extension in `{filename}`"))?;
            Ok(toolchain.to_owned())
        })
        .collect()
}

#[cfg(feature = "dylint-rules")]
fn ensure_toolchain_installed(toolchain: &str) -> Result<()> {
    let installed = Command::new("rustup")
        .args(["toolchain", "list"])
        .output()
        .context("failed to list installed rustup toolchains")?;

    if !installed.status.success() {
        bail!(
            "rustup toolchain list failed: {}",
            String::from_utf8_lossy(&installed.stderr)
        );
    }

    let installed = String::from_utf8(installed.stdout)?;
    let installed_prefix = format!("{toolchain}-");
    if installed
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .any(|installed| installed == toolchain || installed.starts_with(&installed_prefix))
    {
        return Ok(());
    }

    let install = Command::new("rustup")
        .args(["toolchain", "install", toolchain, "--profile", "minimal"])
        .output()
        .with_context(|| format!("failed to install rustup toolchain `{toolchain}`"))?;

    if !install.status.success() {
        bail!(
            "rustup toolchain install failed for `{toolchain}`: {}",
            String::from_utf8_lossy(&install.stderr)
        );
    }

    Ok(())
}

#[cfg(feature = "dylint-rules")]
fn run_dylint() -> Result<()> {
    // Write every embedded dylib to a per-run temp directory so dylint can
    // dlopen them.  The temp dir (and its contents) is removed automatically
    // when `tmp_dir` drops at the end of main, which is safe because dylint
    // has already finished using the files by then.
    let tmp_dir = tempfile::tempdir().context("could not create temp dir for dylibs")?;

    for toolchain in embedded_toolchains()? {
        ensure_toolchain_installed(&toolchain)?;
    }

    let lib_paths: Vec<String> = LIBS
        .iter()
        .map(|(filename, bytes)| {
            let dest = tmp_dir.path().join(filename);
            let mut f = std::fs::File::create(&dest)
                .with_context(|| format!("could not create {filename} in temp dir"))?;
            f.write_all(bytes)
                .with_context(|| format!("could not write {filename} to temp dir"))?;
            Ok(dest.to_string_lossy().into_owned())
        })
        .collect::<Result<_>>()?;

    let opts = dylint::opts::Dylint {
        // Check all packages in the workspace found in the current working
        // directory.  No manifest_path → dylint resolves the workspace from
        // the CWD, which is exactly what we want when the tool is invoked
        // inside a project.
        operation: dylint::opts::Operation::Check(dylint::opts::Check {
            lib_sel: dylint::opts::LibrarySelection {
                // Point directly at the extracted, versioned dylib files.
                // dylint parses the toolchain from each filename so no further
                // discovery or building is necessary.
                lib_paths,
                ..Default::default()
            },
            // Lint the whole workspace, not just the root crate.
            workspace: true,
            ..Default::default()
        }),
        ..Default::default()
    };

    dylint::run(&opts)
}

#[cfg(not(feature = "dylint-rules"))]
fn run_dylint() -> Result<()> {
    unimplemented!("dylint-rules feature not enabled")
}
