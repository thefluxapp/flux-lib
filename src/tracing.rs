use anyhow::Error;

pub fn init() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    Ok(())
}
