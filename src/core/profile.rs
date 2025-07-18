use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use color_eyre::eyre::{self, Context};

use super::Module;

#[derive(Debug, serde::Deserialize)]
pub struct ProfileConfig {
    #[serde(default)]
    pub inherits: Vec<PathBuf>,
    #[serde(default)]
    pub modules: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct Profile {
    pub modules: Vec<Module>,
}

impl Profile {
    #[tracing::instrument(
        fields(
            file = %file.as_ref().display(),
            modules_dir = %modules_dir.as_ref().display()
        )
    )]
    pub async fn load(file: impl AsRef<Path>, modules_dir: impl AsRef<Path>) -> Result<Self> {
        let file = file.as_ref();
        Self::_load(file, modules_dir)
            .await
            .wrap_err_with(|| format!("Failed to load profile: {}", file.display()))
    }

    async fn _load(file: impl AsRef<Path>, modules_dir: impl AsRef<Path>) -> Result<Self> {
        let file = file.as_ref();
        let modules_dir = modules_dir.as_ref();
        let config: Vec<u8> = tokio::fs::read(&file).await?;
        let config: ProfileConfig = serde_yml::from_slice(&config)?;
        let inherits = futures::future::try_join_all(
            config
                .inherits
                .iter()
                .map(|inherit_file| file.parent().unwrap().join(inherit_file))
                .map(async |inherit_file| Self::load(&inherit_file, modules_dir).await),
        )
        .await?;
        let modules = inherits.into_iter().flat_map(|profile| profile.modules);
        let modules = modules.chain(
            futures::future::try_join_all(
                config
                    .modules
                    .into_iter()
                    .map(async |module_path| Module::load(modules_dir.join(module_path)).await),
            )
            .await?,
        );
        Ok(Profile {
            modules: modules.collect(),
        })
    }

    pub async fn merge_modules(self, dest: impl AsRef<Path>) -> Result<()> {
        let dest = dest.as_ref();
        let mut cmd = tokio::process::Command::new("rsync");
        let cmd = cmd
            .args([
                "--info=PROGRESS2",
                "--archive",
                "--delete",
                "--force",
                "--exclude=.packages.yaml",
                "--exclude=.packages.yaml.tmpl",
            ])
            .args(self.modules.iter().map(
                // ensure a trailing slash on the sources
                // ref: <https://download.samba.org/pub/rsync/rsync.1>
                // > A trailing slash on the source changes this behavior to
                // > avoid creating an additional directory level at the
                // > destination. You can think of a trailing / on a source as
                // > meaning "copy the contents of this directory" as opposed to
                // > "copy the directory by name", but in both cases the
                // > attributes of the containing directory are transferred to
                // > the containing directory on the destination.
                |module| module.path.join(""),
            ))
            .arg(dest.as_os_str());
        let status = cmd.status().await?;
        eyre::ensure!(status.success());
        Ok(())
    }
}
