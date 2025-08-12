use crate::core::Environment;
use color_eyre::eyre::Result;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Cli {
    pub file: PathBuf,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        let contents: String = tokio::fs::read_to_string(&self.file).await?;
        let environment = Environment::new();
        let segments = environment.parse_text(contents)?;
        let output = environment.render(&segments)?;
        println!("{}", output);
        Ok(())
    }
}
