use anyhow::{Context, bail};
use cargo_generate::{GenerateArgs, TemplatePath, generate};
use clap::Args;
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct ModuleArgs {
    /// Name of the new module to create (e.g., "goodbye")
    name: String,
    /// Path to the workspace root (defaults to current directory)
    #[arg(short = 'p', long, default_value = ".")]
    path: PathBuf,
    /// Verbose output
    #[arg(short = 'v', long)]
    verbose: bool,
    /// Path to a local template (instead of git)
    #[arg(long, conflicts_with_all = ["git", "branch"])]
    local_path: Option<String>,
    /// URL to the git repo
    #[arg(
        long,
        default_value = "https://github.com/cyberfabric/cf-template-rust"
    )]
    git: Option<String>,
    /// Subfolder relative to the git repo
    #[arg(long, default_value = "Modules")]
    subfolder: String,
    /// Branch of the git repo
    #[arg(long, default_value = "main")]
    branch: Option<String>,
}

impl ModuleArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        let modules_dir = self.path.join("modules");

        if !modules_dir.exists() {
            bail!(
                "modules directory does not exist at {}. Make sure you are in a workspace initialized with 'init'.",
                modules_dir.display()
            );
        }

        // Generate the main module
        self.generate_module()?;
        println!(
            "Module '{}' created at {}",
            self.name,
            modules_dir.display()
        );

        Ok(())
    }

    fn generate_module(&self) -> anyhow::Result<()> {
        let (git, auto_path, branch) = if self.local_path.is_some() {
            (None, None, None)
        } else {
            (
                self.git.clone(),
                Some(self.subfolder.clone()),
                self.branch.clone(),
            )
        };

        let local_path = self.local_path.as_ref().map(|p| {
            PathBuf::from(p)
                .join(&self.subfolder)
                .to_string_lossy()
                .to_string()
        });

        let modules_path = self.path.join("modules");
        let module_path = modules_path.join(&self.name);
        generate(GenerateArgs {
            template_path: TemplatePath {
                auto_path,
                git,
                path: local_path,
                branch,
                ..TemplatePath::default()
            },
            destination: Some(modules_path.clone()),
            name: Some(self.name.clone()),
            quiet: !self.verbose,
            verbose: self.verbose,
            no_workspace: true,
            ..GenerateArgs::default()
        })
        .with_context(|| format!("can't generate module '{}'", self.name))?;

        let mut generated = vec![format!("modules/{}", self.name)];

        let sdk_template = module_path.join("sdk");
        if sdk_template.exists() {
            let name = format!("{}-sdk", self.name);
            generated.push(format!("modules/{name}"));
            generate(GenerateArgs {
                template_path: TemplatePath {
                    path: Some(sdk_template.to_string_lossy().to_string()),
                    ..TemplatePath::default()
                },
                destination: Some(modules_path),
                name: Some(name),
                quiet: !self.verbose,
                verbose: self.verbose,
                no_workspace: true,
                ..GenerateArgs::default()
            })
            .with_context(|| format!("can't generate sdk module '{}-sdk'", self.name))?;
            fs::remove_dir_all(sdk_template)
                .with_context(|| format!("can't remove sdk template for module '{}'", self.name))?;
        }

        self.add_modules_to_workspace(generated)?;

        Ok(())
    }

    fn add_modules_to_workspace(&self, generated: Vec<String>) -> anyhow::Result<()> {
        self.add_to_workspace(move |doc| -> anyhow::Result<()> {
            let members = doc["workspace"]["members"]
                .as_array_mut()
                .context("workspace.members is not an array")?;
            members.extend(generated);
            Ok(())
        })?;

        Ok(())
    }

    fn add_to_workspace(
        &self,
        f: impl FnOnce(&mut toml_edit::DocumentMut) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let cargo_toml_path = self.path.join("Cargo.toml");
        let mut doc = fs::read_to_string(&cargo_toml_path)
            .context("can't read Cargo.toml")?
            .parse::<toml_edit::DocumentMut>()
            .context("can't parse workspace Cargo.toml")?;

        f(&mut doc)?;

        fs::write(&cargo_toml_path, doc.to_string()).context("can't write Cargo.toml")?;

        Ok(())
    }
}
