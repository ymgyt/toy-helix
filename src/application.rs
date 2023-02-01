use std::io::stdout;

use anyhow::{Error, Result};
use crossterm::{execute, terminal};

use crate::{config::Config, tui::backend::crossterm::CrosstermBackend};

type Terminal = crate::tui::terminal::Terminal<CrosstermBackend<std::io::Stdout>>;

pub struct Application {
    config: Config,
    terminal: Terminal,
}

fn restore_term() -> Result<(), Error> {
    let mut stdout = stdout();
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

impl Application {
    pub fn new(config: Config) -> Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;

        // TODO: new editor

        Ok(Self { config, terminal })
    }

    pub async fn run(&mut self) -> Result<i32, Error> {
        self.claim_term().await?;

        // TODO: into event_loop();

        restore_term()?;

        // TODO: return self.editor.exit_code

        Ok(0)
    }

    async fn claim_term(&mut self) -> Result<(), Error> {
        terminal::enable_raw_mode()?;

        let mut stdout = stdout();
        execute!(stdout, terminal::EnterAlternateScreen,)?;
        execute!(stdout, terminal::Clear(terminal::ClearType::All),)?;

        Ok(())
    }
}
