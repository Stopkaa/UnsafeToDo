use crate::todo::Todo;
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

const TITLE_OFFSET: usize = 2;
const STATUS_COLUMN: usize = 3;
const DEFAULT_TERMINAL_WIDTH: usize = 80;
const DEFAULT_TERMINAL_HEIGHT: usize = 24;

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

impl From<&Todo> for TodoDisplay {
    fn from(todo: &Todo) -> Self {
        Self {
            id: todo.get_id().to_string(),
            title: todo.get_title(),
            description: todo.get_description(),
            finished: status_emoji(todo),
            priority: todo.get_priority().to_string(),
            created_at: todo
                .get_creation_date()
                .format("%H:%M %d.%m.%Y")
                .to_string(),
            due_date: todo
                .get_due_date()
                .map(|d| d.format("%d.%m.%Y").to_string())
                .unwrap_or_default(),
        }
    }
}

fn status_emoji(todo: &Todo) -> String {
    match (todo.is_finished(), todo.is_overdue()) {
        (true, _) => "✅",
        (false, true) => "❌",
        (false, false) => "⏳",
    }
    .to_string()
}

pub fn display_todo_vector(todos: &[Todo]) {
    // Create display data
    let display_data: Vec<TodoDisplay> = todos.iter().map(TodoDisplay::from).collect();

    let (width, height) = get_terminal_size();
    let mut table = Table::new(display_data);

    // Apply base styling
    table
        .with(Style::rounded())
        .with(LineText::new("Todos", Rows::first()).offset(TITLE_OFFSET))
        .with(Width::wrap(width).priority(Priority::max(true)))
        .with(Width::increase(width))
        .with(Height::limit(height))
        .with(Height::increase(height))
        .modify(Locator::content("Low"), Color::FG_GREEN)
        .modify(Locator::content("Medium"), Color::FG_YELLOW)
        .modify(Locator::content("High"), Color::FG_RED)
        .modify(Columns::single(STATUS_COLUMN), Alignment::center());

    // Apply row-specific styling for overdue todos
    for (i, todo) in todos.iter().enumerate() {
        if todo.is_overdue() && !todo.is_finished() {
            table.modify(Rows::single(i + 1), Color::FG_RED);
        }
    }

    println!("{table}");
}

fn get_terminal_size() -> (usize, usize) {
    terminal_size()
        .map(|(TerminalWidth(w), TerminalHeight(h))| (w as usize, h as usize))
        .unwrap_or((DEFAULT_TERMINAL_WIDTH, DEFAULT_TERMINAL_HEIGHT))
}
