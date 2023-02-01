use anyhow::{Context, Result};
use toy_helix::{application::Application, config::Config};

fn main() -> Result<()> {
    let exit_code = run()?;
    std::process::exit(exit_code);
}

#[tokio::main]
async fn run() -> Result<i32> {
    // TODO: init subscriber

    let config = Config::default();
    let mut app = Application::new(config).context("unable to create new application")?;

    let exit_code = app.run().await?;

    Ok(exit_code)
}
