use crate::todo::Todo;
use crate::todo::TodoList;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::Alignment;
use tabled::settings::Color;
use tabled::settings::Style;
use tabled::settings::location::Locator;
use tabled::settings::object::Columns;
use tabled::settings::object::Rows;
use tabled::settings::peaker::Priority;
use tabled::settings::style::LineText;
use tabled::settings::{Height, Width};
use terminal_size::{Height as TerminalHeight, Width as TerminalWidth, terminal_size};

#[derive(Tabled)]
struct TodoDisplay {
    id: String,
    title: String,
    description: String,
    finished: String,
    priority: String,
    created_at: String,
    due_date: String,
}

impl TodoDisplay {
    pub fn from(todo: &Todo) -> Self {
        let id = todo.get_id().to_string();
        let title = todo.get_title();
        let description = todo.get_description();
        let finished = (if todo.is_finished() { 
            "✅" 
        }
        else if todo.is_overdue() {
            "❌"
        }
        else { 
            "⏳" 
        }).to_string();
        let priority = todo.get_priority().to_string();
        let created_at = todo
            .get_creation_date()
            .format("%H:%M %d.%m.%Y")
            .to_string();
        let due_date = todo
            .get_due_date()
            .map(|date| date.format("%H:%M %d.%m.%Y").to_string())
            .unwrap_or_default();

        Self {
            id,
            title,
            description,
            finished,
            priority,
            created_at,
            due_date,
        }
    }
}

pub fn display_todo_list(todo_list: &TodoList) {
    let mut todos = vec![];
    for todo in todo_list.todos.iter() {
        todos.push(TodoDisplay::from(todo));
    }
    let (width, height) = get_terminal_size();
    let mut table = Table::new(todos);
    table
        .with(Style::rounded())
        .with(LineText::new("Todos", Rows::first()).offset(2))
        .with(Width::wrap(width).priority(Priority::max(true)))
        .with(Width::increase(width))
        .with(Height::limit(height))
        .with(Height::increase(height))
        .modify(Locator::content("Low"), Color::FG_GREEN)
        .modify(Locator::content("Medium"), Color::FG_YELLOW)
        .modify(Locator::content("High"), Color::FG_RED)
        .modify(Columns::single(3), Alignment::center());

    for (i, todo) in todo_list.todos.iter().enumerate() {
        let is_overdue = todo.is_overdue();
        let finished = todo.is_finished();
        if is_overdue && !finished {
            table.modify(Rows::single(i + 1), Color::FG_RED);
        }
    }
    println!("{table}");
}

fn get_terminal_size() -> (usize, usize) {
    let (TerminalWidth(width), TerminalHeight(height)) =
        terminal_size().expect("failed to obtain a terminal size");

    (width as usize, height as usize)
}
