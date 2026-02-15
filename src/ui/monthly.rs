use chrono::Month;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::app::App;
use crate::model::Category;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
        .split(area);

    render_month_selector(f, app, chunks[0]);
    render_category_breakdown(f, app, chunks[1]);
    render_total_summary(f, app, chunks[2]);
}

fn month_name(month: u32) -> &'static str {
    match Month::try_from(month as u8) {
        Ok(m) => match m {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        },
        Err(_) => "Unknown",
    }
}

fn render_month_selector(f: &mut Frame, app: &App, area: Rect) {
    let text = Line::from(vec![
        Span::styled(
            " < ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{} {}", month_name(app.selected_month), app.selected_year),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " > ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let selector = Paragraph::new(text).centered().block(
        Block::default()
            .title(" Month (←/→ to navigate) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(selector, area);
}

fn render_category_breakdown(f: &mut Frame, app: &App, area: Rect) {
    let spending = app.spending_by_category(app.selected_year, app.selected_month);

    if spending.is_empty() {
        let empty = Paragraph::new("No expenses for this month")
            .centered()
            .style(Style::default().fg(Color::DarkGray))
            .block(
                Block::default()
                    .title(" Category Breakdown ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
        f.render_widget(empty, area);
        return;
    }

    let num_cats = spending.len().min(10);
    let mut constraints: Vec<Constraint> = spending
        .iter()
        .take(num_cats)
        .map(|_| Constraint::Length(2))
        .collect();
    constraints.push(Constraint::Min(0));

    let inner_block = Block::default()
        .title(" Category Breakdown ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner_area = inner_block.inner(area);
    f.render_widget(inner_block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    let colors = [
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Red,
        Color::Magenta,
        Color::Cyan,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightRed,
    ];

    let max_spending = spending.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max);

    for (i, (cat_name, amount)) in spending.iter().take(num_cats).enumerate() {
        let cat_enum = category_from_name(cat_name);
        let budget = cat_enum
            .as_ref()
            .and_then(|c| app.budget_for_category(c));

        let (ratio, label) = if let Some(limit) = budget {
            let r = (amount / limit).min(1.0);
            (r, format!("{}: ${:.0} / ${:.0}", cat_name, amount, limit))
        } else {
            let r = if max_spending > 0.0 {
                amount / max_spending
            } else {
                0.0
            };
            (r, format!("{}: ${:.2}", cat_name, amount))
        };

        let color = if budget.is_some() && ratio > 0.9 {
            Color::Red
        } else {
            colors[i % colors.len()]
        };

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(color))
            .label(Span::styled(label, Style::default().fg(Color::White)))
            .ratio(ratio.min(1.0));

        f.render_widget(gauge, rows[i]);
    }
}

fn render_total_summary(f: &mut Frame, app: &App, area: Rect) {
    let total = app.total_for_month(app.selected_year, app.selected_month);
    let total_budget: f64 = app.budgets.iter().map(|b| b.monthly_limit).sum();

    let text = if total_budget > 0.0 {
        let remaining = total_budget - total;
        let status = if remaining >= 0.0 {
            Span::styled(
                format!("${:.2} remaining", remaining),
                Style::default().fg(Color::Green),
            )
        } else {
            Span::styled(
                format!("${:.2} over budget!", remaining.abs()),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )
        };
        Line::from(vec![
            Span::styled(
                format!("Total: ${:.2}", total),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  |  "),
            Span::styled(
                format!("Budget: ${:.2}", total_budget),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw("  |  "),
            status,
        ])
    } else {
        Line::from(Span::styled(
            format!("Total Spent: ${:.2}", total),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
    };

    let summary = Paragraph::new(text).centered().block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(summary, area);
}

fn category_from_name(name: &str) -> Option<Category> {
    match name {
        "Food" => Some(Category::Food),
        "Transport" => Some(Category::Transport),
        "Rent" => Some(Category::Rent),
        "Utilities" => Some(Category::Utilities),
        "Entertainment" => Some(Category::Entertainment),
        "Shopping" => Some(Category::Shopping),
        "Health" => Some(Category::Health),
        "Education" => Some(Category::Education),
        "Subscriptions" => Some(Category::Subscriptions),
        other => {
            if other.starts_with("Other") {
                let inner = other
                    .strip_prefix("Other(")
                    .and_then(|s| s.strip_suffix(')'))
                    .unwrap_or("")
                    .to_string();
                Some(Category::Other(inner))
            } else {
                None
            }
        }
    }
}
