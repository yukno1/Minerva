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

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let tx_clone = tx.clone();

    let mut app = App::new();

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input == "quit".to_string() {
                break;
            }

            println!("> {input}");
            app.client.send_prompt(input);

            app.client
                .receive_stream(|content| {
                    tx_clone.send(content).unwrap();
                })
                .unwrap();
        }
    });

    for content in rx {
        print!("{}", content);
        io::stdout().flush().unwrap();
    }
}

fn rust_main() -> Result<()> {
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
