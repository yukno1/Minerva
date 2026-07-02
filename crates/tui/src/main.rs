/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

mod client;

use std::io;

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;
use update::update;

use std::io::Write;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Clone)]
enum Message {
    Chunk(String),
    Done,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Message>();
    let tx_clone = tx.clone();

    let mut app = App::new();

    let handle = thread::spawn(move || {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "quit".to_string() {
                let _ = tx_clone.send(Message::Done);
                break;
            }

            let _ = app.client.send_prompt(input);

            app.client
                .receive_stream(|content| {
                    tx_clone.send(Message::Chunk(content)).unwrap();
                })
                .unwrap();
        }
    });

    for message in rx {
        match message {
            Message::Chunk(content) => {
                print!("{}", content);
                io::stdout().flush().unwrap();
            }
            Message::Done => {
                let _ = handle.join();
                break;
            }
        }
    }
}

fn _rust_main() -> Result<()> {
    // Create an application.
    let mut app = App::new();
    // color_eyre::install()?;
    // ratatui::run(|terminal| App::new().run(terminal))

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
