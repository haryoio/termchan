pub mod layout;
pub mod mylist;
pub mod popup;
pub mod stateful_list;
use std::{fmt::Display, vec};

use chrono::{DateTime, NaiveDateTime, Utc};
use rayon::prelude::*;
use termchan_core::get::message::Text;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use self::{
    layout::{single_area, split_area},
    mylist::{List, ListItem},
};
use crate::{
    application::App,
    config::Theme,
    state::{
        layout::Pane,
        post::ThreadPostStateItem,
        tab::{LeftTabItem, RightTabItem, TabsState},
        thread::ThreadStateItem,
    },
};

pub fn draw<'a, B: Backend>(f: &mut Frame<'_, B>, app: &mut App) {
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

    draw_status_line(f, app, chunks[1]);
}

fn draw_right_panel<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
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
    {
        // draw_tabs(f, &app.theme, &mut app.right_tabs, is_focused, tab_chunk).await;
    }
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::TOP)
        .style(block_style);

    f.render_widget(block, content_chunk);
    draw_thread(f, &mut app.clone(), content_chunk);
}

fn draw_left_panel<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
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

    {
        draw_tabs(f, &mut app.theme, &mut app.left_tabs, is_focused, tab_chunk);
    }

    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::TOP)
        .style(block_style);
    f.render_widget(block, content_chunk);
    {
        match app.left_tabs.get() {
            LeftTabItem::Home => draw_home(f, app, content_chunk),
            LeftTabItem::Bookmarks => draw_bookmarks(f, app, content_chunk),
            LeftTabItem::Bbsmenu => draw_bbsmenu(f, app, content_chunk),
            LeftTabItem::Categories => draw_categories(f, app, content_chunk),
            LeftTabItem::Category(_) => draw_category(f, app, content_chunk),
            LeftTabItem::Board(_) => draw_board(f, app, content_chunk),
            LeftTabItem::Settings => draw_settings(f, app, content_chunk),
        }
    }
}

fn draw_tabs<T: Display + Clone + Default, B: Backend>(
    f: &mut Frame<'_, B>,
    theme: &Theme,
    tab_state: &mut TabsState<T>,
    is_active: bool,
    area: Rect,
) {
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
    if tab_state.titles.len() == 0 {
        return f.render_widget(block, area);
    }

    let titles = tab_state
        .titles
        .iter()
        .cloned()
        .map(|t| Spans::from(format!("{}", t)))
        .collect();

    let tabs = Tabs::new(titles)
        .select(tab_state.index)
        .style(style)
        .block(block)
        .highlight_style(highlight_style)
        .divider(symbols::line::VERTICAL);
    f.render_widget(tabs, area);
}

fn draw_home<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" HOME ")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));

    let items = app
        .home
        .items
        .iter()
        .map(|item| {
            ListItem::new(format!("{}", item.item))
                .style(Style::default().fg(app.theme.text).bg(app.theme.reset))
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(app.theme.active_selected_text)
                .bg(app.theme.reset),
        )
        .highlight_symbol(&app.theme.active_item_symbol);

    f.render_stateful_widget(list, area, &mut app.home.state.clone());
}

fn draw_bookmarks<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" Bookmark ")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));

    let items = app
        .bookmark
        .items
        .iter()
        .map(|item| {
            ListItem::new(format!("{} {}", item.name, item.domain))
                .style(Style::default().fg(app.theme.text).bg(app.theme.reset))
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(app.theme.active_selected_text)
                .bg(app.theme.reset),
        )
        .highlight_symbol(&app.theme.active_item_symbol);

    f.render_stateful_widget(list, area, &mut app.bookmark.state.clone());
}

fn draw_bbsmenu<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" BBSmenu ")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));

    let items = app
        .bbsmenu
        .items
        .par_iter()
        .map(|item| {
            ListItem::new(item.url.clone())
                .style(Style::default().fg(app.theme.text).bg(app.theme.reset))
        })
        .collect::<Vec<_>>();

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

fn draw_categories<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" Categories ")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));

    let items = app
        .categories
        .items
        .par_iter()
        .map(|item| ListItem::new(item.name.clone()))
        .collect::<Vec<_>>();

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

fn draw_category<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let category_name = format!(
        " {} ",
        app.categories
            .items
            .get(app.categories.selected())
            .unwrap()
            .name
    );
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(category_name)
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));

    let items = app
        .category
        .items
        .par_iter()
        .map(|item| {
            let item = ListItem::new(item.name.clone())
                .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
            item
        })
        .collect::<Vec<_>>();

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

fn draw_board<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" Board ")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));

    let mut items = app.board.items.clone();
    let sort_type = app.get_sort_order();
    items.sort_by(|a, b| {
        match sort_type.clone() {
            crate::event::Sort::None(order) => {
                match order {
                    crate::event::Order::Asc => a.id.cmp(&b.id),
                    crate::event::Order::Desc => b.id.cmp(&a.id),
                }
            }
            crate::event::Sort::Ikioi(order) => {
                match order {
                    crate::event::Order::Asc => a.ikioi.partial_cmp(&b.ikioi).unwrap(),
                    crate::event::Order::Desc => b.ikioi.partial_cmp(&a.ikioi).unwrap(),
                }
            }
            crate::event::Sort::Latest(order) => {
                match order {
                    crate::event::Order::Asc => a.updated_at.partial_cmp(&b.updated_at).unwrap(),
                    crate::event::Order::Desc => b.updated_at.partial_cmp(&a.updated_at).unwrap(),
                }
            }
            crate::event::Sort::AlreadyRead(order) => {
                match order {
                    crate::event::Order::Asc => a.before_read.partial_cmp(&b.before_read).unwrap(),
                    crate::event::Order::Desc => b.before_read.partial_cmp(&a.before_read).unwrap(),
                }
            }
        }
    });

    let items = items
        .clone()
        .iter()
        .map(|thread| list_item_from_board(thread.clone(), area.width as usize))
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(app.theme.active_selected_text)
                .bg(app.theme.reset),
        )
        .highlight_symbol(&app.theme.active_item_symbol);

    f.render_stateful_widget(list, area, &mut app.board.state.clone());
}

