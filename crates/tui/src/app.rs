use std::path::PrefixComponent;
use std::sync::mpsc;
use std::thread;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::client::Client;
use crate::session::Session;
use protocol::DEFAULT_ADDR;

/// Application.
pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Position of cursor in the editor area.
    pub character_index: usize,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<Message>,
    /// should the application exit?
    pub should_quit: bool,
    /// client
    // pub client: Client,
    /// session
    pub session: Session,
    /// Sends prompts to the streaming worker thread. `None` when no server is connected.
    cmd_tx: Option<mpsc::Sender<String>>,
    /// Receives streaming chunks from the worker thread.
    stream_rx: mpsc::Receiver<StreamEvent>,
    /// Whether an agent response is currently streaming in.
    pub streaming: bool,
}

pub enum InputMode {
    Normal,
    Editing,
}

/// Kind of a message in the history.
pub enum MessageKind {
    User,
    Agent,
}

/// A single message in the conversation history.
pub struct Message {
    pub kind: MessageKind,
    pub content: String,
}

/// Events streamed from the worker thread.
enum StreamEvent {
    Chunk(String),
    Done,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let (stream_tx, stream_rx) = mpsc::channel::<StreamEvent>();
        let cmd_tx = match Client::connect(DEFAULT_ADDR) {
            Ok(mut client) => {
                let (cmd_tx, cmd_rx) = mpsc::channel::<String>();
                thread::spawn(move || {
                    loop {
                        let Ok(prompt) = cmd_rx.recv() else {
                            break;
                        };
                        if client.send_prompt(prompt).is_err() {
                            break;
                        }
                        let tx = stream_tx.clone();
                        let _ = client.receive_stream(move |chunk| {
                            let _ = tx.send(StreamEvent::Chunk(chunk));
                        });
                        let _ = stream_tx.send(StreamEvent::Done);
                    }
                });
                Some(cmd_tx)
            }
            Err(_) => None,
        };

        Self {
            input: String::new(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            character_index: 0,
            should_quit: false,
            // client: Client::connect(DEFAULT_ADDR).expect("can't connect to server"),
            session: Session::new(),
            cmd_tx,
            stream_rx,
            streaming: false,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Drain any streamed chunks that have arrived since the last poll and append
    /// them to the in-flight agent message.
    pub fn poll_stream(&mut self) {
        while let Ok(event) = self.stream_rx.try_recv() {
            match event {
                StreamEvent::Chunk(chunk) => {
                    if let Some(last) = self.messages.last_mut() {
                        if matches!(last.kind, MessageKind::Agent) {
                            last.content.push_str(&chunk);
                        }
                    }
                }
                StreamEvent::Done => self.streaming = false,
            }
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
        // todo!("光标中文bug")
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    const fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        let prompt = self.input.clone();
        if prompt.trim() == "/quit" {
            self.should_quit = true;
        } else {
            self.messages.push(Message {
                kind: MessageKind::User,
                content: prompt.clone(),
            });
            // If a server is connected, open an agent message placeholder and kick off
            // streaming; the worker thread will fill it in chunk by chunk.
            if let Some(tx) = &self.cmd_tx {
                self.messages.push(Message {
                    kind: MessageKind::Agent,
                    content: String::new(),
                });
                self.streaming = true;
                let _ = tx.send(prompt);
            }
            self.input.clear();
            self.reset_cursor();
        }
    }

    pub fn run(&mut self, key: KeyEvent) {
        // loop {
        //     terminal.draw(|frame| render(&mut self, frame))?;

        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    self.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                _ => {}
            },
            InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => self.submit_message(),
                KeyCode::Char(to_insert) => self.enter_char(to_insert),
                KeyCode::Backspace => self.delete_char(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                // KeyCode::Esc => self.input_mode = InputMode::Normal,
                _ => {}
            },
            InputMode::Editing => {}
        }

        // }
    }
}
