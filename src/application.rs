use arc_swap::{access::Map, ArcSwap};
use std::{io::stdout, sync::Arc};

use anyhow::{Context, Error, Result};
use crossterm::{event::Event as CrosstermEvent, execute, terminal};
use futures_util::Stream;
use signal_hook::consts::signal;
use signal_hook_tokio::Signals;

use crate::{
    config::Config,
    term::{
        args::Args,
        compositor::{self, Compositor},
        keymap::Keymaps,
        ui::editor::EditorView,
    },
    tui::backend::crossterm::CrosstermBackend,
    view::{
        editor::{Action, Editor},
        graphics::CursorKind,
    },
};

type Terminal = crate::tui::terminal::Terminal<CrosstermBackend<std::io::Stdout>>;

pub struct Application {
    compositor: Compositor,
    terminal: Terminal,
    pub editor: Editor,

    config: Arc<ArcSwap<Config>>,

    signals: Signals,
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
        let mut compositor = Compositor::new(area);

        let config = Arc::new(ArcSwap::from_pointee(config));
        let mut editor = Editor::new(
            area,
            Arc::new(Map::new(Arc::clone(&config), |config: &Config| &config.editor)),
        );

        let keys = Box::new(Map::new(Arc::clone(&config), |config: &Config| &config.keys));
        let mut editor_view = Box::new(EditorView::new(Keymaps::new(keys)));
        compositor.push(editor_view);

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
                        // TODO: handle --vsplit, --hsplit
                        let action = {
                            if i == 0 {
                                Action::VerticalSplit
                            } else {
                                Action::Load
                            }
                        };
                        let doc_id = editor
                            .open(&file, action)
                            .context(format!("open '{}'", file.to_string_lossy()))?;
                    }
                }
            }
        }

        let signals = Signals::new([signal::SIGTSTP]).context("build signal handler")?;

        Ok(Self {
            compositor,
            editor,
            config,
            terminal,
            signals,
        })
    }

    pub async fn run<S>(&mut self, input_stream: &mut S) -> Result<i32, Error>
    where
        S: Stream<Item = crossterm::Result<crossterm::event::Event>> + Unpin,
    {
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

        self.event_loop(input_stream).await;

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

    pub async fn event_loop<S>(&mut self, input_stream: &mut S)
    where
        S: Stream<Item = crossterm::Result<crossterm::event::Event>> + Unpin,
    {
        self.render().await;

        loop {
            if !self.event_loop_until_idle(input_stream).await {
                break;
            }
        }
    }

    pub async fn event_loop_until_idle<S>(&mut self, input_stream: &mut S) -> bool
    where
        S: Stream<Item = crossterm::Result<crossterm::event::Event>> + Unpin,
    {
        loop {
            // TODO: handle should_close

            use futures_util::StreamExt;

            tokio::select! {
                biased;

                Some(event) = input_stream.next() => {
                    self.handle_terminal_events(event).await;
                }
            }
        }
    }

    async fn render(&mut self) {
        let mut cx = compositor::Context {
            editor: &mut self.editor,
        };

        let area = self.terminal.autoresize().expect("Unable to determine terminal size");

        let surface = self.terminal.current_buffer_mut();

        self.compositor.render(area, surface, &mut cx);

        // TODO: handle cursor

        self.terminal.draw(None, CursorKind::Block).unwrap();
    }

    pub async fn handle_terminal_events(&mut self, event: Result<CrosstermEvent, crossterm::ErrorKind>) {
        let mut cx = compositor::Context {
            editor: &mut self.editor,
        };

        let should_redraw = match event.unwrap() {
            CrosstermEvent::Resize(_, _) => {
                todo!("handle resize event");
            }
            CrosstermEvent::Key(crossterm::event::KeyEvent {
                kind: crossterm::event::KeyEventKind::Release,
                ..
            }) => false,
            event => self.compositor.handle_event(&event.into(), &mut cx),
        };

        // TODO: care should clouse
        if should_redraw {
            self.render().await;
        }
    }
}
