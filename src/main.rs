use anyhow::{Context, Result};
use crossterm::event::EventStream;
use toy_helix::{application::Application, config::Config, term::args::Args};

fn main() -> Result<()> {
    let exit_code = run()?;
    std::process::exit(exit_code);
}

#[tokio::main]
async fn run() -> Result<i32> {
    let args = Args::parse_args()?;

    let _guard = init_tracing();

    tracing::info!("Starting...");

    let config = Config::default();
    let mut app = Application::new(args, config).context("unable to create new application")?;

    let exit_code = app.run(&mut EventStream::new()).await?;

    Ok(exit_code)
}

fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let appender = tracing_appender::rolling::never("/tmp", "toy-helix.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(appender);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .with_writer(non_blocking)
        .init();

    guard
}
