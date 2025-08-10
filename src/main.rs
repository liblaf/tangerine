mod cli;
mod core;
mod logging;

pub use self::cli::Cli;
pub use self::core::Environment;
pub use self::core::PATTERN_END;
pub use self::core::PATTERN_START;
pub use self::core::Template;
use clap::Parser;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    crate::logging::init()?;
    let cli: Cli = Cli::parse();
    cli.run().await
}
