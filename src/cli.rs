use crate::core::Environment;
use color_eyre::Result;
use std::io::{Read, Write};

#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Cli {
    #[command(flatten)]
    shared: grapes::cli::Shared,

    #[arg(value_parser, default_value = "-")]
    input: clio::InputPath,

    #[arg(short, long, value_parser, default_value = "-")]
    output: clio::OutputPath,

    #[arg(short, long)]
    in_place: bool,
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        match self.shared.execute::<Cli>()? {
            grapes::cli::ExecuteResult::EarlyExit => return Ok(()),
            grapes::cli::ExecuteResult::Success => {}
        }
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
        {
            let _span = tracing::trace_span!("write_all");
            output.write_all(result.as_bytes())?;
        }
        Ok(())
    }
}
