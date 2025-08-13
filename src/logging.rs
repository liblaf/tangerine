use color_eyre::eyre::Result;

pub fn init<L>(verbosity: clap_verbosity_flag::Verbosity<L>) -> Result<()>
where
    L: clap_verbosity_flag::LogLevel,
{
    let ansi = supports_color::on_cached(supports_color::Stream::Stderr)
        .map(|c| c.has_basic)
        .unwrap_or(false);
    let builder = tracing_subscriber::fmt()
        .with_ansi(ansi)
        .with_max_level(verbosity.tracing_level())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_timer(tracing_subscriber::fmt::time::Uptime::default())
        .with_thread_names(true)
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
