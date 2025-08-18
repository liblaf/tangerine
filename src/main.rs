mod cli;
mod core;
mod utils;

pub use self::cli::Cli;
pub use self::core::AUTHOR;
pub use self::core::Environment;
pub use self::core::PATTERN_END;
pub use self::core::PATTERN_START;
pub use self::core::Template;
use clap::Parser;
use color_eyre::eyre::Result;

shadow_rs::shadow!(build);

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    cli.invoke()
}
