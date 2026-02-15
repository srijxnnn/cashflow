use chrono::{Datelike, Local};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Paragraph, Sparkline},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(area);

    render_summary_cards(f, app, chunks[0]);
    render_category_chart(f, app, chunks[1]);
    render_sparkline(f, app, chunks[2]);
}

fn render_summary_cards(f: &mut Frame, app: &App, area: Rect) {
    let now = Local::now();
    let month_total = app.total_for_month(now.year(), now.month());
    let year_total = app.total_for_year(now.year());
    let count = app.expenses.len();

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);

    let card_style = Style::default().fg(Color::White);

    let month_card = Paragraph::new(vec![
        Line::from(Span::styled(
            "This Month",
            Style::default().add_modifier(Modifier::DIM),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("${:.2}", month_total),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
    ])
    .style(card_style)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    let year_card = Paragraph::new(vec![
        Line::from(Span::styled(
            "This Year",
            Style::default().add_modifier(Modifier::DIM),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("${:.2}", year_total),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ])
    .style(card_style)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    let count_card = Paragraph::new(vec![
        Line::from(Span::styled(
            "Total Expenses",
            Style::default().add_modifier(Modifier::DIM),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("{}", count),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
    ])
    .style(card_style)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(month_card, cols[0]);
    f.render_widget(year_card, cols[1]);
    f.render_widget(count_card, cols[2]);
}

fn render_category_chart(f: &mut Frame, app: &App, area: Rect) {
    let now = Local::now();
    let data = app.spending_by_category(now.year(), now.month());

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

    let bars: Vec<Bar> = data
        .iter()
        .enumerate()
        .map(|(i, (cat, amount))| {
            let label = if cat.len() > 10 {
                format!("{}...", &cat[..8])
            } else {
                cat.clone()
            };
            Bar::default()
                .value(*amount as u64)
                .label(Line::from(label))
                .style(Style::default().fg(colors[i % colors.len()]))
                .value_style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
        })
        .collect();

    let chart = BarChart::default()
        .block(
            Block::default()
                .title(" Spending by Category (This Month) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .data(BarGroup::default().bars(&bars))
        .bar_width(
            if data.is_empty() {
                5
            } else {
                let available = area.width.saturating_sub(2);
                let per = available / data.len().max(1) as u16;
                per.max(3).min(12)
            },
        )
        .bar_gap(1);

    f.render_widget(chart, area);
}

fn render_sparkline(f: &mut Frame, app: &App, area: Rect) {
    let data = app.daily_spending_last_30_days();

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(" Daily Spending (Last 30 Days) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .data(&data)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(sparkline, area);
}
