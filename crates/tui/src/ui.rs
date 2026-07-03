use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Wrap};

use crate::app::{App, InputMode, Message, MessageKind};

/// Build the [`Line`]s for a single message, with a role-labelled first line and
/// a streaming cursor appended to the last line when `streaming_cursor` is set.
fn message_lines(msg: &Message, streaming_cursor: bool) -> Vec<Line<'static>> {
    let (label, label_color) = match msg.kind {
        MessageKind::User => ("you: ", Color::Cyan),
        MessageKind::Agent => ("agent: ", Color::Green),
    };

    let segments: Vec<&str> = msg.content.split('\n').collect();
    let mut lines: Vec<Line> = segments
        .iter()
        .enumerate()
        .map(|(i, segment)| {
            let mut spans: Vec<Span> = Vec::new();
            if i == 0 {
                spans.push(Span::styled(label, Style::default().fg(label_color)));
            }
            spans.push(Span::raw((*segment).to_string()));
            Line::from(spans)
        })
        .collect();

    if streaming_cursor {
        if let Some(last) = lines.last_mut() {
            last.spans
                .push(Span::styled("▌", Style::default().fg(Color::Yellow)));
        }
    }

    lines
}

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(3),
    ]);
    let [help_area, messages_area, input_area] = frame.area().layout(&layout);

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "e".bold(),
                " to start editing.".bold(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to record the message".into(),
            ],
            Style::default(),
        ),
    };
    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Input"));
    frame.render_widget(input, input_area);
    match app.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        InputMode::Normal => {}

        // Make the cursor visible and ask ratatui to put it at the specified coordinates after
        // rendering
        #[expect(clippy::cast_possible_truncation)]
        InputMode::Editing => frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position can be controlled via the left and right arrow key
            input_area.x + app.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        )),
    }

    let messages_len = app.messages.len();
    let mut all_lines: Vec<Line> = Vec::new();
    for (i, m) in app.messages.iter().enumerate() {
        let is_last = i + 1 == messages_len;
        let show_cursor = is_last && app.streaming;
        all_lines.extend(message_lines(m, show_cursor));
    }

    // Wrap long lines inside the bordered area and auto-scroll so the most
    // recent content (where streaming lands) stays visible.
    let inner_width = messages_area.width.saturating_sub(2) as usize;
    let wrap_width = inner_width.max(1);
    let total_lines: usize = all_lines
        .iter()
        .map(|line| line.width().div_ceil(wrap_width).max(1))
        .sum();
    let visible_height = messages_area.height.saturating_sub(2) as usize;
    #[expect(clippy::cast_possible_truncation)]
    let scroll = total_lines.saturating_sub(visible_height) as u16;

    let messages = Paragraph::new(all_lines)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0))
        .block(Block::bordered().title("Messages"));
    frame.render_widget(messages, messages_area);
}
