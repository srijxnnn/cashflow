pub mod add_form;
pub mod dashboard;
pub mod expenses;
pub mod monthly;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
    Frame,
};

use crate::app::{App, InputMode, Tab};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(f.area());

    render_tabs(f, app, chunks[0]);
    render_content(f, app, chunks[1]);
    render_status_bar(f, app, chunks[2]);

    // Render overlays
    if app.input_mode == InputMode::AddForm || app.input_mode == InputMode::EditForm {
        add_form::render(f, app, f.area());
    }

    if app.input_mode == InputMode::HelpPopup {
        render_help_popup(f, f.area());
    }
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = Tab::titles()
        .iter()
        .map(|t| Line::from(Span::styled(*t, Style::default().fg(Color::White))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .title(" Cashflow ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .select(app.active_tab.index())
        .style(Style::default().fg(Color::DarkGray))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn render_content(f: &mut Frame, app: &App, area: Rect) {
    match app.active_tab {
        Tab::Dashboard => dashboard::render(f, app, area),
        Tab::Expenses => expenses::render(f, app, area),
        Tab::Monthly => monthly::render(f, app, area),
    }
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let text = if let Some(ref msg) = app.status_message {
        Line::from(Span::styled(msg.as_str(), Style::default().fg(Color::Green)))
    } else {
        Line::from(vec![
            Span::styled(
                " q:quit  ?:help  1-3:tabs  a:add  c:currency  x:export ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                format!(" [{}] ", app.currency.display_name()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    };

    let bar = Paragraph::new(text);
    f.render_widget(bar, area);
}

fn render_help_popup(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(50, 60, area);
    f.render_widget(Clear, popup_area);

    let help_text = vec![
        Line::from(Span::styled(
            "Global Keybindings",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  q, Ctrl+C    Quit"),
        Line::from("  1-3          Switch tabs"),
        Line::from("  Tab          Next tab"),
        Line::from("  Shift+Tab    Previous tab"),
        Line::from("  a            Add new expense"),
        Line::from("  c/C          Cycle currency forward/back"),
        Line::from("  x            Export to CSV"),
        Line::from("  ?            Toggle this help"),
        Line::from(""),
        Line::from(Span::styled(
            "Expenses Tab",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  j/↓          Move down"),
        Line::from("  k/↑          Move up"),
        Line::from("  /            Search"),
        Line::from("  e            Edit selected"),
        Line::from("  d            Delete selected"),
        Line::from("  r            Toggle recurring filter"),
        Line::from(""),
        Line::from(Span::styled(
            "Monthly Tab",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  ←/h          Previous month"),
        Line::from("  →/l          Next month"),
        Line::from(""),
        Line::from(Span::styled(
            "Form",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Tab          Next field"),
        Line::from("  Shift+Tab    Previous field"),
        Line::from("  ←/→          Cycle options"),
        Line::from("  Space        Toggle boolean"),
        Line::from("  Enter        Save"),
        Line::from("  Esc          Cancel"),
    ];

    let help = Paragraph::new(help_text).block(
        Block::default()
            .title(" Help ")
            .title_bottom(Line::from(" Press ? or Esc to close ").centered())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );

    f.render_widget(help, popup_area);
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
