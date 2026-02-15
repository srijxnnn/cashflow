use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, FormField, FormState, InputMode};
use crate::model::{Category, Recurrence};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    if app.input_mode != InputMode::AddForm && app.input_mode != InputMode::EditForm {
        return;
    }

    let popup_area = centered_rect(60, 70, area);
    f.render_widget(Clear, popup_area);

    let title = if app.input_mode == InputMode::EditForm {
        " Edit Expense "
    } else {
        " Add Expense "
    };

    let block = Block::default()
        .title(title)
        .title_bottom(Line::from(" Tab:next  Shift+Tab:prev  Enter:save  Esc:cancel ").centered())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let fields = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    render_field(f, "Amount", &app.form.amount_input, app.form.active_field == FormField::Amount, fields[0]);
    render_category_field(f, &app.form, fields[1]);
    render_field(f, "Description", &app.form.description_input, app.form.active_field == FormField::Description, fields[2]);
    render_field(f, "Date (YYYY-MM-DD)", &app.form.date_input, app.form.active_field == FormField::Date, fields[3]);
    render_toggle_field(f, "Recurring", app.form.is_recurring, app.form.active_field == FormField::Recurring, fields[4]);
    render_recurrence_field(f, &app.form, fields[5]);

    render_validation(f, &app.form, fields[6]);
}

fn render_field(f: &mut Frame, label: &str, value: &str, active: bool, area: Rect) {
    let style = if active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let display = if active {
        format!("{}_", value)
    } else {
        value.to_string()
    };

    let paragraph = Paragraph::new(display).block(
        Block::default()
            .title(format!(" {} ", label))
            .borders(Borders::ALL)
            .border_style(style),
    );

    f.render_widget(paragraph, area);
}

fn render_category_field(f: &mut Frame, form: &FormState, area: Rect) {
    let active = form.active_field == FormField::Category;
    let style = if active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let names = Category::all_display_names();
    let selected = names.get(form.category_index).unwrap_or(&"Other");

    let display = if active {
        let mut parts = Vec::new();
        parts.push(Span::styled("< ", Style::default().fg(Color::Yellow)));
        parts.push(Span::styled(
            selected.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));
        parts.push(Span::styled(" >", Style::default().fg(Color::Yellow)));
        if form.category_index == 9 {
            parts.push(Span::raw(format!(" ({})", form.custom_category)));
        }
        Line::from(parts)
    } else {
        let mut text = selected.to_string();
        if form.category_index == 9 && !form.custom_category.is_empty() {
            text = format!("Other({})", form.custom_category);
        }
        Line::from(text)
    };

    let hint = if active {
        " Category (←/→ to change) "
    } else {
        " Category "
    };

    let paragraph = Paragraph::new(display).block(
        Block::default()
            .title(hint)
            .borders(Borders::ALL)
            .border_style(style),
    );

    f.render_widget(paragraph, area);
}

fn render_toggle_field(f: &mut Frame, label: &str, value: bool, active: bool, area: Rect) {
    let style = if active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let display = if value {
        Span::styled("Yes", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("No", Style::default().fg(Color::Red))
    };

    let hint = if active {
        format!(" {} (Space to toggle) ", label)
    } else {
        format!(" {} ", label)
    };

    let paragraph = Paragraph::new(Line::from(display)).block(
        Block::default()
            .title(hint)
            .borders(Borders::ALL)
            .border_style(style),
    );

    f.render_widget(paragraph, area);
}

fn render_recurrence_field(f: &mut Frame, form: &FormState, area: Rect) {
    let active = form.active_field == FormField::RecurrenceType;
    let style = if active && form.is_recurring {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let display = if !form.is_recurring {
        Line::from(Span::styled("N/A", Style::default().fg(Color::DarkGray)))
    } else {
        let names = Recurrence::all_display_names();
        let selected = names.get(form.recurrence_index).unwrap_or(&"Monthly");
        if active {
            Line::from(vec![
                Span::styled("< ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    selected.to_string(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" >", Style::default().fg(Color::Yellow)),
            ])
        } else {
            Line::from(selected.to_string())
        }
    };

    let hint = if active && form.is_recurring {
        " Recurrence (←/→ to change) "
    } else {
        " Recurrence "
    };

    let paragraph = Paragraph::new(display).block(
        Block::default()
            .title(hint)
            .borders(Borders::ALL)
            .border_style(style),
    );

    f.render_widget(paragraph, area);
}

fn render_validation(f: &mut Frame, form: &FormState, area: Rect) {
    let mut errors = Vec::new();

    if !form.amount_input.is_empty() {
        if form.amount_input.parse::<f64>().is_err() {
            errors.push("Amount must be a valid number");
        } else if form.amount_input.parse::<f64>().unwrap_or(0.0) <= 0.0 {
            errors.push("Amount must be positive");
        }
    }

    if !form.date_input.is_empty()
        && chrono::NaiveDate::parse_from_str(&form.date_input, "%Y-%m-%d").is_err()
    {
        errors.push("Date must be YYYY-MM-DD format");
    }

    if !errors.is_empty() {
        let text: Vec<Line> = errors
            .iter()
            .map(|e| {
                Line::from(Span::styled(
                    format!("  * {}", e),
                    Style::default().fg(Color::Red),
                ))
            })
            .collect();
        let paragraph = Paragraph::new(text);
        f.render_widget(paragraph, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
