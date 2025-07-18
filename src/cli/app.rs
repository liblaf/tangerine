use std::path::PathBuf;

use color_eyre::eyre::Result;

use crate::core::Profile;

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct App {
    #[arg(long, default_value = "home/")]
    source_dir: PathBuf,
    #[arg(long, default_value = "modules/")]
    modules_dir: PathBuf,
    #[arg(default_value = "profile.yaml")]
    profile: PathBuf,
}

impl App {
    pub async fn run(&self) -> Result<()> {
        let profile = Profile::load(&self.profile, &self.modules_dir).await?;
        profile.merge_modules(&self.source_dir).await?;
        Ok(())
    }
}
