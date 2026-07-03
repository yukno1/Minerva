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
mod session;

use std::io;

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;
use update::update;

use std::io::Write;

use crate::client::Client;
use crate::session::Session;
use protocol::DEFAULT_ADDR;

fn main() -> Result<()> {
    let mut app = App::new();
    // let mut client = Client::connect(DEFAULT_ADDR).expect("can't connect to server");
    // let mut session = Session::new();

    // loop {
    //     print!("> ");
    //     io::stdout().flush().unwrap();
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();

    //     if input.trim() == "quit" {
    //         break;
    //     }
    //     session.add_userinput(input.clone());

    //     let _ = client.send_prompt(input);

    //     print!("> ");
    //     io::stdout().flush().unwrap();
    //     session.assign_agentoutput_block();
    //     client
    //         .receive_stream(|content| {
    //             session.push_agentoutput_chunk(content.clone());
    //             print!("{}", content);
    //             io::stdout().flush().unwrap();
    //         })
    //         .unwrap();
    // }

    // color_eyre::install()?;
    // ratatui::run(|terminal| App::new().run(terminal))

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(100);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Pull any streamed chunks that arrived since the last iteration and fold
        // them into the in-flight agent message before rendering.
        app.poll_stream();
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
