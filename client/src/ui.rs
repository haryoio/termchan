use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::{self, DOT},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Widget},
    Frame,
};

use crate::{
    application::{App, Pane, TabsState},
    config::Theme,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(f.size());

    let upper_chunk_block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" termchan ")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
    f.render_widget(upper_chunk_block, chunks[0]);

    if app.layout.visible_sidepane {
        let chunks = split_area(chunks[0]);

        draw_left_panel(f, app, chunks[0]);
        draw_right_panel(f, app, chunks[1]);
    } else {
        let chunk = single_area(chunks[0]);
        draw_right_panel(f, app, chunk);
    }

    draw_status_line(f, app, chunks[1])
}

fn draw_right_panel<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let is_focused = app.layout.focus_pane == Pane::Main;

    let block_style = if is_focused {
        Style::default().fg(app.theme.text).bg(app.theme.reset)
    } else {
        Style::default().fg(app.theme.inactive).bg(app.theme.reset)
    };

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(2)].as_ref())
        .horizontal_margin(1)
        .split(area);
    let top_block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::RIGHT | Borders::LEFT)
        .style(block_style);
    f.render_widget(top_block, area);

    let tab_chunk = layout[0];
    let content_chunk = layout[1];
    draw_tabs(f, &app.theme, &mut app.right_tabs, is_focused, tab_chunk);
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::TOP)
        .style(block_style);
    f.render_widget(block, content_chunk);

    let inner = match app.right_tabs.index {
        0 => Block::default().title("Inner 0"),
        1 => Block::default().title("Inner 1"),
        2 => Block::default().title("Inner 2"),
        3 => Block::default().title("Inner 3"),
        _ => unreachable!(),
    };
    f.render_widget(inner, content_chunk);
}

fn draw_left_panel<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let is_focused = app.layout.focus_pane == Pane::Side;

    let block_style = if is_focused {
        Style::default().fg(app.theme.text).bg(app.theme.reset)
    } else {
        Style::default().fg(app.theme.inactive).bg(app.theme.reset)
    };

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(2)].as_ref())
        .horizontal_margin(1)
        .split(area);
    let top_block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::LEFT)
        .style(block_style);

    f.render_widget(top_block, area);

    let tab_chunk = layout[0];
    let content_chunk = layout[1];

    draw_tabs(f, &mut app.theme, &mut app.left_tabs, is_focused, tab_chunk);

    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::TOP)
        .style(block_style);
    f.render_widget(block, content_chunk);

    let inner = match app.right_tabs.index {
        0 => Block::default(),
        1 => Block::default(),
        2 => Block::default(),
        3 => Block::default(),
        _ => unreachable!(),
    };
    f.render_widget(inner, content_chunk);
}

fn draw_tabs<B: Backend>(
    f: &mut Frame<B>,
    theme: &Theme,
    tab_state: &mut TabsState,
    is_active: bool,
    area: Rect,
) {
    let titles = tab_state.titles.iter().cloned().map(Spans::from).collect();
    let style = if is_active {
        Style::default().fg(theme.text).bg(theme.reset)
    } else {
        Style::default().fg(theme.inactive).bg(theme.reset)
    };
    let block = Block::default().style(style);

    let highlight_style = if is_active {
        Style::default()
            .fg(theme.active_selected_text)
            .bg(theme.reset)
    } else {
        Style::default()
            .fg(theme.inactive_selected_text)
            .bg(theme.reset)
    };

    let tabs = Tabs::new(titles)
        .select(tab_state.index)
        .style(style)
        .block(block)
        .highlight_style(highlight_style)
        .divider(symbols::line::VERTICAL);
    f.render_widget(tabs, area);
}

fn draw_status_line<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let lower_chunk_block = Paragraph::new(Span::from("")).style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(app.theme.status_bar),
    );
    f.render_widget(lower_chunk_block, area);
}

fn split_area(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .vertical_margin(1)
        .split(area);

    chunks
}

fn single_area(area: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .vertical_margin(1)
        .split(area);

    chunks[0]
}
