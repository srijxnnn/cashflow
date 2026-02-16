use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::app::{App, InputMode};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    render_search_bar(f, app, chunks[0]);
    render_table(f, app, chunks[1]);
}

fn render_search_bar(f: &mut Frame, app: &App, area: Rect) {
    let (style, title) = if app.input_mode == InputMode::Search {
        (
            Style::default().fg(Color::Yellow),
            " Search (Esc to cancel) ",
        )
    } else {
        (Style::default().fg(Color::DarkGray), " Search (/ to search) ")
    };

    let mut spans = vec![Span::raw(&app.search_query)];
    if app.input_mode == InputMode::Search {
        spans.push(Span::styled("_", Style::default().fg(Color::Yellow)));
    }
    if app.show_recurring_only {
        spans.push(Span::styled(
            " [Recurring Only]",
            Style::default().fg(Color::Magenta),
        ));
    }

    let search = Paragraph::new(Line::from(spans)).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(style),
    );

    f.render_widget(search, area);
}

fn render_table(f: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["ID", "Date", "Amount", "Category", "Description", "Recurring"]
        .iter()
        .map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
        });
    let header = Row::new(header_cells).height(1);

    let rows: Vec<Row> = app
        .filtered_indices
        .iter()
        .map(|&i| {
            let expense = &app.expenses[i];
            let recurring_str = if expense.is_recurring {
                expense
                    .recurrence
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| "Yes".to_string())
            } else {
                String::from("-")
            };
            Row::new(vec![
                Cell::from(expense.id.to_string()),
                Cell::from(expense.date.format("%Y-%m-%d").to_string()),
                Cell::from(app.fmt(expense.amount))
                    .style(Style::default().fg(Color::Green)),
                Cell::from(expense.category.to_string()),
                Cell::from(expense.description.clone()),
                Cell::from(recurring_str),
            ])
        })
        .collect();

    let selected_style = Style::default()
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD);

    let widths = [
        Constraint::Length(6),
        Constraint::Length(12),
        Constraint::Length(12),
        Constraint::Length(15),
        Constraint::Min(20),
        Constraint::Length(10),
    ];

    let count = app.filtered_indices.len();
    let title = if app.search_query.is_empty() && !app.show_recurring_only {
        format!(" Expenses ({}) ", count)
    } else {
        format!(" Filtered ({}) ", count)
    };

    let hint = if app.input_mode == InputMode::ConfirmDelete {
        " Press y to confirm delete, n to cancel "
    } else {
        " a:add  e:edit  d:delete  r:recurring  /:search  x:export "
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(title)
                .title_bottom(Line::from(hint).centered())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .row_highlight_style(selected_style)
        .highlight_symbol(">> ");

    let mut state = TableState::default();
    if !app.filtered_indices.is_empty() {
        state.select(Some(app.expense_table_index));
    }

    f.render_stateful_widget(table, area, &mut state);
}
