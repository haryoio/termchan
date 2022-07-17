mod layout;

use std::{fmt::Display, sync::Arc};

use termchan::get::bbsmenu::{CategoryContent, CategoryItem};
use tokio::sync::Mutex;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::{self, DOT},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Widget},
    Frame,
};

use self::layout::{single_area, split_area};
use crate::{
    application::{App, Pane, TabsState},
    config::Theme,
    state::{LeftTabItem, RightTabItem},
};

pub fn draw<'a, B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'a>) {
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
        futures::executor::block_on(draw_left_panel(f, app, chunks[0]));
        futures::executor::block_on(draw_right_panel(f, app, chunks[1]));
    } else {
        let chunk = single_area(chunks[0]);
        futures::executor::block_on(draw_right_panel(f, app, chunk));
    }

    futures::executor::block_on(draw_status_line(f, app, chunks[1]))
}

async fn draw_right_panel<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
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
    draw_tabs(f, &app.theme, &mut app.right_tabs, is_focused, tab_chunk).await;
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::TOP)
        .style(block_style);
    f.render_widget(block, content_chunk);

    match app.right_tabs.get() {
        RightTabItem::Thread(_) => draw_thread(f, app, content_chunk).await,
    }
}

async fn draw_left_panel<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
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

    draw_tabs(f, &mut app.theme, &mut app.left_tabs, is_focused, tab_chunk).await;

    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::TOP)
        .style(block_style);
    f.render_widget(block, content_chunk);

    match app.left_tabs.get() {
        LeftTabItem::Bbsmenu => draw_bbsmenu(f, app, content_chunk).await,
        LeftTabItem::Categories => draw_categories(f, app, content_chunk).await,
        LeftTabItem::Category(_) => draw_category(f, app, content_chunk).await,
        LeftTabItem::Board(_) => draw_board(f, app, content_chunk).await,
        LeftTabItem::Threads => draw_threads(f, app, content_chunk).await,
        LeftTabItem::Settings => draw_settings(f, app, content_chunk).await,
    }
}

async fn draw_tabs<T: Display + Clone, B: Backend>(
    f: &mut Frame<'_, B>,
    theme: &Theme,
    tab_state: &mut TabsState<T>,
    is_active: bool,
    area: Rect,
) {
    let titles = tab_state
        .titles
        .iter()
        .cloned()
        .map(|t| Spans::from(format!("{}", t)))
        .collect();
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

async fn draw_thread<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" Thread ")
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
    f.render_widget(block, area);
}

async fn draw_bbsmenu<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" BBSmenu ")
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
    let items = app
        .bbsmenu
        .to_list_items(&|i| ListItem::new(i.clone()))
        .await;
    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(app.theme.active_selected_text)
                .bg(app.theme.reset),
        )
        .highlight_symbol(&app.theme.active_item_symbol);
    f.render_stateful_widget(list, area, &mut app.bbsmenu.state.clone());
}

async fn draw_categories<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" Categories ")
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
    let items = app
        .categories
        .to_list_items(&|i: &CategoryItem| {
            let item = ListItem::new(i.category_name.clone())
                .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
            item
        })
        .await;
    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(app.theme.active_selected_text)
                .bg(app.theme.reset),
        )
        .highlight_symbol(&app.theme.active_item_symbol);
    f.render_stateful_widget(list, area, &mut app.categories.state.clone());
}

async fn draw_category<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
    let category_name = format!(" {} ", app.categories.get().await.unwrap().category_name);
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(category_name)
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
    let items = app
        .category
        .to_list_items(&|i: &CategoryContent| {
            let item = ListItem::new(i.board_name.clone())
                .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
            item
        })
        .await;
    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(app.theme.active_selected_text)
                .bg(app.theme.reset),
        )
        .highlight_symbol(&app.theme.active_item_symbol);
    f.render_stateful_widget(list, area, &mut app.category.state.clone());
}

async fn draw_board<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" Board ")
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
    f.render_widget(block, area);
}

async fn draw_threads<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
    f.render_widget(block, area);
}

async fn draw_settings<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" Settings ")
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
    f.render_widget(block, area);
}

async fn draw_status_line<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
    let lower_chunk_block = Paragraph::new(Span::from("")).style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(app.theme.status_bar),
    );
    f.render_widget(lower_chunk_block, area);
}
