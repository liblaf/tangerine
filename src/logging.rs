use color_eyre::eyre::Result;

pub fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    color_eyre::install()?;
    Ok(())
}
