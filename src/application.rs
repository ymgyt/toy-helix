use arc_swap::{access::Map, ArcSwap};
use std::{io::stdout, sync::Arc};

use anyhow::{Context, Error, Result};
use crossterm::{execute, terminal};

use crate::{
    config::Config,
    term::{
        args::Args,
        compositor::{self, Compositor},
    },
    tui::backend::crossterm::CrosstermBackend,
    view::editor::{Action, Editor},
};

type Terminal = crate::tui::terminal::Terminal<CrosstermBackend<std::io::Stdout>>;

pub struct Application {
    compositor: Compositor,
    terminal: Terminal,
    pub editor: Editor,

    config: Arc<ArcSwap<Config>>,
}

fn restore_term() -> Result<(), Error> {
    let mut stdout = stdout();
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

impl Application {
    pub fn new(args: Args, config: Config) -> Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        let area = terminal.size().expect("Couldn't get terminal size");
        let compositor = Compositor::new(area);

        let config = Arc::new(ArcSwap::from_pointee(config));
        let mut editor = Editor::new(
            area,
            Arc::new(Map::new(Arc::clone(&config), |config: &Config| &config.editor)),
        );

        if !args.files.is_empty() {
            let first = &args.files[0].0; // we know it's not empty
            if first.is_dir() {
                anyhow::bail!("directory handling not implemented");
            } else {
                let nr_of_files = args.files.len();
                for (i, (file, pos)) in args.files.into_iter().enumerate() {
                    if file.is_dir() {
                        anyhow::bail!(
                            "expected a path to file, found a directory. (to open a directory pass it as first argument)"
                        )
                    } else {
                        let action = Action::Load;
                        let doc_id = editor
                            .open(&file, action)
                            .context(format!("open '{}'", file.to_string_lossy()))?;
                    }
                }
            }
        }

        Ok(Self {
            compositor,
            editor,
            config,
            terminal,
        })
    }

    pub async fn run(&mut self) -> Result<i32, Error> {
        self.claim_term().await?;

        // Exit the alternate screen and disable raw mode before panicking
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            // We can't handle errors properly inside this closure.
            // And it's probably not a good idea to `unwrap()` inside a panic handler.
            // So we just ignore the `Result`.
            let _ = restore_term();
            hook(info);
        }));

        self.event_loop().await;

        restore_term()?;

        Ok(self.editor.exit_code)
    }

    async fn claim_term(&mut self) -> Result<(), Error> {
        terminal::enable_raw_mode()?;

        let mut stdout = stdout();
        execute!(stdout, terminal::EnterAlternateScreen,)?;
        execute!(stdout, terminal::Clear(terminal::ClearType::All),)?;

        Ok(())
    }

    pub async fn event_loop(&mut self) {
        self.render().await;
    }

    async fn render(&mut self) {
        let mut cx = compositor::Context {
            editor: &mut self.editor,
        };

        let area = self.terminal.autoresize().expect("Unable to determine terminal size");

        let surface = self.terminal.current_buffer_mut();

        self.compositor.render(area, surface, &mut cx);
    }
}
