use color_eyre::eyre::Result;

pub fn init<L>(verbosity: clap_verbosity_flag::Verbosity<L>) -> Result<()>
where
    L: clap_verbosity_flag::LogLevel,
{
    let builder = tracing_subscriber::fmt()
        .with_max_level(verbosity.tracing_level())
        .with_timer(tracing_subscriber::fmt::time::Uptime::default())
        .with_writer(std::io::stderr);
    if let Some(level) = verbosity.tracing_level()
        && level >= tracing::Level::DEBUG
    {
        builder.pretty().init();
    } else {
        builder.init();
    }
    color_eyre::install()?;
    Ok(())
}
