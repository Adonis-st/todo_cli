use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    title: String,
    completed: bool,
}

enum InputMode {
    Normal,
    Editing,
}

fn load_todos() -> Vec<Todo> {
    if let Ok(content) = fs::read_to_string("todos.json") {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        vec![]
    }
}

fn save_todos(todos: &Vec<Todo>) {
    if let Ok(json) = serde_json::to_string_pretty(todos) {
        let _ = fs::write("todos.json", json);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    let mut todos = load_todos();
    let mut selected = 0;
    let mut input = String::new();
    let mut input_mode = InputMode::Normal;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(size);

            let items: Vec<ListItem> = todos
                .iter()
                .enumerate()
                .map(|(i, todo)| {
                    let symbol = if todo.completed { " ✅" } else { " □" };
                    let content = format!("{} {}", symbol, todo.title);
                    if i == selected {
                        ListItem::new(Spans::from(vec![Span::styled(
                            content,
                            Style::default().add_modifier(Modifier::REVERSED),
                        )]))
                    } else {
                        ListItem::new(content)
                    }
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Todos").borders(Borders::ALL));

            let input_box = match input_mode {
                InputMode::Editing => Paragraph::new(input.as_ref())
                    .block(Block::default().title("New Todo").borders(Borders::ALL)),
                InputMode::Normal => Paragraph::new("Press 'a' to add, ⬆⬇ to navigate, [space] to toggle, [d] to delete, [q] to quit.")
                    .block(Block::default().borders(Borders::ALL)),
            };

            f.render_widget(list, chunks[0]);
            f.render_widget(input_box, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => {
                            save_todos(&todos);
                            return Ok(());
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
                        KeyCode::Char(' ') => {
                            if let Some(todo) = todos.get_mut(selected) {
                                todo.completed = !todo.completed;
                            }
                        }
                        KeyCode::Char('d') => {
                            if selected < todos.len() {
                                todos.remove(selected);
                                if selected >= todos.len() && !todos.is_empty() {
                                    selected = todos.len() - 1;
                                }
                            }
                        }
                        KeyCode::Char('a') => {
                            input_mode = InputMode::Editing;
                            input.clear();
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            if !input.trim().is_empty() {
                                todos.push(Todo {
                                    title: input.trim().to_string(),
                                    completed: false,
                                });
                                input.clear();
                                input_mode = InputMode::Normal;
                            }
                        }
                        KeyCode::Esc => {
                            input_mode = InputMode::Normal;
                            input.clear();
                        }
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        KeyCode::Char(c) => {
                            input.push(c);
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
