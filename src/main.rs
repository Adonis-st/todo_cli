use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io::{self, Write};

struct Todo {
    title: String,
    completed: bool,
}

enum InputMode {
    Normal,
    Editing,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut todos: Vec<Todo> = vec![];
    let mut selected: usize = 0;
    let mut input_mode = InputMode::Normal;
    let mut input = String::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(size);

            // Todo list
            let items: Vec<ListItem> = todos
                .iter()
                .enumerate()
                .map(|(i, todo)| {
                    let status = if todo.completed { " ✅" } else { " □" };
                    let line = Line::from(Span::raw(format!("{} {}", status, todo.title)));
                    let style = if i == selected {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    };
                    ListItem::new(line).style(style)
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .title("📋 Todos (↑↓ Space a d q)")
                        .borders(Borders::ALL),
                )
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            f.render_widget(list, chunks[0]);

            // Input box
            let prompt = match input_mode {
                InputMode::Normal => Span::raw("Press 'a' to add todo, 'q' to quit"),
                InputMode::Editing => Span::raw(format!("New Todo: {}", input)),
            };

            let input_paragraph = Paragraph::new(Line::from(prompt))
                .block(Block::default().borders(Borders::ALL).title("Input"));

            f.render_widget(input_paragraph, chunks[1]);
        })?;

        // Handle key events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => {
                            input_mode = InputMode::Editing;
                            input.clear();
                        }
                        KeyCode::Char('d') => {
                            if !todos.is_empty() && selected < todos.len() {
                                todos.remove(selected);
                                if selected >= todos.len() && selected > 0 {
                                    selected -= 1;
                                }
                            }
                        }
                        KeyCode::Char(' ') => {
                            if selected < todos.len() {
                                todos[selected].completed = !todos[selected].completed;
                            }
                        }
                        KeyCode::Down => {
                            if selected + 1 < todos.len() {
                                selected += 1;
                            }
                        }
                        KeyCode::Up => {
                            if selected > 0 {
                                selected -= 1;
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Esc => {
                            input_mode = InputMode::Normal;
                            input.clear();
                        }
                        KeyCode::Enter => {
                            if !input.trim().is_empty() {
                                todos.push(Todo {
                                    title: input.trim().to_string(),
                                    completed: false,
                                });
                            }
                            input_mode = InputMode::Normal;
                            input.clear();
                        }
                        KeyCode::Char(c) => {
                            input.push(c);
                        }
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}
