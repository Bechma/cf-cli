mod run_loop;
mod templates;

use crate::common::CommonArgs;
use crate::run::run_loop::RunSignal;
use anyhow::Context;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct RunArgs {
    /// Path to the module to run
    #[arg(short = 'p', long, default_value = ".")]
    path: PathBuf,
    /// Watch for changes
    #[arg(short = 'w', long)]
    watch: bool,
    /// Use OpenTelemetry tracing while running the project
    #[arg(long)]
    otel: bool,
    /// Run in release mode
    #[arg(short = 'r', long, hide = true)]
    release: bool,
    #[command(flatten)]
    common_args: CommonArgs,
}

impl RunArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        let path = self
            .path
            .canonicalize()
            .context("can't canonicalize workspace")?;

        let config_path = self
            .common_args
            .config
            .canonicalize()
            .context("can't canonicalize config")?;

        let rl = run_loop::RunLoop::new(path, config_path);
        run_loop::OTEL.store(self.otel, std::sync::atomic::Ordering::Relaxed);
        run_loop::RELEASE.store(self.release, std::sync::atomic::Ordering::Relaxed);

        loop {
            match rl.run(self.watch)? {
                RunSignal::Rerun => continue,
                RunSignal::Stop => break Ok(()),
            }
        }
    }
}