fn draw_thread<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" Thread ")
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));

    let posts = app.thread.items.clone();

    let items = posts
        .par_iter()
        .map(|post| {
            let item = list_item_from_message(post.clone(), area.width as usize).clone();
            item
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(app.theme.active_selected_text)
                .bg(app.theme.reset),
        )
        .highlight_symbol(&app.theme.active_item_symbol);
    f.render_stateful_widget(list, area, &mut app.thread.state.clone());
}

fn draw_settings<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .border_type(app.theme.border_type())
        .borders(Borders::ALL)
        .title(" Settings ")
        .style(Style::default().fg(app.theme.text).bg(app.theme.reset));
    f.render_widget(block, area);
}

fn draw_status_line<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let mut lines = vec![];

    // スレ一覧のときのソートメニュー
    if matches!(app.left_tabs.get_current(), LeftTabItem::Board(_)) {
        //　ソートのオーダーをここに
        lines.push(Span::styled(
            " 並び替え: Ctrl+f",
            Style::default().bg(Color::White).fg(Color::Black),
        ));
        lines.push(Span::styled(
            format!(" {} ", app.get_sort_order()),
            Style::default().bg(Color::LightBlue),
        ));
    }

    lines.push(Span::styled(
        app.message.clone(),
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(app.theme.status_bar),
    ));

    let paragraph = Paragraph::new(Spans::from(lines));

    f.render_widget(paragraph, area);
}

fn list_item_from_board<'a>(thread: ThreadStateItem, width: usize) -> ListItem<'a> {
    // let frame_width = f.size().width as usize;
    let mut row_size = 0;
    let mut row = String::new();
    let mut texts = Vec::new();

    // スレッド作成時刻
    for (_, c) in thread.name.chars().enumerate() {
        row.push(c);
        row_size += c.len_utf8();
        if row_size > width {
            texts.push(Spans::from(format!("{}", row)));
            row = String::new();
            row_size = 0;
        }
    }

    texts.push(Spans::from(format!("{}", row)));
    let mut last_row = thread.updated_at.to_string();
    for _ in last_row.len()
        ..width
            - format!("{:.2} {:>4}", thread.ikioi, &thread.count.to_string())
                .as_str()
                .len()
            - 4
    {
        last_row.push(' ');
    }
    // TODO: 勢いによって色を変える
    last_row.push_str(format!("{:.2} {:>4}", thread.ikioi, &thread.count.to_string()).as_str());

    texts.push(Spans::from(Span::styled(
        last_row,
        Style::default().fg(Color::Gray),
    )));

    ListItem::new(texts)
}

fn list_item_from_message<'a>(thread: ThreadPostStateItem, width: usize) -> ListItem<'a> {
    let thread = thread.clone();

    // Spans Vector
    let mut texts = vec![];

    let mut header_spans = vec![Span::styled(
        format!("{} ", thread.index),
        Style::default().fg(Color::Blue),
    )];

    header_spans.push(Span::styled(thread.name, Style::default().fg(Color::White)));

    header_spans.push(Span::styled(
        thread.email.unwrap_or(" ".to_string()),
        Style::default().fg(Color::Gray),
    ));

    texts.push(Spans::from(header_spans));

    let naive = NaiveDateTime::from_timestamp(thread.date, 0);
    let date: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let date = date.format("%Y/%m/%d %H:%M:%S").to_string();
    texts.push(Spans::from(vec![
        Span::styled(format!("{}   ", date), Style::default().fg(Color::Gray)),
        Span::styled(thread.post_id.clone(), Style::default().fg(Color::Gray)),
    ]));

    let mut spans = vec![];
    for text in thread.message.text.iter() {
        use Text::*;
        match text {
            Plain(t) => {
                spans.push(Span::styled(
                    format!("{}", t),
                    Style::default().fg(Color::White),
                ))
            }
            Link(t) => {
                spans.push(Span::styled(
                    format!("{}", t),
                    Style::default().fg(Color::Cyan),
                ))
            }
            Image(t) => {
                spans.push(Span::styled(
                    format!("{}", t),
                    Style::default().fg(Color::Cyan),
                ))
            }
            AnchorRange(..) | Anchor(_) | Anchors(_) => {
                spans.push(Span::styled(
                    format!("{}", text),
                    Style::default().fg(Color::Cyan),
                ))
            }
            NewLine => {
                texts.push(Spans::from(spans.clone()));
                spans.clear();
            }
            Space => spans.push(Span::styled(" ", Style::default().fg(Color::White))),
            End => {
                texts.push(Spans::from(spans.clone()));
                spans.clear();
            }
        }
    }

    texts.push(Spans::from(spans.clone()));
    let mut hr = String::new();
    for _ in 0..width {
        hr.push('─');
    }

    texts.push(Spans::from(Span::styled(
        hr,
        Style::default().fg(Color::Gray),
    )));
    ListItem::new(texts).style(Style::default().fg(Color::White))
}
