use crate::core::Environment;
use color_eyre::eyre::Result;
use std::io::{Read, Write};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Cli {
    #[arg(value_parser, default_value = "-")]
    input: clio::InputPath,

    #[arg(short, long, value_parser, default_value = "-")]
    output: clio::OutputPath,

    #[arg(short, long)]
    in_place: bool,

    #[command(flatten)]
    color: colorchoice_clap::Color,

    #[cfg(debug_assertions)]
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::TraceLevel>,

    #[cfg(not(debug_assertions))]
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::WarnLevel>,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let mut contents = String::new();
        self.input.clone().open()?.read_to_string(&mut contents)?;
        let environment = Environment::new()?;
        let segments = environment.parse_text(contents)?;
        let result = environment.render(&segments)?;
        let mut output = if self.in_place {
            self.input.path().clone().create()?
        } else {
            self.output.create()?
        };
        output.write_all(result.as_bytes())?;
        Ok(())
    }
}
